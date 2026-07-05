use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

pub enum LlmProvider {
    OpenAi(openai::OpenAiProvider),
    Anthropic(anthropic::AnthropicProvider),
    Opencode(opencode::OpencodeProvider),
}

impl LlmProvider {
    pub async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: &[ToolDef],
    ) -> Result<ChatResponse, Box<dyn std::error::Error>> {
        match self {
            LlmProvider::OpenAi(p) => p.chat(messages, tools).await,
            LlmProvider::Anthropic(p) => p.chat(messages, tools).await,
            LlmProvider::Opencode(p) => p.chat(messages, tools).await,
        }
    }
}

pub fn create(cfg: &Config) -> LlmProvider {
    match cfg.provider.as_str() {
        "anthropic" => LlmProvider::Anthropic(anthropic::AnthropicProvider::new(cfg)),
        "opencode" => LlmProvider::Opencode(opencode::OpencodeProvider::new(cfg)),
        "deepseek" => LlmProvider::OpenAi(openai::OpenAiProvider::with_base_url(
            cfg,
            "https://api.deepseek.com/v1",
        )),
        "openrouter" => LlmProvider::OpenAi(openai::OpenAiProvider::with_base_url(
            cfg,
            &cfg.base_url
                .clone()
                .unwrap_or_else(|| "https://openrouter.ai/api/v1".into()),
        )),
        _ => LlmProvider::OpenAi(openai::OpenAiProvider::new(cfg)),
    }
}

pub mod anthropic;
pub mod openai;
pub mod opencode;
