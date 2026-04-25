import * as path from "path";
import { workspace, ExtensionContext } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(context: ExtensionContext) {
  const config = workspace.getConfiguration("lm");
  const serverPath = config.get<string>("serverPath", "lmc");

  const serverOptions: ServerOptions = {
    command: serverPath,
    args: ["lsp"],
    transport: TransportKind.stdio,
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "lm" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/*.lm"),
    },
  };

  client = new LanguageClient(
    "lm-lsp",
    "LM Language Server",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
