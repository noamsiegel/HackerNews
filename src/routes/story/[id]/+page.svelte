<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { marked } from "marked";
  import { Button } from "$lib/components/ui/button";
  import { Badge } from "$lib/components/ui/badge";
  import * as Card from "$lib/components/ui/card";
  import { Separator } from "$lib/components/ui/separator";
  import * as Menubar from "$lib/components/ui/menubar";
  import { ChevronLeft, Settings as SettingsIcon } from "lucide-svelte";
  import { settingsOpen } from "$lib/stores/settingsStore";
  import { Skeleton } from "$lib/components/ui/skeleton";

  let story: any = null;
  let loading = true;
  let content: string | null = null;
  let contentLoading = false;
  let summary: string | null = null;
  let summaryLoading = false;

  function openSettings() {
    $settingsOpen = true;
  }

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
  <Menubar.Root class="navbar">
    <Button variant="ghost" on:click={goBack} size="icon">
      <ChevronLeft class="h-4 w-4" />
      <span class="sr-only">Back to Home</span>
    </Button>
    <div class="flex-grow"></div>
    <Button variant="ghost" on:click={openSettings} size="icon">
      <SettingsIcon class="h-4 w-4" />
      <span class="sr-only">Open Settings</span>
    </Button>
  </Menubar.Root>

  {#if loading}
    <Card.Root class="mt-4">
      <Card.Header>
        <Skeleton class="h-8 w-3/4" />
        <Card.Description>
          <div class="flex space-x-2">
            <Skeleton class="h-5 w-20" />
            <Skeleton class="h-5 w-20" />
            <Skeleton class="h-5 w-24" />
          </div>
        </Card.Description>
      </Card.Header>
      <Card.Content>
        <div class="flex space-x-2 mb-4">
          <Skeleton class="h-10 w-24" />
          <Skeleton class="h-10 w-32" />
          <Skeleton class="h-10 w-28" />
        </div>
        <Skeleton class="h-4 w-full" />
        <Skeleton class="h-4 w-full mt-2" />
        <Skeleton class="h-4 w-3/4 mt-2" />
      </Card.Content>
    </Card.Root>
  {:else if story}
    <Card.Root class="mt-4">
      <Card.Header>
        <Card.Title>{story.title}</Card.Title>
        <Card.Description>
          <!-- By: {story.by} -->
          <!-- <Badge variant -->
          <!-- make the by  story a badge -->
          <Badge variant="secondary" class="ml-2">By: {story.by}</Badge>
          <Badge variant="secondary" class="ml-2">By: {story.score}</Badge>
          <Badge variant="secondary" class="ml-2"
            >Comments: {story.descendants}</Badge
          >
        </Card.Description>
      </Card.Header>
      <Card.Content>
        {#if story.url}
          <div class="flex space-x-2 mb-4">
            <Button
              variant="outline"
              href={story.url}
              target="_blank"
              rel="noopener noreferrer"
            >
              Go to article
            </Button>
            <Button
              variant="outline"
              on:click={showContent}
              disabled={contentLoading}
            >
              {contentLoading ? "Loading..." : "Scrape Content"}
            </Button>
            <Button
              variant="outline"
              on:click={summarizeContent}
              disabled={summaryLoading}
            >
              {summaryLoading ? "Summarizing..." : "Summarize"}
            </Button>
          </div>
        {/if}
        {#if story.text}
          <p>{@html story.text}</p>
        {/if}

        {#if content}
          <Separator class="my-4" />
          <h2 class="text-xl font-semibold mb-2">Article Content:</h2>
          <div class="markdown-content">{@html marked(content)}</div>
        {/if}

        {#if summary}
          <Separator class="my-4" />
          <h2 class="text-xl font-semibold mb-2">Summary:</h2>
          <div class="markdown-content">{@html marked(summary)}</div>
        {/if}
      </Card.Content>
    </Card.Root>
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
    padding: 5px 10px;
    background-color: #f8f8f8;
    margin-bottom: 20px;
  }
  .flex-grow {
    flex-grow: 1;
  }
</style>
