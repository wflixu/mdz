import * as vscode from 'vscode';
import { exportMarkdownFile, exportMarkdownAs } from './commands/export';
import { unpackMdzFile } from './commands/unpack';
import { checkMdzCli, getMdzCliPath } from './utils/mdzCli';

export function activate(context: vscode.ExtensionContext) {
    console.log('MDZ extension is now active!');

    // Check if mdz CLI is available
    checkMdzCli().then((isAvailable) => {
        if (!isAvailable) {
            vscode.window.showWarningMessage(
                'MDZ CLI not found. Please install mdz CLI tool: cargo install mdz',
                'Install Guide'
            ).then(selection => {
                if (selection === 'Install Guide') {
                    vscode.env.openExternal(vscode.Uri.parse('https://github.com/wflixu/mdz'));
                }
            });
        } else {
            vscode.window.showInformationMessage('MDZ CLI detected and ready to use!');
        }
    });

    // Register export command
    const exportCommand = vscode.commands.registerCommand('mdz.exportFile', (uri?: vscode.Uri) => {
        exportMarkdownFile(uri);
    });

    // Register export as command
    const exportAsCommand = vscode.commands.registerCommand('mdz.exportAs', (uri?: vscode.Uri) => {
        exportMarkdownAs(uri);
    });

    // Register unpack command
    const unpackCommand = vscode.commands.registerCommand('mdz.unpackFile', (uri?: vscode.Uri) => {
        unpackMdzFile(uri);
    });

    context.subscriptions.push(exportCommand, exportAsCommand, unpackCommand);
}

export function deactivate() {
    console.log('MDZ extension deactivated');
}