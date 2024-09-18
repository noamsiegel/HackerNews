use std::{fs, sync::Mutex};
use tauri::Manager;

pub struct SelectedPrompt(pub Mutex<String>);
pub struct OpenAIKey(pub Mutex<String>);
pub struct JinaAIKey(pub Mutex<String>);
pub struct SelectedModel(pub Mutex<String>);

#[tauri::command]
pub fn update_openai_api_key(
    key: String,
    state: tauri::State<'_, OpenAIKey>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut openai_key = state.0.lock().unwrap();
    *openai_key = key.clone();

    update_settings(&app, "openAIKey", &key)
}

#[tauri::command]
pub fn get_openai_api_key(state: tauri::State<'_, OpenAIKey>) -> String {
    let openai_key = state.0.lock().unwrap();
    openai_key.clone()
}

pub fn get_api_key(
    app_handle: tauri::AppHandle,
    key_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let app_dir = app_handle.path().app_data_dir().unwrap();
    let settings_file = app_dir.join("settings.json");

    if settings_file.exists() {
        let contents = fs::read_to_string(settings_file)?;
        let settings: serde_json::Value = serde_json::from_str(&contents)?;
        Ok(settings[key_name].as_str().unwrap_or("").to_string())
    } else {
        Ok(String::new())
    }
}

#[tauri::command]
pub fn update_jina_api_key(
    key: String,
    state: tauri::State<'_, JinaAIKey>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut jina_key = state.0.lock().unwrap();
    *jina_key = key.clone();

    update_settings(&app, "jinaAIKey", &key)
}

#[tauri::command]
pub fn get_jina_api_key(state: tauri::State<'_, JinaAIKey>) -> String {
    let jina_key = state.0.lock().unwrap();
    jina_key.clone()
}

pub fn update_settings(app: &tauri::AppHandle, key: &str, value: &str) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().unwrap();
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let settings_file = app_dir.join("settings.json");

    let mut settings = if settings_file.exists() {
        let contents = fs::read_to_string(&settings_file).map_err(|e| e.to_string())?;
        serde_json::from_str(&contents).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    settings[key] = serde_json::Value::String(value.to_string());

    fs::write(
        settings_file,
        serde_json::to_string_pretty(&settings).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_selected_model(
    model: String,
    state: tauri::State<'_, SelectedModel>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut selected_model = state.0.lock().unwrap();
    *selected_model = model.clone();

    update_settings(&app, "selectedModel", &model)
}

// #[tauri::command]
// pub fn get_selected_model(state: tauri::State<'_, SelectedModel>) -> String {
//     let selected_model = state.0.lock().unwrap();
//     selected_model.clone()
// }

#[tauri::command]
pub fn get_selected_model(app_handle: tauri::AppHandle) -> Result<String, String> {
    get_api_key(app_handle, "selectedModel").map_err(|e| e.to_string())
}
