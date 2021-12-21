import * as vscode from "vscode";
import { ActionCollection } from "./actions/action-collection";
import { PluginConfiguration } from "./configuration/plugin-configuration";

export function activate(context: vscode.ExtensionContext) {
  console.log("Cubensis link established");
  let configuration = new PluginConfiguration();
  let actionCollection = new ActionCollection(configuration);
  actionCollection.registerDisposals(context);
}

export function deactivate() {}
