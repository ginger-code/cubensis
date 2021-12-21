import { PluginActionBase } from "./plugin-action-base";
import * as vscode from "vscode";
import {
  PluginConfiguration,
  PLUGIN_NAME,
} from "../configuration/plugin-configuration";
import { CubensisRpcClient } from "../client/cubensis-rpc-client";

export class SetProjectWithHotReloadAction extends PluginActionBase {
  createCommand(): vscode.Disposable {
    return vscode.commands.registerCommand(
      `${PLUGIN_NAME}.${this.name}`,
      (path) => this.execute(path),
      this
    );
  }
  execute(
    path: string | undefined = vscode.window.activeTextEditor?.document.uri
      .fsPath
  ): void {
    if (path !== undefined) {
      this.client.setShaderProject(path, true);
    }
  }
  constructor(configuration: PluginConfiguration, client: CubensisRpcClient) {
    super("SetProjectWithHotReload", configuration, client);
  }
}
