<script lang="ts">
    let props: any = $props();
</script>

<div class="max-w-[50vw] mb-2.5">
    {#each props.job.content as content}
        {#if content.type === "text"}
          <p>{content.content}</p>
        {/if}
        {#if content.type === "thinking"}
            <p class="text-red-500">{content.content}</p>
        {/if}
    {/each}
</div>
