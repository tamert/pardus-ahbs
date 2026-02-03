<script lang="ts">
  import "../app.css";
  import { Theme } from "carbon-components-svelte";
  import { onMount } from "svelte";

  let { children } = $props();
  let theme = $state<"white" | "g10" | "g80" | "g90" | "g100">("white");

  onMount(() => {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const updateTheme = (e: MediaQueryListEvent | MediaQueryList) => {
      theme = e.matches ? "g100" : "white";
      if (e.matches) {
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
      }
    };

    updateTheme(mediaQuery);
    mediaQuery.addEventListener("change", updateTheme);

    return () => {
      mediaQuery.removeEventListener("change", updateTheme);
    };
  });
</script>

<Theme bind:theme persist persistKey="ahbs-theme">
  <div class="min-h-screen">
    {@render children()}
  </div>
</Theme>
