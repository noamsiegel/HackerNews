use lazy_static::lazy_static;
use std::sync::Mutex;
// use tauri::command;

lazy_static! {
    static ref OPENAI_API_KEY: Mutex<String> = Mutex::new(String::new());
    static ref JINAAI_API_KEY: Mutex<String> = Mutex::new(String::new());
}

// #[command]
// fn update_openai_api_key(key: String) {
//     std::env::set_var("OPENAI_API_KEY", key);
// }

// #[command]
// fn update_jinaai_api_key(key: String) {
//     std::env::set_var("JINAAI_API_KEY", key);
// }