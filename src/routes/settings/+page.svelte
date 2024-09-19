<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import * as Sheet from "$lib/components/ui/sheet";
  import * as Select from "$lib/components/ui/select";

  let settings = {
    openAiApiKey: "",
    jinaAiApiKey: "",
    selectedPrompt: "SUMMARY_SYSTEM_PROMPT",
    selectedModel: "gpt-4o",
  };

  const prompts = [
    { value: "SUMMARY_SYSTEM_PROMPT", label: "Summary System Prompt" },
    { value: "PODCASTER_SYSTEM_PROMPT", label: "Podcaster System Prompt" },
  ];

  let models: { [key: string]: string } = {};

  onMount(async () => {
    const savedSettings = localStorage.getItem("settings");
    if (savedSettings) {
      settings = JSON.parse(savedSettings);
    }
    settings.selectedPrompt = await fetchCurrentPrompt();
    settings.openAiApiKey = await get_openai_api_key();
    models = await invoke("get_llm_models");

    settings.selectedModel = await fetchCurrentModel();
  });

  async function saveSettings() {
    localStorage.setItem("settings", JSON.stringify(settings));
    try {
      await invoke("update_selected_prompt", {
        prompt: settings.selectedPrompt,
      });
      await invoke("update_openai_api_key", {
        key: settings.openAiApiKey,
      });
      await invoke("update_selected_model", {
        model: settings.selectedModel,
      });
    } catch (error) {
      console.error("Error saving settings:", error);
    }
    console.log("Saving settings:", settings);
  }

  async function fetchCurrentPrompt() {
    const prompt = await invoke("get_selected_prompt");
    return prompt as string;
  }

  async function get_openai_api_key() {
    const openai_key = await invoke("get_openai_api_key");
    return openai_key as string;
  }

  async function fetchCurrentModel() {
    const model = await invoke("get_selected_model");
    return model as string;
  }
</script>

<Sheet.Header>
  <Sheet.Title>Settings</Sheet.Title>
  <Sheet.Description>Adjust your app preferences here.</Sheet.Description>
</Sheet.Header>

<div class="grid gap-4 py-4">
  <div class="grid grid-cols-4 items-center gap-4">
    <label for="openai-key" class="text-right">OpenAI API Key:</label>
    <input
      id="openai-key"
      bind:value={settings.openAiApiKey}
      on:change={saveSettings}
      class="col-span-3"
      type="password"
    />
  </div>
  <div class="grid grid-cols-4 items-center gap-4">
    <label for="jina-key" class="text-right">Jina AI API Key:</label>
    <input
      id="jina-key"
      bind:value={settings.jinaAiApiKey}
      on:change={saveSettings}
      class="col-span-3"
      type="password"
    />
  </div>
  <label>
    Select Prompt:
    <select bind:value={settings.selectedPrompt} on:change={saveSettings}>
      {#each prompts as prompt}
        <option value={prompt.value}>{prompt.label}</option>
      {/each}
    </select>
  </label>
  <label>
    Select Model:
    <select bind:value={settings.selectedModel} on:change={saveSettings}>
      {#each Object.entries(models) as [value, label]}
        <option {value}>{label}</option>
      {/each}
    </select>
  </label>
</div>

<style>
  label {
    display: block;
    margin: 10px 0;
  }
</style>
