import * as vscode from "vscode";
import { CubensisRpcClient } from "../client/cubensis-rpc-client";
export const PLUGIN_NAME: string = "cubensis-vs-code";

export class PluginConfiguration {
  host: string;
  port: number;
  constructor() {
    this.host =
      vscode.workspace
        .getConfiguration(PLUGIN_NAME)
        .get<string | undefined>("host") ?? "127.0.0.1";
    this.port =
      vscode.workspace
        .getConfiguration(PLUGIN_NAME)
        .get<number | undefined>("port") ?? 3751;
  }
  getAddress(): string {
    return `${this.host}:${this.port}`;
  }

  createClient(): CubensisRpcClient {
    return new CubensisRpcClient(this);
  }
}
