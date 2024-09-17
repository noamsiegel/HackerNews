<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let settings = {
    openAiApiKey: "",
    jinaAiApiKey: "",
    selectedPrompt: "SUMMARY_SYSTEM_PROMPT",
  };

  const prompts = [
    { value: "SUMMARY_SYSTEM_PROMPT", label: "Summary System Prompt" },
    { value: "PODCASTER_SYSTEM_PROMPT", label: "Podcaster System Prompt" },
  ];

  let showBanner = false;
  let bannerMessage = "";
  let currentPrompt = "";

  onMount(async () => {
    const savedSettings = localStorage.getItem("settings");
    if (savedSettings) {
      settings = JSON.parse(savedSettings);
    }
    settings.selectedPrompt = await fetchCurrentPrompt();
    settings.openAiApiKey = await get_openai_api_key();
  });

  function goBack() {
    goto("/");
  }

  async function saveSettings() {
    localStorage.setItem("settings", JSON.stringify(settings));
    try {
      await invoke("update_selected_prompt", {
        prompt: settings.selectedPrompt,
      });
      await invoke("update_openai_api_key", {
        key: settings.openAiApiKey,
      });
      showBanner = true;
      bannerMessage = `Settings saved`;
    } catch (error) {
      console.error("Error updating selected prompt:", error);
      showBanner = true;
      bannerMessage = "Error saving settings. Please try again.";
    }
    setTimeout(() => {
      showBanner = false;
    }, 3000);
  }

  async function fetchCurrentPrompt() {
    const prompt = await invoke("get_selected_prompt");
    return prompt as string;
  }

  async function get_openai_api_key() {
    const openai_key = await invoke("get_openai_api_key");
    return openai_key as string;
  }
</script>

<nav class="navbar">
  <h1>Settings</h1>
  <button class="back-icon" on:click={goBack}>←</button>
</nav>

<div class="settings-container">
  <h2>Application Settings</h2>
  <label>
    Open AI API Key:
    <input
      type="text"
      bind:value={settings.openAiApiKey}
      on:change={saveSettings}
    />
  </label>
  <label>
    Jina AI API Key:
    <input
      type="text"
      bind:value={settings.jinaAiApiKey}
      on:change={saveSettings}
    />
  </label>

  <label>
    Select Prompt:
    <select bind:value={settings.selectedPrompt} on:change={saveSettings}>
      {#each prompts as prompt}
        <option value={prompt.value}>{prompt.label}</option>
      {/each}
    </select>
  </label>

  {#if showBanner}
    <div class="banner">{bannerMessage}</div>
  {/if}
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
  .back-icon {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 20px;
  }
  .settings-container {
    padding: 20px;
    max-width: 800px;
    margin: 0 auto;
  }
  label {
    display: block;
    margin: 10px 0;
  }
  select {
    width: 100%;
    padding: 5px;
    margin-top: 5px;
  }
  .banner {
    margin-top: 20px;
    padding: 10px;
    background-color: #4caf50;
    color: white;
    text-align: center;
    border-radius: 5px;
  }
</style>
