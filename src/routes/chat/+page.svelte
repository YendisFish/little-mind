<script lang="ts">
import type { Little } from "$lib/little.svelte";
import { config, complete, LittleJob } from "$lib/little.svelte";
    import ChatBox from "../../components/ChatBox.svelte";
import "../../css/app.css"

let jobs: LittleJob[] = $state([]);
let msg = $state("");
let currentJob: LittleJob | null = null;

let lttle: Little = {
  conf: config,
  jobs: new Map(),
  context: new Map([["messages", []]]),
};

async function prompt() {
  lttle.context.get("messages")?.push({ role: "user", content: msg });
  let faux = new LittleJob();
  faux.push("user", "text", msg);
  jobs.push(faux);

  let j = new LittleJob();
  jobs.push(j);
  currentJob = j;

  await complete(lttle, j, "high");
  lttle.context.get("messages")?.push({ role: "assistant", content: j.contentToString() });
}

document.addEventListener("keydown", (e) => {
  if(e.key === "Escape") {
    currentJob?.controller.abort();
  }
});
</script>

<div>
    {#each jobs as job}
          <ChatBox job={job} />
    {/each}
    <input bind:value={msg} type="text"/>
    <button onclick={async () => await prompt()}>Send</button>
</div>
