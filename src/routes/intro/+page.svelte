<script lang="ts">
    import { complete, config, LittleJob, type Little } from "$lib/little.svelte";
    import { onDestroy } from "svelte";
    import ChatBox from "../../components/ChatBox.svelte";
    import { goto, invalidateAll } from "$app/navigation";
    import { politicalQuizQuestions, popCultureQuizQuestions } from "$lib/checkins";

    // let questions = [
    //   "Whats your name?",
    //   "What is your favorite color?",
    //   "Do you like superheros, if so who?",
    //   "Do you like princesses, if so which ones?",
    //   "Whats your favorite movie?"
    // ];
    let questions = popCultureQuizQuestions;

    let currentQuestion = $state(0);
    let answer = $state("");
    let final = $state(new LittleJob());

    onDestroy(() => {
      final.controller.abort();
    });

    let lttle: Little = {
      conf: config,
      jobs: new Map(),
      context: new Map([["messages", []]]),
    };

    let collected = "";

    async function answerAndNext() {
      collected += "[QUESTION]";
      collected += questions[currentQuestion];
      collected += "[ANSWER]";
      collected += answer;

      currentQuestion += 1;
      answer = "";

      if(questions.length <= currentQuestion) {
        currentQuestion = -1;

        let prompt = `
          You are an LLM agent tasked with determining the identity, labels,
          and orientations that best match the answers to the questions given
          below.

          Your answer should include the following sections:

          - Identity: You should neatly and cleanly give a single word to describe
          the orientation of the user, and also explain what the word means. Try and
          stay way from 1 word descriptions of identity unless it really is one word.
          If you can include important and specific markers then that would be very
          helpful.
          - Peers: You should give a list of 3 people the user may agree with or
          enjoy looking into. These people should closely match the users preferences.
          Please describe how the users preferences align with those religious figures
          and also describe the figures ideology to the user.
          - Alternative Viewpoints: You should give a list of 3 people that challenge the users position
          and may be worth looking into. Frame this list less as a "this is why your wrong"
          and more like a "if you enjoy looking at alternative viewpoints".
          - Breakdown: Please breakdown the users preferences from their answers and
          describe where within the spectrum they fall. Please try to avoid bias and
          only base your answer off of their own.

          Please ensure that your answer follows this structure and that sections are
          clearly labeled. Also try not to go off topic or talk too much. It should
          be neat and easy to understand. Be detailed but concise. It's okay to have a
          moderately longed answer, or a moderately short one. Just give what is needed.

          Also please ensure that your answer is framed as though you are talking
          directly to the user. You should be warm and friendly to them. You're helping
          them reflect on themselves as the main goal of this message.

          The questions and answers given to the user are as follows:
          ${collected}
        `;

        lttle.context.get("messages")?.push({ role: "user", content: prompt });

        let job = new LittleJob();
        final = job;

        await complete(lttle, job, "high");
      }
    }

    document.addEventListener("keydown", async (e) => {
      if(e.key === "Escape") {
        final.controller.abort();
      }

      if(e.key === "Enter" && e.shiftKey) {
        await answerAndNext();
      }
    });
</script>

<div>
    {#if currentQuestion > -1}
        <p>{questions[currentQuestion]}</p>
    {/if}
    {#if currentQuestion < 0}
        <ChatBox job={final} />
    {/if}
    <textarea bind:value={answer}></textarea>
    <button onclick={async () => await answerAndNext()}>Next</button>
</div>
