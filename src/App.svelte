<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { ModeWatcher } from "mode-watcher";
  import "./app.css";
  import MenuBarApp from "./components/MenuBarApp.svelte";
  let a = 43;
  let fd = 323;
</script>

<main class="text-xs">
  <ModeWatcher />
  <MenuBarApp />

  <Button>Heil Heydrich</Button>
</main>
