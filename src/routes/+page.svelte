<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import "../css/app.css";
    import { fly } from "svelte/transition";
    import { goto } from "$app/navigation";

    let name = $state("");
    let greetMsg = $state("");

    async function greet(event: Event) {
      event.preventDefault();
      // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
      greetMsg = await invoke("greet", { name });
    }
</script>

<main data-theme="dark">
    <h1 class="text-3xl" transition:fly>Welcome to Little Mind!</h1>
    <button onclick={() => goto('/chat')}>Chat</button>
    <button onclick={() => goto('/intro')}>Intro</button>
</main>
