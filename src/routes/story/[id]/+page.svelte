<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import { marked } from "marked";

  let story: any = null;
  let loading = true;
  let content: string | null = null;
  let contentLoading = false;
  let summary: string | null = null;
  let summaryLoading = false;

  onMount(async () => {
    const id = $page.params.id;
    try {
      story = await invoke("fetch_story", { id: parseInt(id) });
    } catch (error) {
      console.error("Failed to fetch story:", error);
    } finally {
      loading = false;
    }
  });

  function goBack() {
    history.back();
  }

  async function showContent() {
    if (!story.url) return;

    contentLoading = true;
    try {
      content = await invoke("scrape_url", { url: story.url });
    } catch (error) {
      console.error("Failed to fetch content:", error);
      content = "Failed to load content.";
    } finally {
      contentLoading = false;
    }
  }

  async function summarizeContent() {
    if (!story.url) return;

    summaryLoading = true;
    try {
      summary = await invoke("summarize_story", { url: story.url });
    } catch (error) {
      console.error("Failed to summarize content:", error);
      summary = "Failed to generate summary.";
    } finally {
      summaryLoading = false;
    }
  }
</script>

<div class="story-details">
  <div class="navbar">
    <button on:click={goBack}>Back to Home</button>
    <button class="settings-icon" on:click={() => goto("/settings")}>⚙️</button>
  </div>

  {#if loading}
    <p>Loading...</p>
  {:else if story}
    <h1>{story.title}</h1>
    <p>By: {story.by}</p>
    <p>Score: {story.score}</p>
    {#if story.url}
      <p>
        <a href={story.url} target="_blank" rel="noopener noreferrer"
          >Read more</a
        >
      </p>
      <button on:click={showContent} disabled={contentLoading}>
        {contentLoading ? "Loading..." : "Show Content"}
      </button>
      <button on:click={summarizeContent} disabled={summaryLoading}>
        {summaryLoading ? "Summarizing..." : "Summarize"}
      </button>
    {/if}
    {#if story.text}
      <p>{@html story.text}</p>
    {/if}

    <p>Comments: {story.descendants}</p>

    {#if content}
      <div class="content-block">
        <h2>Article Content:</h2>
        <div class="markdown-content">{@html marked(content)}</div>
      </div>
    {/if}

    {#if summary}
      <div class="content-block">
        <h2>Summary:</h2>
        <div class="markdown-content">{@html marked(summary)}</div>
      </div>
    {/if}
  {:else}
    <p>Failed to load story.</p>
  {/if}
</div>

<style>
  .story-details {
    padding: 20px;
    max-width: 800px;
    margin: 0 auto;
  }
  button {
    margin-bottom: 20px;
  }
  .content-block {
    margin-top: 20px;
    padding: 10px;
    background-color: #f0f0f0;
    border-radius: 5px;
  }
  .markdown-content {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      "Helvetica Neue", Arial, sans-serif;
  }
  .markdown-content h1,
  .markdown-content h2,
  .markdown-content h3,
  .markdown-content h4,
  .markdown-content h5,
  .markdown-content h6 {
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
    line-height: 1.25;
  }
  .markdown-content p {
    margin-top: 0;
    margin-bottom: 16px;
  }
  .markdown-content pre {
    padding: 16px;
    overflow: auto;
    font-size: 85%;
    line-height: 1.45;
    background-color: #f6f8fa;
    border-radius: 6px;
  }
  .markdown-content code {
    padding: 0.2em 0.4em;
    margin: 0;
    font-size: 85%;
    background-color: rgba(27, 31, 35, 0.05);
    border-radius: 6px;
  }
  .navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    /* background-color: #f8f8f8; */
    border-bottom: 1px solid #ccc;
  }

  .settings-icon {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 20px;
  }
</style>
