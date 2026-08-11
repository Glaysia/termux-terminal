import { App, Plugin, PluginSettingTab, Setting, WorkspaceLeaf } from "obsidian";
import { TERMINAL_VIEW_TYPE, TerminalView } from "./terminal-view";

interface TermuxTerminalSettings { bridgeUrl: string; token: string; inputDebugLog: boolean; }
const DEFAULT_SETTINGS: TermuxTerminalSettings = { bridgeUrl: "ws://127.0.0.1:11557", token: "", inputDebugLog: false };

export default class ObsidianTermuxPlugin extends Plugin {
  settings: TermuxTerminalSettings = DEFAULT_SETTINGS;

  async onload(): Promise<void> {
    this.settings = { ...DEFAULT_SETTINGS, ...(await this.loadData() as Partial<TermuxTerminalSettings> | null) };
    this.registerView(
      TERMINAL_VIEW_TYPE,
      (leaf: WorkspaceLeaf) => new TerminalView(leaf, () => ({ url: this.settings.bridgeUrl, token: this.settings.token, inputDebugLog: this.settings.inputDebugLog })),
    );
    this.addSettingTab(new TermuxTerminalSettingTab(this.app, this));
    this.addRibbonIcon("terminal", "Open Termux terminal", () => void this.openTerminal());

    this.addCommand({
      id: "open-terminal",
      name: "Open terminal",
      callback: () => this.openTerminal(),
    });
  }

  onunload(): void {
    this.app.workspace.detachLeavesOfType(TERMINAL_VIEW_TYPE);
  }

  private async openTerminal(): Promise<void> {
    const leaf = this.app.workspace.getLeaf("tab");
    await leaf.setViewState({ type: TERMINAL_VIEW_TYPE, active: true });
    this.app.workspace.revealLeaf(leaf);
  }

  async saveSettings(): Promise<void> { await this.saveData(this.settings); }
}

class TermuxTerminalSettingTab extends PluginSettingTab {
  constructor(app: App, private readonly plugin: ObsidianTermuxPlugin) { super(app, plugin); }
  display(): void {
    const { containerEl } = this;
    containerEl.empty();
    new Setting(containerEl).setName("Bridge URL").setDesc("Local Termux bridge WebSocket URL.").addText((text) => text.setValue(this.plugin.settings.bridgeUrl).onChange(async (value) => { this.plugin.settings.bridgeUrl = value.trim(); await this.plugin.saveSettings(); }));
    new Setting(containerEl).setName("Bridge token").setDesc("Token from ~/.termux_terminal_token.").addText((text) => { text.inputEl.type = "password"; text.setValue(this.plugin.settings.token).onChange(async (value) => { this.plugin.settings.token = value; await this.plugin.saveSettings(); }); });
    new Setting(containerEl).setName("Input debug log").setDesc("Show session-local keyboard and terminal input records. Logs are not saved or sent anywhere else.").addToggle((toggle) => toggle.setValue(this.plugin.settings.inputDebugLog).onChange(async (value) => { this.plugin.settings.inputDebugLog = value; await this.plugin.saveSettings(); }));
  }
}
