use std::collections::HashMap;
use std::path::Path;

use crate::config::Config;
use crate::output::{self, WikiMeta};
use crate::prompts;
use crate::provider::{ChatMessage, LlmProvider, ToolCall, ToolDef};
use crate::scanner;

fn tool_definitions() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "list_files".into(),
            description: "List files and directories at a given path. Pass an empty string to list the root directory.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative path to list. Empty string for root."
                    }
                },
                "required": ["path"]
            }),
        },
        ToolDef {
            name: "read_file".into(),
            description: "Read the full contents of a file.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative path to the file to read."
                    }
                },
                "required": ["path"]
            }),
        },
        ToolDef {
            name: "search".into(),
            description: "Search the codebase for a pattern. Returns matching lines with file paths and line numbers. Use this to find definitions, usages, or patterns.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "Text or substring to search for."
                    }
                },
                "required": ["pattern"]
            }),
        },
        ToolDef {
            name: "write_doc".into(),
            description: "Write a documentation file into the codewiki/ output directory. Use this to create or update documentation. Do NOT write outside codewiki/.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative path within codewiki/ to write to (e.g. 'index.md', 'architecture.md')."
                    },
                    "content": {
                        "type": "string",
                        "description": "Markdown content for the documentation file."
                    }
                },
                "required": ["path", "content"]
            }),
        },
    ]
}

fn execute_tool(
    tool: &ToolCall,
    project_dir: &Path,
    codewiki_dir: &Path,
    _wiki_meta: &WikiMeta,
) -> String {
    match tool.name.as_str() {
        "list_files" => {
            let args: HashMap<String, String> =
                serde_json::from_str(&tool.arguments).unwrap_or_default();
            let subpath = args.get("path").map(|s| s.as_str()).unwrap_or("");
            let dir_to_list = if subpath.is_empty() {
                project_dir.to_path_buf()
            } else {
                project_dir.join(subpath.trim_start_matches('/'))
            };

            if !dir_to_list.exists() {
                return format!("Error: path does not exist: {:?}", dir_to_list);
            }

            match scanner::scan_project(&dir_to_list) {
                Ok(entries) => {
                    if entries.is_empty() {
                        format!("Directory is empty: {:?}", dir_to_list)
                    } else {
                        let mut lines: Vec<String> = Vec::new();
                        for e in &entries {
                            let kb = e.size / 1024;
                            lines.push(format!(
                                "{} ({} KB)",
                                e.relative_path,
                                if kb == 0 { 1 } else { kb }
                            ));
                        }
                        lines.join("\n")
                    }
                }
                Err(e) => format!("Error listing directory: {e}"),
            }
        }
        "read_file" => {
            let args: HashMap<String, String> =
                serde_json::from_str(&tool.arguments).unwrap_or_default();
            let filepath = args.get("path").map(|s| s.as_str()).unwrap_or("");

            if filepath.is_empty() {
                return "Error: no path provided".into();
            }

            let full_path = project_dir.join(filepath.trim_start_matches('/'));
            match scanner::read_file(&full_path) {
                Ok(content) => {
                    let line_count = content.lines().count();
                    if content.len() > 100_000 {
                        format!(
                            "File too large ({} lines, {} bytes). Here are the first 500 lines:\n\n{}",
                            line_count,
                            content.len(),
                            content.lines().take(500).collect::<Vec<_>>().join("\n")
                        )
                    } else {
                        format!(
                            "File: {filepath} ({} lines, {} bytes)\n\n{content}",
                            line_count,
                            content.len()
                        )
                    }
                }
                Err(e) => format!("Error reading file: {e}"),
            }
        }
        "search" => {
            let args: HashMap<String, String> =
                serde_json::from_str(&tool.arguments).unwrap_or_default();
            let pattern = args.get("pattern").map(|s| s.as_str()).unwrap_or("");

            if pattern.is_empty() {
                return "Error: no pattern provided".into();
            }

            match scanner::search_codebase(project_dir, pattern) {
                Ok(results) => {
                    if results.is_empty() {
                        format!("No matches found for '{pattern}'")
                    } else {
                        let mut output = String::new();
                        for (file, line, text) in &results {
                            output.push_str(&format!("{file}:{line}: {text}\n"));
                        }
                        output
                    }
                }
                Err(e) => format!("Error searching: {e}"),
            }
        }
        "write_doc" => {
            let args: HashMap<String, String> =
                serde_json::from_str(&tool.arguments).unwrap_or_default();
            let path = args.get("path").map(|s| s.as_str()).unwrap_or("");
            let content = args.get("content").map(|s| s.as_str()).unwrap_or("");

            if path.is_empty() {
                return "Error: no path provided".into();
            }

            let _full_path = output::write_doc(codewiki_dir, path, content);
            format!(
                "Documentation written: codewiki/{} ({} bytes)",
                path,
                content.len()
            )
        }
        _ => format!("Unknown tool: {}", tool.name),
    }
}

pub async fn run_interactive(
    project_dir: &Path,
    provider: &LlmProvider,
    _cfg: &Config,
    initial_prompt: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let codewiki_dir = project_dir.join("codewiki");
    std::fs::create_dir_all(&codewiki_dir)?;

    let wiki_meta = output::load_wiki_meta(&codewiki_dir);
    let tools = tool_definitions();

    let mut messages: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".into(),
            content: prompts::SYSTEM_PROMPT.into(),
        },
        ChatMessage {
            role: "user".into(),
            content: prompts::initial_prompt(
                initial_prompt
                    .unwrap_or("Please generate comprehensive documentation for this codebase."),
            ),
        },
    ];

    let mut total_tool_calls = 0;
    let max_tool_calls = 100;

    loop {
        let resp = provider.chat(&messages, &tools).await?;

        if let Some(tool_calls) = resp.tool_calls {
            if tool_calls.is_empty() {
                if !resp.message.content.is_empty() {
                    println!("\n{}", resp.message.content);
                }
                break;
            }

            messages.push(ChatMessage {
                role: "assistant".into(),
                content: if resp.message.content.is_empty() {
                    String::new()
                } else {
                    resp.message.content.clone()
                },
            });

            for tc in &tool_calls {
                println!("  -> {}", tc.name);

                let result = execute_tool(tc, project_dir, &codewiki_dir, &wiki_meta);

                messages.push(ChatMessage {
                    role: "tool".into(),
                    content: result,
                });

                total_tool_calls += 1;

                if total_tool_calls >= max_tool_calls {
                    println!(
                        "\nReached maximum tool calls ({}). Stopping.",
                        max_tool_calls
                    );
                    break;
                }
            }

            messages.push(ChatMessage {
                role: "user".into(),
                content: prompts::tool_response_prompt().into(),
            });
        } else {
            println!("\n{}", resp.message.content);

            if resp.message.content.to_uppercase().contains("DONE") {
                break;
            }

            let read_line = || -> Option<String> {
                use std::io::Write;
                print!("\n> ");
                let _ = std::io::stdout().flush();
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(0) => None,
                    Ok(_) => Some(input.trim().to_string()),
                    Err(_) => None,
                }
            };

            while let Some(user_input) = read_line() {
                if user_input.is_empty() {
                    continue;
                }
                if user_input.eq_ignore_ascii_case("quit")
                    || user_input.eq_ignore_ascii_case("exit")
                    || user_input.eq_ignore_ascii_case("done")
                {
                    return Ok(());
                }
                messages.push(ChatMessage {
                    role: "user".into(),
                    content: user_input,
                });
                break;
            }
        }
    }

    output::save_wiki_meta(
        &codewiki_dir,
        &WikiMeta {
            file_hashes: HashMap::new(),
        },
    );

    let _ = output::append_agents_reference(project_dir);

    println!("\nDocumentation written to codewiki/");
    Ok(())
}

pub async fn run_oneshot(
    project_dir: &Path,
    provider: &LlmProvider,
    _cfg: &Config,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let codewiki_dir = project_dir.join("codewiki");
    std::fs::create_dir_all(&codewiki_dir)?;

    let wiki_meta = output::load_wiki_meta(&codewiki_dir);
    let tools = tool_definitions();

    let mut messages: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".into(),
            content: prompts::SYSTEM_PROMPT.into(),
        },
        ChatMessage {
            role: "user".into(),
            content: prompts::initial_prompt(prompt),
        },
    ];

    let mut total_tool_calls = 0;
    let max_tool_calls = 100;
    let mut final_output = String::new();

    loop {
        let resp = provider.chat(&messages, &tools).await?;

        if let Some(tool_calls) = resp.tool_calls {
            if tool_calls.is_empty() {
                final_output.push_str(&resp.message.content);
                break;
            }

            messages.push(ChatMessage {
                role: "assistant".into(),
                content: resp.message.content,
            });

            for tc in &tool_calls {
                let result = execute_tool(tc, project_dir, &codewiki_dir, &wiki_meta);
                messages.push(ChatMessage {
                    role: "tool".into(),
                    content: result,
                });
                total_tool_calls += 1;
                if total_tool_calls >= max_tool_calls {
                    break;
                }
            }

            messages.push(ChatMessage {
                role: "user".into(),
                content: prompts::tool_response_prompt().into(),
            });
        } else {
            final_output.push_str(&resp.message.content);
            break;
        }
    }

    Ok(final_output)
}

pub async fn update_docs(
    project_dir: &Path,
    codewiki_dir: &Path,
    wiki_meta: &WikiMeta,
    provider: &LlmProvider,
    _cfg: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let tools = tool_definitions();

    let mut messages: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".into(),
            content: prompts::SYSTEM_PROMPT.into(),
        },
        ChatMessage {
            role: "user".into(),
            content: "The codewiki/ directory already has existing documentation. Please review the codebase for any changes since the docs were last generated, and update the documentation files with write_doc to reflect the current state of the codebase. Focus on what changed.".into(),
        },
    ];

    let mut total_tool_calls = 0;
    let max_tool_calls = 100;

    loop {
        let resp = provider.chat(&messages, &tools).await?;

        if let Some(tool_calls) = resp.tool_calls {
            if tool_calls.is_empty() {
                if !resp.message.content.is_empty() {
                    println!("\n{}", resp.message.content);
                }
                break;
            }

            messages.push(ChatMessage {
                role: "assistant".into(),
                content: resp.message.content,
            });

            for tc in &tool_calls {
                let result = execute_tool(tc, project_dir, codewiki_dir, wiki_meta);
                messages.push(ChatMessage {
                    role: "tool".into(),
                    content: result,
                });
                total_tool_calls += 1;
                if total_tool_calls >= max_tool_calls {
                    break;
                }
            }

            messages.push(ChatMessage {
                role: "user".into(),
                content: prompts::tool_response_prompt().into(),
            });
        } else {
            println!("\n{}", resp.message.content);
            break;
        }
    }

    Ok(())
}
