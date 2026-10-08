<script lang="ts">
    import "../css/app.css";
    import { goto } from "$app/navigation";
    import { createChat, getChatIds } from "$lib/lib";
    import { onMount } from "svelte";

    let chats: string[] = $state(["asdfasdf"]);

    onMount(async () => {
      chats = await getChatIds();
    });

    async function createNewChat() {
      let id = await createChat();
      goto("/chat/" + id);
    }

    async function gotoChat(chatId: string) {
      goto("/chat/" + chatId);
    }
</script>

<main data-theme="dark">
    <h1 class="text-3xl">Welcome to Little Mind!</h1>
    <button onclick={() => goto('/chat')}>Chat</button>
    <button onclick={async () => await createNewChat()}>New Chat</button>
    <button onclick={() => goto('/intro')}>Intro</button>

    <br />

    {#each chats as chat}
        <button onclick={async () => await gotoChat(chat)}>{chat}</button>
    {/each}
</main>
