use crate::config::Config;
use serde_json::Value;
use std::process::Stdio;

use super::{ChatMessage, ChatResponse, ToolCall, ToolDef};

pub struct OpencodeProvider {
    model_override: String,
}

impl OpencodeProvider {
    pub fn new(cfg: &Config) -> Self {
        Self {
            model_override: if cfg.model.is_empty() {
                String::new()
            } else {
                cfg.model.clone()
            },
        }
    }

    pub async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDef],
    ) -> Result<ChatResponse, Box<dyn std::error::Error>> {
        let (system_parts, conversation): (Vec<&ChatMessage>, Vec<&ChatMessage>) =
            messages.iter().partition(|m| m.role == "system");

        let mut system_text = system_parts
            .iter()
            .map(|m| m.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");

        if !tools.is_empty() {
            let tool_section = format_tool_definitions(tools);
            system_text.push_str(&tool_section);
        }

        let conversation_text = conversation
            .iter()
            .map(|m| {
                let role = match m.role.as_str() {
                    "user" => "User",
                    "assistant" => "Assistant",
                    "tool" => "Tool result",
                    _ => &m.role,
                };
                format!("[{role}]:\n{}\n", m.content)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let full_prompt = if system_text.is_empty() {
            conversation_text
        } else {
            format!("<system>\n{system_text}\n</system>\n\n{conversation_text}")
        };

        let mut cmd = std::process::Command::new("opencode");
        cmd.arg("run")
            .arg(&full_prompt)
            .arg("--format")
            .arg("json")
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        if !self.model_override.is_empty() {
            cmd.arg("--model").arg(&self.model_override);
        }

        let output = tokio::task::spawn_blocking(move || cmd.output()).await??;

        if !output.status.success() {
            return Err(format!("opencode exited with status: {}", output.status).into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let (text, tool_calls) = parse_opencode_output(&stdout);

        Ok(ChatResponse {
            message: ChatMessage {
                role: "assistant".into(),
                content: text,
            },
            tool_calls,
        })
    }
}

fn format_tool_definitions(tools: &[ToolDef]) -> String {
    let mut s = String::from("\n\n## Available Tools\n\nYou have access to these tools. To call a tool, output a single JSON object on its own line:\n\n```json\n{\"tool\": \"<name>\", \"args\": {<params>}}\n```\n\nThe tool result will be shown after your tool call. You may call multiple tools across multiple turns.\n\n");
    for t in tools {
        s.push_str(&format!(
            "### {}\n{}\nParameters: {}\n\n",
            t.name, t.description, t.parameters
        ));
    }
    s
}

fn parse_opencode_output(stdout: &str) -> (String, Option<Vec<ToolCall>>) {
    let mut final_text = String::new();

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let event: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if let Some("text") = event["type"].as_str() {
            if let Some(text) = event["part"]["text"].as_str() {
                final_text.push_str(text);
            }
        }
    }

    let tool_calls = extract_tool_calls(&final_text);

    if tool_calls.is_some() {
        (final_text, tool_calls)
    } else {
        (final_text, None)
    }
}

fn extract_tool_calls(text: &str) -> Option<Vec<ToolCall>> {
    let mut tool_calls: Vec<ToolCall> = Vec::new();

    let in_code_block = |line: &str| line.trim().starts_with("```");
    let mut inside_fence = false;

    for line in text.lines() {
        let trimmed = line.trim();
        if in_code_block(trimmed) {
            inside_fence = !inside_fence;
            continue;
        }
        if inside_fence {
            continue;
        }

        if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
            if v.get("tool").and_then(|t| t.as_str()).is_some()
                && v.get("args").is_some()
            {
                tool_calls.push(ToolCall {
                    id: format!("call_{}", tool_calls.len()),
                    name: v["tool"].as_str().unwrap_or("").to_string(),
                    arguments: v["args"].to_string(),
                });
            }
        }

        if let Some(start) = trimmed.find(r#"{"tool""#) {
            let slice = &trimmed[start..];
            if let Some(end) = slice.find('}') {
                let candidate = &slice[..=end];
                if let Ok(v) = serde_json::from_str::<Value>(candidate) {
                    if v.get("tool").and_then(|t| t.as_str()).is_some()
                        && v.get("args").is_some()
                    {
                        tool_calls.push(ToolCall {
                            id: format!("call_{}", tool_calls.len()),
                            name: v["tool"].as_str().unwrap_or("").to_string(),
                            arguments: v["args"].to_string(),
                        });
                    }
                }
            }
        }
    }

    if tool_calls.is_empty() {
        None
    } else {
        Some(tool_calls)
    }
}
