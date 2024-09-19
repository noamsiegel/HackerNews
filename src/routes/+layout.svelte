<!-- <script>import "../app.css";</script><slot></slot> -->
<script lang="ts">
  import "../app.pcss";
  import Settings from "./settings/+page.svelte";
  import * as Sheet from "$lib/components/ui/sheet";
  import { settingsOpen } from "$lib/stores/settingsStore";
</script>

<slot />

<Sheet.Root bind:open={$settingsOpen}>
  <Sheet.Content>
    <Settings />
  </Sheet.Content>
</Sheet.Root>
