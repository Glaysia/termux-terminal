export interface BridgeRuntimeInfo {
  host: string;
  port: number;
}

export const DEFAULT_BRIDGE_RUNTIME: BridgeRuntimeInfo = {
  host: "127.0.0.1",
  port: 11557
};
