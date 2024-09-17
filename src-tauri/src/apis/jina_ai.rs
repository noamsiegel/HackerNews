use tauri::command;
use tauri_plugin_http::reqwest::Client;

#[command]
pub async fn scrape_url(url: &str) -> Result<String, String> {
    let jina_url = format!("https://r.jina.ai/{}", url);
    let client = Client::new();
    
    match client.get(&jina_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(content) => Ok(content),
                    Err(e) => Err(format!("Failed to read response: {}", e))
                }
            } else {
                Err(format!("Request failed with status: {}", response.status()))
            }
        },
        Err(e) => Err(format!("Request failed: {}", e))
    }
}