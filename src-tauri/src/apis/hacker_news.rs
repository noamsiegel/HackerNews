use tauri::command;
use tauri_plugin_http::reqwest;


#[command]
pub async fn fetch_top_stories() -> Result<Vec<u64>, String> {
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let story_ids: Vec<u64> = response.json().await.map_err(|e| e.to_string())?;
    Ok(story_ids)
}

#[command]
pub async fn fetch_story(id: u64) -> Result<serde_json::Value, String> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let story: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(story)
}