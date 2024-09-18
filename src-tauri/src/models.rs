use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref LLM_MODELS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("chatgpt-4o-latest", "GPT-4o latest");
        m.insert("chatgpt-4o-min", "GPT-40 mini");
        m
    };
}

#[tauri::command]
pub fn get_llm_models() -> HashMap<&'static str, &'static str> {
    LLM_MODELS.clone()
}
