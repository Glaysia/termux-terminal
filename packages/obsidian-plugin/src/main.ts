import { App, Plugin, PluginSettingTab, Setting, SettingDefinitionItem, WorkspaceLeaf } from "obsidian";
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
      callback: () => void this.openTerminal(),
    });
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

  getSettingDefinitions(): SettingDefinitionItem<keyof TermuxTerminalSettings>[] {
    return [
      {
        name: "Bridge URL",
        desc: "Local Termux bridge WebSocket URL.",
        control: { type: "text", key: "bridgeUrl" },
      },
      {
        name: "Bridge token",
        desc: "Token from ~/.termux_terminal_token.",
        render: (setting) => {
          setting.addText((text) => {
            text.inputEl.type = "password";
            text.setValue(this.plugin.settings.token);
            text.onChange(async (value) => {
              this.plugin.settings.token = value;
              await this.plugin.saveSettings();
            });
          });
        },
      },
      {
        name: "Input debug log",
        desc: "Show session-local keyboard and terminal input records. Logs are not saved or sent anywhere else.",
        control: { type: "toggle", key: "inputDebugLog" },
      },
    ];
  }
}
