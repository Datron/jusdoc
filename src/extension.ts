import * as vscode from 'vscode';
import { addJusdocEntry, hoverHandler, initJusdocFile } from './commands';

export function activate(context: vscode.ExtensionContext) {
	console.log('Congratulations, your extension "jusdoc" is now active!');
	let create = vscode.commands.registerCommand('jusdoc.createJusdoc', initJusdocFile);
	let jusdoc = vscode.commands.registerCommand('jusdoc.jusDoc', addJusdocEntry);

	vscode.languages.registerHoverProvider({'scheme': 'file'}, {
		provideHover: hoverHandler
	});

	context.subscriptions.push(create, jusdoc);
}

export function deactivate() {}
