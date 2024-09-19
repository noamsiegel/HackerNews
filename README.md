## Hacker News App Documentation

This application fetches the top stories from Hacker News and provides summaries of them.

### Inputs

- **URL**: A URL to an article from Hacker News. If no URL is given, it will fetch the top 10 stories from the Hacker News API.

### Outputs

- **Story Details**: The application will display the title and score of the story, along with a button to go to the full story on Hacker News. If a URL is provided, a button to fetch and display the content of the URL will appear.
- **Scraped Content**:  The application will display the full content of the URL provided.
- **Summary**: The application will provide a summary of the content. The summary will be generated using the OpenAI API. The user can choose between different summaries based on various system prompts.

### Usage

1. **Open the application**: The application will show a list of the top 10 stories from Hacker News.
2. **Click on a story**: This will navigate to a page with the full details of the story, including a link to the article on Hacker News.
3. **Scrape the content**: If a story has a URL, click on the "Scrape Content" button to fetch and display the full text content of the URL.
4. **Summarize the content**: Click on the "Summarize" button to generate a summary of the article. The summary is generated using the OpenAI API.

### Settings

The application allows the user to adjust the following settings:

- **OpenAI API Key**: This is required to generate summaries using the OpenAI API. You can obtain an OpenAI API Key from [https://beta.openai.com/account/api-keys](https://beta.openai.com/account/api-keys).
- **Jina AI API Key**: This is required to scrape and fetch content using Jina AI. You can obtain a Jina AI API Key from [https://jina.ai/](https://jina.ai/).
- **Selected Prompt**: This option allows you to choose the type of summary you want to generate. Currently, the available prompts are "Summary System Prompt" and "Podcaster System Prompt".
- **Selected Model**: This option allows you to choose the LLM model used to generate summaries. Currently, the available models are "gpt-4o-latest" and "gpt-4o-min".
