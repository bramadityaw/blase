import * as vscode from 'vscode';

import {
	Executable,
	LanguageClient,
	LanguageClientOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;
let log = vscode.window.createOutputChannel('Blase');

async function startClient() {
	console.debug('Blase started');
	const config = vscode.workspace.getConfiguration("blase");
	log.appendLine(`Client configuration for blase: ${config}`);

	let execString: string = config.get("executablePath") || "blase";

	const [execPath, ...execArgs] = execString.trim().split(/\s+/);
	log.appendLine(`Blase executable path: ${execPath}`);
	log.appendLine(`Blase command line args: ${execArgs}`);

	const serverExecutable: Executable = {
		command: execPath,
		args: execArgs,
	};

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: 'file', language: 'blade' }]
	};

	client = new LanguageClient('blase', 'Blase', serverExecutable, clientOptions);

	await client.start();
}

export async function activate(context: vscode.ExtensionContext) {
	log.appendLine('Blase activation event');

	vscode.workspace.onDidChangeConfiguration((event) => {
		if (event.affectsConfiguration('blase.executablePath')) {
			restartClient();
		}
	});

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