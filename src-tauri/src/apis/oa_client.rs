use async_openai::{Client, config::OpenAIConfig};

pub fn create_client(api_key: &str) -> Client<OpenAIConfig> {
    Client::with_config(OpenAIConfig::new().with_api_key(api_key))
}