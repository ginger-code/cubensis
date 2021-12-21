import { WebSocket } from "ws";
import { PluginConfiguration } from "../configuration/plugin-configuration";
import * as vscode from "vscode";

import { RpcResponse, SetProjectRequest } from "./cubensis-rpc-types";

export class CubensisRpcClient {
  ws: WebSocket;
  constructor(configuration: PluginConfiguration) {
    this.ws = new WebSocket(`ws://${configuration.getAddress()}/socket`, {
      port: configuration.port,
    });
    this.ws.on("open", function open() {
      vscode.window.showInformationMessage(
        "Connection to Cubensis established"
      );
    });
    this.ws.on("close", function close() {
      vscode.window.showWarningMessage("Connection to Cubensis lost");
    });
    this.ws.on("message", function message(data) {
      let response = RpcResponse.parse(data.toString());
      response.display();
    });
  }
  setShaderProject(projectPath: string, enableHotReload: boolean) {
    if (this.ws.OPEN) {
      let request = new SetProjectRequest(projectPath, enableHotReload);
      this.ws.send(request.serialize());
    }
  }
}
