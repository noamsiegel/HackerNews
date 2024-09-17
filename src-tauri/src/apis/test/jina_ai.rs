use tauri::

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://r.jina.ai/https://example.com").await?;
    println!("Status: {}", res.status());

    Ok(())
}
