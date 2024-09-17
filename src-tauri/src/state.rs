use std::sync::Mutex;

pub struct SelectedPrompt(pub Mutex<String>);
pub struct OpenAIKey(pub Mutex<String>);
pub struct JinaAIKey(pub Mutex<String>);
