import * as vscode from "vscode";

export abstract class RpcRequest {
  abstract requestKind(): string;
  serialize(): string {
    return `{"${this.requestKind()}":${JSON.stringify(this)}}`;
  }
}

export class SetProjectRequest extends RpcRequest {
  public project_path: string;
  public enable_hot_reload: boolean;
  constructor(projectPath: string, enableHotReload: boolean) {
    super();
    this.project_path = projectPath;
    this.enable_hot_reload = enableHotReload;
  }
  requestKind(): string {
    return "SetProject";
  }
}

export enum RpcSeverity {
  None = 0,
  Info = 1,
  Warning = 2,
  Error = 3,
}

export class RpcResponse {
  public is_error: boolean;
  public severity: RpcSeverity;
  public message: string;
  constructor(is_error: boolean, severity: RpcSeverity, message: string) {
    this.is_error = is_error;
    this.severity = severity;
    this.message = message;
  }
  static parse(json: string): RpcResponse {
    let obj: RpcResponse = JSON.parse(json);
    return obj;
  }
  display(): void {
    let err = this.is_error ? " [ERROR]" : "";
    switch (this.severity) {
      case RpcSeverity.None:
        console.log(`CUBENSIS DEBUG${err}: ${this.message}`);
        break;
      case RpcSeverity.Info:
        console.log(`CUBENSIS INFO${err}: ${this.message}`);
        vscode.window.showInformationMessage(this.message);
        break;
      case RpcSeverity.Warning:
        console.warn(`CUBENSIS WARNING${err}: ${this.message}`);
        vscode.window.showWarningMessage(this.message);
        break;
      case RpcSeverity.Error:
        console.error(`CUBENSIS ERROR${err}: ${this.message}`);
        vscode.window.showErrorMessage(this.message);
        break;
    }
  }
}
