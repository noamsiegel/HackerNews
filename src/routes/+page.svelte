<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import Settings from "./settings/+page.svelte";
  import * as Sheet from "$lib/components/ui/sheet";
  import { settingsOpen } from "$lib/stores/settingsStore";
  import * as Card from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Badge } from "$lib/components/ui/badge";
  import * as Menubar from "$lib/components/ui/menubar";
  import { Settings as SettingsIcon } from "lucide-svelte";

  let stories: { id: number; title: string; score: number }[] = [];
  let loading = true;

  onMount(async () => {
    try {
      const storyIds: number[] = await invoke("fetch_top_stories");
      const storyPromises = storyIds.slice(0, 10).map(async (id) => {
        const story: any = await invoke("fetch_story", { id });
        return { id: story.id, title: story.title, score: story.score };
      });
      stories = await Promise.all(storyPromises);
    } catch (error) {
      console.error("Failed to fetch stories:", error);
    }
  });

  function openStory(id: number) {
    goto(`/story/${id}`);
  }
</script>

<!-- Navbar -->
<nav class="navbar">
  <h1>Hacker News</h1>
  <button class="settings-icon" on:click={() => goto("/settings")}>⚙️</button>
</nav>

<div class="story-list">
  {#each stories as story}
    <div
      class="story"
      on:click={() => openStory(story.id)}
      on:keydown={(e) => e.key === "Enter" && openStory(story.id)}
      tabindex="0"
      role="button"
    >
      <span class="title">{story.title}</span>
      <span class="score">{story.score}</span>
    </div>
  {/each}
</div>

<style>
  .navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    background-color: #f8f8f8;
    border-bottom: 1px solid #ccc;
  }
  .settings-icon {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 20px;
  }
  .story-list {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }
  .story {
    padding: 10px;
    margin-bottom: 10px;
    background-color: #f0f0f0;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .story:hover {
    background-color: #e0e0e0;
  }
  .title {
    flex-grow: 1;
    margin-right: 10px;
  }
  .score {
    font-weight: bold;
    min-width: 30px;
    text-align: right;
  }
</style>
