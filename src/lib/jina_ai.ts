import fetch from "node-fetch";

export async function ScrapeURL(url: string): Promise<string> {
  const jinaUrl = `https://r.jina.ai/${url}`;
  const savedSettings = localStorage.getItem("settings");
  const token = savedSettings ? JSON.parse(savedSettings).jinaAiApiKey : "";

  console.log("Token:", token); // Debug log

  const headers = {
    Authorization: `Bearer ${token}`,
  };

  try {
    const response = await fetch(jinaUrl, {
      method: "GET",
      headers: headers,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.text();
  } catch (error) {
    console.error("Error fetching data:", error);
    throw error;
  }
}

// Example usage
async function main() {
  try {
    const data = await ScrapeURL("https://example.com");
    console.log("Fetched data:", data);
  } catch (error) {
    console.error("Error in main:", error);
  }
}

main();
