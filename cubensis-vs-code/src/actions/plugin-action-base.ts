import * as vscode from "vscode";
import { CubensisRpcClient } from "../client/cubensis-rpc-client";
import { PluginConfiguration } from "../configuration/plugin-configuration";

export abstract class PluginActionBase {
  name: string;
  configuration: PluginConfiguration;
  command: vscode.Disposable;
  client: CubensisRpcClient;
  abstract createCommand(): vscode.Disposable;
  dispose(): Function {
    return () => this.command?.dispose();
  }
  constructor(
    name: string,
    configuration: PluginConfiguration,
    client: CubensisRpcClient
  ) {
    this.name = name;
    this.configuration = configuration;
    this.command = this.createCommand();
    this.client = client;
  }
}
