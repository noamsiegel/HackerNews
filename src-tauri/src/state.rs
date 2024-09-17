use std::sync::Mutex;

pub struct SelectedPrompt(pub Mutex<String>);
