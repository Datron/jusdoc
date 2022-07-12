import * as vscode from 'vscode';
import { readFileSync } from 'fs';
import { CancellationToken, Hover, Position, ProviderResult, TextDocument } from 'vscode';

export let initJusdocFile = () => {
    const workspacePath = getWorkspacePath();
    if (workspacePath === undefined) { return; }
    const jusDocUri = getJusdocUri(workspacePath);
    const createEdit = new vscode.WorkspaceEdit();
    createEdit.createFile(jusDocUri, { ignoreIfExists: true });
    vscode.workspace.applyEdit(createEdit);
    vscode.window.showInformationMessage('created a new Jusdoc file!');
};

export let addJusdocEntry = () => {
    vscode.window.showInformationMessage('jusdoc it');
};

export let hoverHandler = (document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Hover> => {
    const wordRange = document.getWordRangeAtPosition(position);
    const word = document.getText(wordRange);
    const workspacePath = getWorkspacePath(true);
    const workspaceUriPath = getWorkspacePath();
    if (workspacePath === undefined || workspaceUriPath === undefined) { return; }
    const workspaceRoot = getWorkspaceRootName(workspacePath);
    if (workspaceRoot === undefined) { return; }
    const jusdocKey = getJusdocKey(workspaceRoot, document.uri.path);
    const jusdocData = JSON.parse(readFileSync(workspaceUriPath + '/.jusdoc.json', 'utf8'));
    console.log("data:", jusdocData[jusdocKey][word].summary);
    return {
        contents: [jusdocData[jusdocKey][word].summary]
    };
};


let getWorkspacePath = (path?: boolean): string | undefined => {
    if (vscode.workspace.workspaceFolders === undefined) {
        vscode.window.showErrorMessage("no workspace found to initiate jusdoc, please open a folder");
        return;
    }
    return path ?
        vscode.workspace.workspaceFolders[0].uri.path
        : vscode.workspace.workspaceFolders[0].uri.fsPath;
};

let getWorkspaceRootName = (workspace: string): string | undefined => {
    return workspace.split("/").pop();
};

let getJusdocUri = (workspacePath: string) => vscode.Uri.file(workspacePath + '/.jusdoc.json');

let getJusdocKey = (workspacePath: string, filePath: string) => filePath.substring(filePath.indexOf(workspacePath), filePath.length);