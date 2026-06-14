import * as vscode from 'vscode';
import path from 'node:path';

import {
	Executable,
	LanguageClient,
	LanguageClientOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;
let extensionPath: string;

let log = vscode.window.createOutputChannel('Blase');

async function startClient() {
	console.debug('Blase started');
	const execPath = path.join(extensionPath, 'dist', 'blase');

	log.appendLine(`Blase executable path: ${execPath}`);

	const serverExecutable: Executable = {
		command: execPath,
	};

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: 'file', language: 'blade' }]
	};

	client = new LanguageClient('blase', 'Blase', serverExecutable, clientOptions);

	await client.start();
}

export async function activate(context: vscode.ExtensionContext) {
	log.appendLine('Blase activation event');

        extensionPath = context.extensionPath;

	for (const [command, handler] of Object.entries(commands())) {
		context.subscriptions.push(vscode.commands.registerCommand(command, handler));
	}

	await startClient();
}

async function restartClient() {
	log.appendLine('Restarting language client');

	deactivate();

	await startClient();
}

function commands(): Record<string, () => Promise<void>> {
	return {
		'blase.restartServer': restartClient,
	}
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
