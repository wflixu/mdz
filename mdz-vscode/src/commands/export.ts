import * as vscode from 'vscode';
import { executeMdzCommand, showProgress } from '../utils/mdzCli';
import {
    getFileUri,
    getOutputPath,
    confirmOverwrite,
    refreshFileExplorer,
    isMarkdownFile,
    fileExists
} from '../utils/fileUtils';
import * as path from 'path';

export async function exportMarkdownFile(uri?: vscode.Uri) {
    const fileUri = getFileUri(uri);

    if (!fileUri) {
        vscode.window.showErrorMessage('No markdown file is currently open');
        return;
    }

    const filePath = fileUri.fsPath;

    if (!isMarkdownFile(filePath)) {
        vscode.window.showErrorMessage('Only markdown files (.md) can be exported to MDZ format');
        return;
    }

    const outputPath = getOutputPath(filePath);

    if (!await confirmOverwrite(outputPath)) {
        return;
    }

    await showProgress(
        'Exporting to MDZ...',
        async (progress) => {
            progress.report({ increment: 10, message: 'Preparing export...' });

            const fileName = path.basename(outputPath);
            progress.report({ increment: 30, message: `Exporting to ${fileName}...` });

            try {
                const command = `pack "${filePath}" -o "${outputPath}"`;
                await executeMdzCommand(command, path.dirname(filePath));

                progress.report({ increment: 80, message: 'Finalizing...' });

                // Refresh the file explorer to show the new file
                refreshFileExplorer();

                progress.report({ increment: 100, message: 'Complete!' });

                vscode.window.showInformationMessage(
                    `Successfully exported to ${fileName}`,
                    'Open File',
                    'Show in Explorer'
                ).then(async (selection) => {
                    if (selection === 'Open File') {
                        // Open the MDZ file in the default application
                        vscode.commands.executeCommand('vscode.open', vscode.Uri.file(outputPath));
                    } else if (selection === 'Show in Explorer') {
                        vscode.commands.executeCommand('revealFileInOS', vscode.Uri.file(outputPath));
                    }
                });

            } catch (error: any) {
                const errorMessage = error?.message || 'Unknown error occurred';
                vscode.window.showErrorMessage(`Failed to export markdown: ${errorMessage}`);
                throw error;
            }
        }
    );
}

export async function exportMarkdownAs(uri?: vscode.Uri) {
    const fileUri = getFileUri(uri);

    if (!fileUri) {
        vscode.window.showErrorMessage('No markdown file is currently open');
        return;
    }

    const filePath = fileUri.fsPath;

    if (!isMarkdownFile(filePath)) {
        vscode.window.showErrorMessage('Only markdown files (.md) can be exported to MDZ format');
        return;
    }

    const defaultName = path.basename(filePath, path.extname(filePath)) + '.mdz';

    const outputName = await vscode.window.showInputBox({
        prompt: 'Enter output filename for MDZ file',
        value: defaultName,
        validateInput: (value) => {
            if (!value.trim()) {
                return 'Filename cannot be empty';
            }

            if (!value.toLowerCase().endsWith('.mdz')) {
                return 'Filename must end with .mdz';
            }

            return null;
        }
    });

    if (!outputName) {
        return;
    }

    const outputPath = getOutputPath(filePath, outputName);

    if (!await confirmOverwrite(outputPath)) {
        return;
    }

    await showProgress(
        'Exporting to MDZ...',
        async (progress) => {
            progress.report({ increment: 10, message: 'Preparing export...' });

            const fileName = path.basename(outputPath);
            progress.report({ increment: 30, message: `Exporting to ${fileName}...` });

            try {
                const command = `pack "${filePath}" -o "${outputPath}"`;
                await executeMdzCommand(command, path.dirname(filePath));

                progress.report({ increment: 80, message: 'Finalizing...' });

                // Refresh the file explorer to show the new file
                refreshFileExplorer();

                progress.report({ increment: 100, message: 'Complete!' });

                vscode.window.showInformationMessage(
                    `Successfully exported to ${fileName}`,
                    'Open File',
                    'Show in Explorer'
                ).then(async (selection) => {
                    if (selection === 'Open File') {
                        // Open the MDZ file in the default application
                        vscode.commands.executeCommand('vscode.open', vscode.Uri.file(outputPath));
                    } else if (selection === 'Show in Explorer') {
                        vscode.commands.executeCommand('revealFileInOS', vscode.Uri.file(outputPath));
                    }
                });

            } catch (error: any) {
                const errorMessage = error?.message || 'Unknown error occurred';
                vscode.window.showErrorMessage(`Failed to export markdown: ${errorMessage}`);
                throw error;
            }
        }
    );
}