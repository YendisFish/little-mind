import { invoke } from "@tauri-apps/api/core";

export async function grabConfig(): Promise<string> {
  return await invoke("grab_config");
}
