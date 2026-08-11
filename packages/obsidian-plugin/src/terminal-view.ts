import { ItemView, WorkspaceLeaf, setIcon } from "obsidian";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import {
  BridgeClient,
  BridgeStatus,
  DEFAULT_BRIDGE_URL,
} from "./bridge-client";
import { PLUGIN_VERSION } from "./build-version";

export const TERMINAL_VIEW_TYPE = "obsidian-termux-terminal";
const INPUT_DEBUG_LOG_LIMIT = 2_000;

interface TerminalConnection {
  url: string;
  token: string;
  inputDebugLog: boolean;
}

export class TerminalView extends ItemView {
  private terminal: Terminal | null = null;
  private fitAddon: FitAddon | null = null;
  private bridgeClient: BridgeClient | null = null;
  private statusEl: HTMLElement | null = null;
  private resizeObserver: ResizeObserver | null = null;
  private resizeFrame: number | null = null;
  private bridgeStatus: BridgeStatus | null = null;
  private inputDebugEl: HTMLTextAreaElement | null = null;
  private inputDebugEntries: string[] = [];
  private terminalContainer: HTMLElement | null = null;
  private readonly windowKeyDownHandler = (event: KeyboardEvent) => this.handleWindowKeyDown(event);

  constructor(leaf: WorkspaceLeaf, private readonly getConnection: () => TerminalConnection) {
    super(leaf);
  }

  getViewType(): string {
    return TERMINAL_VIEW_TYPE;
  }

  getDisplayText(): string {
    return "bash";
  }

  async onOpen(): Promise<void> {
    this.contentEl.empty();
    this.contentEl.addClass("obsidian-termux-terminal");

    const terminalContainer = this.contentEl.createDiv({
      cls: "obsidian-termux-terminal__surface",
    });
    const statusEl = this.contentEl.createDiv({
      cls: "obsidian-termux-terminal__status",
    });
    const terminal = new Terminal({ disableStdin: true });
    const fitAddon = new FitAddon();

    terminal.loadAddon(fitAddon);
    terminal.open(terminalContainer);
    terminal.onData((data) => this.sendInput(data, "xterm"));
    terminalContainer.addEventListener("keydown", (event) => this.handleKeyDown(event), true);
    window.addEventListener("keydown", this.windowKeyDownHandler, true);

    this.terminal = terminal;
    this.fitAddon = fitAddon;
    this.terminalContainer = terminalContainer;
    this.statusEl = statusEl;
    if (this.getConnection().inputDebugLog) {
      this.createInputDebugPanel();
    }
    this.resizeObserver = new ResizeObserver(() => this.scheduleFit());
    this.resizeObserver.observe(terminalContainer);
    this.scheduleFit();
    this.connectBridge();
  }

  async onClose(): Promise<void> {
    this.bridgeClient?.close();
    this.bridgeClient = null;
    this.resizeObserver?.disconnect();
    this.resizeObserver = null;
    if (this.resizeFrame !== null) {
      window.cancelAnimationFrame(this.resizeFrame);
      this.resizeFrame = null;
    }
    this.fitAddon?.dispose();
    this.fitAddon = null;
    this.terminal?.dispose();
    this.terminal = null;
    this.statusEl = null;
    this.bridgeStatus = null;
    this.inputDebugEl = null;
    this.inputDebugEntries = [];
    window.removeEventListener("keydown", this.windowKeyDownHandler, true);
    this.terminalContainer = null;
    this.contentEl.empty();
  }

  private connectBridge(): void {
    const connection = this.getConnection();
    this.bridgeClient = new BridgeClient(connection.url || DEFAULT_BRIDGE_URL, PLUGIN_VERSION, connection.token, {
      onOutput: (data) => this.terminal?.write(data),
      onError: (message) => this.terminal?.writeln(`\r\n[bridge] ${message}`),
      onStatus: (status) => this.setBridgeStatus(status),
      onExit: (exitCode) =>
        this.handleTerminalExit(exitCode),
    });
    this.bridgeClient.connect();
  }

  private handleKeyDown(event: KeyboardEvent): void {
    this.recordInputDebug(`key ${this.describeKeyEvent(event)}`);
  }

  private handleWindowKeyDown(event: KeyboardEvent): void {
    if (!this.terminalHasFocus() || !event.ctrlKey || event.altKey || event.metaKey) {
      return;
    }

    this.recordInputDebug(`window key ${this.describeKeyEvent(event)}`);
    const controlByte = this.controlByteFor(event.key);
    if (controlByte === null) {
      return;
    }

    event.preventDefault();
    event.stopImmediatePropagation();
    this.sendInput(controlByte, `Ctrl-${event.key.toUpperCase()}`);
  }

  private sendInput(data: string, source: string): void {
    this.recordInputDebug(`send ${source} ${JSON.stringify(data)}`);
    this.bridgeClient?.sendInput(data);
  }

  private createInputDebugPanel(): void {
    const debugPanel = this.contentEl.createDiv({ cls: "obsidian-termux-terminal__input-debug-panel" });
    const controls = debugPanel.createDiv({ cls: "obsidian-termux-terminal__input-debug-controls" });
    const copyButton = controls.createEl("button", {
      attr: { type: "button", "aria-label": "Copy terminal input debug log", title: "Copy input log" },
    });
    setIcon(copyButton, "copy");
    copyButton.addEventListener("click", () => void navigator.clipboard.writeText(this.inputDebugEntries.join("\n")));
    const clearButton = controls.createEl("button", {
      attr: { type: "button", "aria-label": "Clear terminal input debug log", title: "Clear input log" },
    });
    setIcon(clearButton, "trash-2");
    clearButton.addEventListener("click", () => {
      this.inputDebugEntries = [];
      if (this.inputDebugEl !== null) {
        this.inputDebugEl.value = "";
      }
    });
    const debugEl = debugPanel.createEl("textarea", {
      cls: "obsidian-termux-terminal__input-debug",
      attr: { readonly: "readonly", spellcheck: "false", "aria-label": "Terminal input debug log" },
    });
    debugEl.rows = 8;
    this.inputDebugEl = debugEl;
    this.recordInputDebug("input diagnostics enabled");
  }

  private recordInputDebug(entry: string): void {
    if (this.inputDebugEl === null) {
      return;
    }
    this.inputDebugEntries.push(`${new Date().toISOString()} ${entry}`);
    if (this.inputDebugEntries.length > INPUT_DEBUG_LOG_LIMIT) {
      this.inputDebugEntries.splice(0, this.inputDebugEntries.length - INPUT_DEBUG_LOG_LIMIT);
    }
    this.inputDebugEl.value = this.inputDebugEntries.join("\n");
    this.inputDebugEl.scrollTop = this.inputDebugEl.scrollHeight;
  }

  private describeKeyEvent(event: KeyboardEvent): string {
    return JSON.stringify({
      key: event.key,
      code: event.code,
      ctrl: event.ctrlKey,
      alt: event.altKey,
      shift: event.shiftKey,
      meta: event.metaKey,
      repeat: event.repeat,
    });
  }

  private terminalHasFocus(): boolean {
    return this.terminalContainer?.contains(document.activeElement) ?? false;
  }

  private controlByteFor(key: string): string | null {
    if (key.length !== 1) {
      return null;
    }
    const charCode = key.toUpperCase().charCodeAt(0);
    return charCode >= 64 && charCode <= 95
      ? String.fromCharCode(charCode - 64)
      : null;
  }

  private setBridgeStatus(status: BridgeStatus): void {
    if (status === this.bridgeStatus) {
      return;
    }
    this.bridgeStatus = status;
    if (this.terminal !== null) {
      this.terminal.options.disableStdin = status !== "connected";
    }
    if (this.statusEl === null) {
      return;
    }
    this.statusEl.dataset.status = status;
    this.statusEl.ariaLabel = `Termux bridge ${status}`;
    switch (status) {
      case "connecting":
        this.terminal?.writeln("Connecting to Termux bridge...");
        break;
      case "connected":
        this.terminal?.writeln("Connected.");
        this.sendTerminalSize();
        break;
      case "disconnected":
        this.terminal?.writeln("Disconnected. Retrying...");
        break;
      case "exited":
        break;
      case "error":
        break;
    }
  }

  private scheduleFit(): void {
    if (this.resizeFrame !== null) {
      return;
    }

    this.resizeFrame = window.requestAnimationFrame(() => {
      this.resizeFrame = null;
      if (this.contentEl.clientWidth === 0 || this.contentEl.clientHeight === 0) {
        return;
      }
      this.fitAddon?.fit();
      this.sendTerminalSize();
    });
  }

  private sendTerminalSize(): void {
    if (this.terminal === null) {
      return;
    }
    this.bridgeClient?.resize(this.terminal.cols, this.terminal.rows);
  }

  private handleTerminalExit(exitCode: number): void {
    this.terminal?.writeln(`\r\n[process exited with code ${exitCode}]`);
    window.setTimeout(() => this.leaf.detach(), 3_000);
  }
}
