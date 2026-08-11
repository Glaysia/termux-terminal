import { ItemView, WorkspaceLeaf } from "obsidian";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import {
  BridgeClient,
  BridgeStatus,
  DEFAULT_BRIDGE_URL,
} from "./bridge-client";
import { PLUGIN_VERSION } from "./build-version";

export const TERMINAL_VIEW_TYPE = "obsidian-termux-terminal";

export class TerminalView extends ItemView {
  private terminal: Terminal | null = null;
  private fitAddon: FitAddon | null = null;
  private bridgeClient: BridgeClient | null = null;
  private statusEl: HTMLElement | null = null;
  private resizeObserver: ResizeObserver | null = null;
  private resizeFrame: number | null = null;
  private bridgeStatus: BridgeStatus | null = null;

  constructor(leaf: WorkspaceLeaf, private readonly getConnection: () => { url: string; token: string }) {
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
    terminal.onData((data) => this.bridgeClient?.sendInput(data));
    terminal.attachCustomKeyEventHandler((event) => {
      if (
        event.type === "keydown" &&
        event.ctrlKey &&
        !event.altKey &&
        !event.metaKey &&
        event.key.toLowerCase() === "d"
      ) {
        this.bridgeClient?.sendInput("\u0004");
        return false;
      }
      return true;
    });

    this.terminal = terminal;
    this.fitAddon = fitAddon;
    this.statusEl = statusEl;
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
