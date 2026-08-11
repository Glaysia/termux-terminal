export const DEFAULT_BRIDGE_URL = "ws://127.0.0.1:11557";
const CONNECTION_TIMEOUT_MS = 5_000;
const RECONNECT_DELAYS_MS = [1_000, 2_000, 5_000, 10_000];

export type BridgeStatus =
  | "connecting"
  | "connected"
  | "disconnected"
  | "exited"
  | "error";

interface BridgeClientOptions {
  onOutput(data: string): void;
  onError(message: string): void;
  onStatus(status: BridgeStatus): void;
  onExit(exitCode: number): void;
}

interface BridgeMessage {
  type?: unknown;
  data?: unknown;
  exitCode?: unknown;
  message?: unknown;
}

export class BridgeClient {
  private socket: WebSocket | null = null;
  private attached = false;
  private connectionTimeout: number | null = null;
  private reconnectTimeout: number | null = null;
  private closed = false;
  private terminalExited = false;
  private reportedConnectionFailure = false;
  private reconnectAttempt = 0;
  private authenticationFailed = false;

  constructor(
    private readonly url: string,
    private readonly version: string,
    private readonly token: string,
    private readonly options: BridgeClientOptions,
  ) {}

  connect(): void {
    this.closed = false;
    this.terminalExited = false;
    this.authenticationFailed = false;
    this.openSocket();
  }

  close(): void {
    this.closed = true;
    this.clearConnectionTimeout();
    if (this.reconnectTimeout !== null) {
      window.clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }
    const socket = this.socket;
    this.socket = null;
    this.attached = false;
    socket?.close();
  }

  sendInput(data: string): void {
    if (!this.attached || data.length === 0) {
      return;
    }
    this.send({ type: "terminal.input", data });
  }

  resize(cols: number, rows: number): void {
    if (
      !this.attached ||
      !Number.isInteger(cols) ||
      !Number.isInteger(rows) ||
      cols < 1 ||
      rows < 1 ||
      cols > 65_535 ||
      rows > 65_535
    ) {
      return;
    }
    this.send({ type: "terminal.resize", cols, rows });
  }

  private openSocket(): void {
    if (this.socket !== null || this.closed) {
      return;
    }

    this.options.onStatus("connecting");
    const socket = new WebSocket(this.url);
    this.socket = socket;
    this.connectionTimeout = window.setTimeout(() => {
      if (this.socket !== socket) {
        return;
      }
      this.reportConnectionFailure("Timed out connecting to the local Termux bridge.");
      socket.close();
    }, CONNECTION_TIMEOUT_MS);

    socket.addEventListener("open", () => {
      if (this.socket !== socket) {
        return;
      }
      this.clearConnectionTimeout();
      this.send({ type: "hello", client: "obsidian-plugin", version: this.version, token: this.token });
    });

    socket.addEventListener("message", (event: MessageEvent<unknown>) => {
      if (this.socket !== socket) {
        return;
      }
      this.handleMessage(event.data);
    });

    socket.addEventListener("error", () => {
      if (this.socket === socket) {
        this.attached = false;
        this.reportConnectionFailure("Could not connect to the local Termux bridge.");
      }
    });

    socket.addEventListener("close", () => {
      if (this.socket === socket) {
        this.socket = null;
        this.attached = false;
        this.clearConnectionTimeout();
        if (this.terminalExited || this.authenticationFailed) {
          return;
        }
        this.options.onStatus("disconnected");
        this.scheduleReconnect();
      }
    });
  }

  private handleMessage(data: unknown): void {
    if (typeof data !== "string") {
      this.options.onStatus("error");
      this.options.onError("The Termux bridge sent a non-text WebSocket message.");
      return;
    }

    let message: BridgeMessage;
    try {
      message = JSON.parse(data) as BridgeMessage;
    } catch {
      this.options.onStatus("error");
      this.options.onError("The Termux bridge sent invalid JSON.");
      return;
    }

    switch (message.type) {
      case "hello.ack":
        this.send({ type: "session.create" });
        break;
      case "session.ready":
        this.send({ type: "session.attach" });
        this.attached = true;
        this.reportedConnectionFailure = false;
        this.reconnectAttempt = 0;
        this.options.onStatus("connected");
        break;
      case "terminal.output":
        if (typeof message.data === "string") {
          this.options.onOutput(message.data);
        }
        break;
      case "terminal.exit":
        this.attached = false;
        this.terminalExited = true;
        this.options.onStatus("exited");
        this.options.onExit(
          typeof message.exitCode === "number" ? message.exitCode : -1,
        );
        this.socket?.close();
        break;
      case "error":
        this.attached = false;
        this.authenticationFailed = true;
        this.options.onStatus("error");
        this.options.onError(
          typeof message.message === "string"
            ? message.message
            : "The Termux bridge rejected a request.",
        );
        break;
      default:
        this.options.onStatus("error");
        this.options.onError("The Termux bridge sent an unsupported message.");
    }
  }

  private send(message: Record<string, unknown>): void {
    if (this.socket?.readyState !== WebSocket.OPEN) {
      return;
    }
    this.socket.send(JSON.stringify(message));
  }

  private clearConnectionTimeout(): void {
    if (this.connectionTimeout !== null) {
      window.clearTimeout(this.connectionTimeout);
      this.connectionTimeout = null;
    }
  }

  private reportConnectionFailure(message: string): void {
    this.options.onStatus("error");
    if (!this.reportedConnectionFailure) {
      this.reportedConnectionFailure = true;
      this.options.onError(message);
    }
  }

  private scheduleReconnect(): void {
    if (this.closed || this.reconnectTimeout !== null) {
      return;
    }
    const delay = RECONNECT_DELAYS_MS[Math.min(this.reconnectAttempt, RECONNECT_DELAYS_MS.length - 1)];
    this.reconnectAttempt += 1;
    this.reconnectTimeout = window.setTimeout(() => {
      this.reconnectTimeout = null;
      this.openSocket();
    }, delay);
  }
}
