<script lang="ts">
    import "../css/app.css";
    import { goto } from "$app/navigation";
</script>

<main data-theme="dark">
    <h1 class="text-3xl">Welcome to Little Mind!</h1>
    <button onclick={() => goto('/chat')}>Chat</button>
    <button onclick={() => goto('/intro')}>Intro</button>
</main>
