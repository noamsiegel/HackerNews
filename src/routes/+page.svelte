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
    loading = false;
  });

  function openStory(id: number) {
    goto(`/story/${id}`);
  }
</script>

<Menubar.Root class="navbar">
  <h1 class="text-xl font-bold">Hacker News</h1>
  <div class="flex-grow"></div>
  <Button variant="ghost" on:click={() => ($settingsOpen = true)} size="icon">
    <SettingsIcon class="h-4 w-4" />
    <span class="sr-only">Open Settings</span>
  </Button>
</Menubar.Root>

<div class="story-list">
  {#if loading}
    {#each Array(10) as _}
      <Card.Root class="mb-4">
        <Card.Header>
          <div class="flex justify-between items-center">
            <Skeleton class="h-4 w-3/4" />
            <Skeleton class="h-4 w-16" />
          </div>
        </Card.Header>
      </Card.Root>
    {/each}
  {:else}
    {#each stories as story (story.id)}
      <Card.Root class="mb-4 cursor-pointer hover:bg-accent transition-colors">
        <Card.Header>
          <button
            class="w-full text-left"
            on:click={() => openStory(story.id)}
            on:keydown={(e) => e.key === "Enter" && openStory(story.id)}
          >
            <div class="flex justify-between items-center">
              <Card.Title>{story.title}</Card.Title>
              <Badge variant="secondary">{story.score}</Badge>
            </div>
          </button>
        </Card.Header>
      </Card.Root>
    {/each}
  {/if}
</div>

<Sheet.Root bind:open={$settingsOpen}>
  <Sheet.Content>
    <Settings />
  </Sheet.Content>
</Sheet.Root>

<style>
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
  .story-list {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }
</style>
