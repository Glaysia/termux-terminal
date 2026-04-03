import { Plugin } from "obsidian";

export default class ObsidianTermuxPlugin extends Plugin {
  async onload(): Promise<void> {
    console.log("obsidian-termux plugin loaded");
  }

  onunload(): void {
    console.log("obsidian-termux plugin unloaded");
  }
}
