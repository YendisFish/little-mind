import { SvelteMap } from "svelte/reactivity";

type LittleConfig = {
  url: string,
  model: string,
};

export const config: LittleConfig = {
  url: "http://localhost:11434/v1",
  model: ""
};

type LittleText = {
  role: "user" | "assistant",
  type: "text" | "thinking",
  content: string,
};

export class LittleJob {
  public content = $state<LittleText[]>([]);
  controller = new AbortController();

  public push(role: "user" | "assistant", type: "text" | "thinking", content: string) {
    if (this.content.length === 0) {
      const val: LittleText = {
        role,
        type,
        content,
      }
      this.content.push(val);
      return;
    }

    if (this.content[this.content.length - 1].type === type) {
      this.content[this.content.length - 1].content += content;
    } else {
      const val: LittleText = {
        role,
        type,
        content,
      }
      this.content.push(val);
    }
  }

  public contentToString(): string {
    let ret = "";

    for (const val of this.content.values()) {
      ret += val.content;
    }

    return ret;
  }
}

export interface Little {
  conf: LittleConfig,
  jobs: Map<string, LittleJob>,
  context: Map<string, any[]>,
}

export const complete = async (little: Little, job: LittleJob, reasoning: string) => {
  let stream = await fetch(`${little.conf.url}/chat/completions`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      model: "batiai/gemma4-26b:q3",
      messages: little.context.get("messages"),
      stream: true,
      reasoning_effort: reasoning,
      keep_alive: -1,
    }),
  });
  if (!stream.ok || !stream.body) throw new Error(`${stream.status}: ${await stream.text()}`);

  let reader = stream.body.getReader();
  let decoder = new TextDecoder();

  while (true) {
    const { done, value } = await reader.read();
    if (done) break;

    let buffer = decoder.decode(value, { stream: true });
    const lines = buffer.split("\n");
    buffer = lines.pop() ?? "";

    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed.startsWith("data:")) continue;
      const data = trimmed.slice(5).trim();
      if (data === "[DONE]") return;

      const delta = JSON.parse(data).choices?.[0]?.delta;
      if (delta?.reasoning) job.push("assistant", "thinking", delta.reasoning);
      if (delta?.content) job.push("assistant", "text", delta.content);
    }
  }
};
