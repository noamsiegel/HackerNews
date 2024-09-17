use super::jina_ai::scrape_url;
use super::oa_client::create_client;
use crate::prompts::{PODCASTER_SYSTEM_PROMPT, SUMMARY_SYSTEM_PROMPT};
use crate::state::SelectedPrompt;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use tauri::command;

#[command]
pub async fn summarize_story(
    url: &str,
    state: tauri::State<'_, SelectedPrompt>,
) -> Result<String, String> {
    // Extract the value from the MutexGuard before entering async context
    let selected_prompt = state.0.lock().map_err(|e| e.to_string())?.clone();
    let system_prompt = match selected_prompt.as_str() {
        "SUMMARY_SYSTEM_PROMPT" => SUMMARY_SYSTEM_PROMPT,
        "PODCASTER_SYSTEM_PROMPT" => PODCASTER_SYSTEM_PROMPT,
        _ => SUMMARY_SYSTEM_PROMPT,
    };

    let content = scrape_url(url).await?;
    let client = create_client();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()
                    .map_err(|e| e.to_string())?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(format!("STORY:{}", content))
                    .build()
                    .map_err(|e| e.to_string())?,
            ),
        ])
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| e.to_string())?;

    let summary = response
        .choices
        .first()
        .and_then(|choice| choice.message.content.clone())
        .ok_or_else(|| "No summary generated".to_string())?;

    Ok(summary)
}
