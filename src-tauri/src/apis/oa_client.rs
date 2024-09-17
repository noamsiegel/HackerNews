use std::env;
use async_openai::{Client, config::OpenAIConfig};
use dotenv::dotenv;


pub fn create_client() -> Client<OpenAIConfig> {
    // Create a OpenAI client with api key from env var OPENAI_API_KEY and default base url.

    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env file");

        // Create a custom configuration with the API key
        let config = OpenAIConfig::new().with_api_key(api_key);
    
        // Create and return the client with the custom configuration
        Client::with_config(config)
}