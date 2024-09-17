// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod apis {
    pub mod hacker_news;
    pub mod jina_ai;
    pub mod oa_client;
    pub mod summaries;
}

mod prompts;
mod state;
use crate::apis::hacker_news::{fetch_story, fetch_top_stories};
use crate::apis::jina_ai::scrape_url;
use crate::apis::summaries::summarize_story;
use crate::state::{
    get_jina_api_key, get_openai_api_key, load_api_key, update_jina_api_key, update_openai_api_key,
};
use crate::state::{JinaAIKey, OpenAIKey, SelectedPrompt};
use std::fs;
use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // Initialize states
            let selected_prompt = load_selected_prompt(app.handle().clone())
                .unwrap_or_else(|_| "SUMMARY_SYSTEM_PROMPT".to_string());
            app.manage(SelectedPrompt(Mutex::new(selected_prompt)));
            let openai_key =
                load_api_key(app.handle().clone(), "openAIKey").unwrap_or_else(|_| String::new());
            app.manage(OpenAIKey(Mutex::new(openai_key)));

            let jina_key =
                load_api_key(app.handle().clone(), "jinaAIKey").unwrap_or_else(|_| String::new());
            app.manage(JinaAIKey(Mutex::new(jina_key)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_story,
            fetch_top_stories,
            scrape_url,
            summarize_story,
            update_selected_prompt,
            get_selected_prompt,
            update_openai_api_key,
            get_openai_api_key,
            update_jina_api_key,
            get_jina_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn load_selected_prompt(
    app_handle: tauri::AppHandle,
) -> Result<String, Box<dyn std::error::Error>> {
    let app_dir = app_handle.path().app_data_dir().unwrap();
    let settings_file = app_dir.join("settings.json");

    if settings_file.exists() {
        let contents = fs::read_to_string(settings_file)?;
        let settings: serde_json::Value = serde_json::from_str(&contents)?;
        Ok(settings["selectedPrompt"]
            .as_str()
            .unwrap_or("SUMMARY_SYSTEM_PROMPT")
            .to_string())
    } else {
        Ok("SUMMARY_SYSTEM_PROMPT".to_string())
    }
}

#[tauri::command]
fn update_selected_prompt(
    prompt: String,
    state: tauri::State<'_, SelectedPrompt>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut selected_prompt = state.0.lock().unwrap();
    *selected_prompt = prompt.clone();

    let app_dir = app.path().app_data_dir().unwrap();
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let settings_file = app_dir.join("settings.json");
    let settings = serde_json::json!({
        "selectedPrompt": prompt
    });
    fs::write(
        settings_file,
        serde_json::to_string_pretty(&settings).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_selected_prompt(state: tauri::State<'_, SelectedPrompt>) -> String {
    let selected_prompt = state.0.lock().unwrap();
    selected_prompt.clone()
}

// #[tauri::command]
// fn update_openai_api_key(
//     key: String,
//     state: tauri::State<'_, OpenAIKey>,
//     app: tauri::AppHandle,
// ) -> Result<(), String> {
//     let mut openai_key = state.0.lock().unwrap();
//     *openai_key = key.clone();

//     update_settings(&app, "openAIKey", &key)
// }

// #[tauri::command]
// fn get_openai_api_key(state: tauri::State<'_, OpenAIKey>) -> String {
//     let openai_key = state.0.lock().unwrap();
//     openai_key.clone()
// }

// #[tauri::command]
// fn update_jina_api_key(
//     key: String,
//     state: tauri::State<'_, JinaAIKey>,
//     app: tauri::AppHandle,
// ) -> Result<(), String> {
//     let mut jina_key = state.0.lock().unwrap();
//     *jina_key = key.clone();

//     update_settings(&app, "jinaAIKey", &key)
// }

// #[tauri::command]
// fn get_jina_api_key(state: tauri::State<'_, JinaAIKey>) -> String {
//     let jina_key = state.0.lock().unwrap();
//     jina_key.clone()
// }

// fn update_settings(app: &tauri::AppHandle, key: &str, value: &str) -> Result<(), String> {
//     let app_dir = app.path().app_data_dir().unwrap();
//     fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
//     let settings_file = app_dir.join("settings.json");

//     let mut settings = if settings_file.exists() {
//         let contents = fs::read_to_string(&settings_file).map_err(|e| e.to_string())?;
//         serde_json::from_str(&contents).unwrap_or_else(|_| serde_json::json!({}))
//     } else {
//         serde_json::json!({})
//     };

//     settings[key] = serde_json::Value::String(value.to_string());

//     fs::write(
//         settings_file,
//         serde_json::to_string_pretty(&settings).unwrap(),
//     )
//     .map_err(|e| e.to_string())?;
//     Ok(())
// }
