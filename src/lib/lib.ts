import { invoke } from "@tauri-apps/api/core";

export async function grabConfig(): Promise<string> {
  return await invoke("grab_config");
}

export async function createChat(): Promise<string> {
  return await invoke("create_chat");
}

export async function getChatIds(): Promise<string[]> {
  return await invoke("get_chat_ids");
}
