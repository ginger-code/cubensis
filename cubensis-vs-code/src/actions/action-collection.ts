import { ExtensionContext } from "vscode";
import { CubensisRpcClient } from "../client/cubensis-rpc-client";
import { PluginConfiguration } from "../configuration/plugin-configuration";
import { PluginActionBase } from "./plugin-action-base";
import { SetProjectWithHotReloadAction } from "./set-project-action";

export class ActionCollection {
  client: CubensisRpcClient;
  actions: PluginActionBase[];

  registerDisposals(context: ExtensionContext): void {
    this.actions.forEach((action) => {
      context.subscriptions.push(action.command);
    });
  }

  constructor(configuration: PluginConfiguration) {
    this.client = configuration.createClient();
    this.actions = [
      new SetProjectWithHotReloadAction(configuration, this.client),
    ];
  }
}
