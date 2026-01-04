use std::option::Option;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use anyhow::Context;
use super::react_template;

#[derive(Debug)]
pub struct DeepSeekClient {
    client: Client,
    api_key: String,
    base_url: String,
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    /// 创建用户消息
    pub fn user(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: react_template::complete_usr_template(content),
        }
    }

    pub fn observation(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: react_template::complete_observation_template(content),
        }
    }

    pub fn system(tool_info: &str) -> Self {
        Self {
            role: "system".to_string(),
            content: react_template::get_react_prompt().replace("${tool_list}", tool_info),
        }
    }

    pub fn assistant(content: &str) -> Self{
        Self{
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
}

/// 聊天请求
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// 聊天响应
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
    usage: Option<Usage>,
}

/// 聊天选择
#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
    index: u32,
    finish_reason: String,
}

/// Token 使用情况
#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl DeepSeekClient {

    pub fn new(api_key: impl Into<String>) -> Self {
        let api_key = api_key.into();

        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .connect_timeout(Duration::from_secs(100))
            .build()
            .unwrap();

        Self {
            client,
            api_key,
            base_url: "https://api.deepseek.com".to_string(),
        }
    }

    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        model: Option<String>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> anyhow::Result<String> {
        let request = ChatRequest {
            model: model.unwrap_or("deepseek-chat".to_string()),
            messages,
            temperature,
            max_tokens,
            stream: Some(false),
        };

        let url = format!("{}/v1/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to send request to DeepSeek API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".to_string());

            anyhow::bail!("DeepSeek API error ({}): {}", status, error_text);
        }

        let api_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse DeepSeek API response")?;

        let choice = api_response
            .choices
            .first()
            .context("No choices in API response")?;

        Ok(choice.message.content.clone())
    }

    pub async fn list_models(&self) -> anyhow::Result<Vec<String>> {
        let url = format!("{}/v1/models", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to fetch models")?;

        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Failed to fetch models: HTTP {}", status);
        }

        #[derive(Debug, Deserialize)]
        struct ModelsResponse {
            data: Vec<ModelData>,
        }

        #[derive(Debug, Deserialize)]
        struct ModelData {
            id: String,
        }

        let models_response: ModelsResponse = response
            .json()
            .await
            .context("Failed to parse models response")?;

        Ok(models_response.data.into_iter().map(|m| m.id).collect())
    }
}