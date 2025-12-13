import * as vscode from 'vscode';
import { executeMdzCommand, showProgress } from '../utils/mdzCli';
import {
    getFileUri,
    refreshFileExplorer,
    isMdzFile,
    openFileInEditor
} from '../utils/fileUtils';
import * as path from 'path';

export async function unpackMdzFile(uri?: vscode.Uri) {
    const fileUri = getFileUri(uri);

    if (!fileUri) {
        vscode.window.showErrorMessage('No MDZ file is currently open or selected');
        return;
    }

    const filePath = fileUri.fsPath;

    if (!isMdzFile(filePath)) {
        vscode.window.showErrorMessage('Only MDZ files (.mdz) can be unpacked');
        return;
    }

    // Confirm before unpacking
    const fileName = path.basename(filePath);
    const result = await vscode.window.showInformationMessage(
        `Unpack "${fileName}" to the current directory?`,
        { modal: true },
        'Unpack',
        'Cancel'
    );

    if (result !== 'Unpack') {
        return;
    }

    await showProgress(
        'Unpacking MDZ file...',
        async (progress) => {
            progress.report({ increment: 10, message: 'Preparing unpack...' });

            const dirPath = path.dirname(filePath);
            progress.report({ increment: 30, message: `Unpacking ${fileName}...` });

            try {
                const command = `unpack "${filePath}"`;
                await executeMdzCommand(command, dirPath);

                progress.report({ increment: 80, message: 'Finalizing...' });

                // Refresh the file explorer to show extracted files
                refreshFileExplorer();

                progress.report({ increment: 100, message: 'Complete!' });

                vscode.window.showInformationMessage(
                    `Successfully unpacked ${fileName}`,
                    'Show Extracted Files'
                ).then(async (selection) => {
                    if (selection === 'Show Extracted Files') {
                        // Focus on the file explorer
                        vscode.commands.executeCommand('workbench.view.explorer');

                        // Try to find and open the extracted markdown file
                        const extractedMarkdownPath = path.join(dirPath, path.basename(filePath, '.mdz'));
                        if (await fileExists(extractedMarkdownPath)) {
                            await openFileInEditor(extractedMarkdownPath);
                        }
                    }
                });

            } catch (error: any) {
                const errorMessage = error?.message || 'Unknown error occurred';
                vscode.window.showErrorMessage(`Failed to unpack MDZ file: ${errorMessage}`);
                throw error;
            }
        }
    );
}

// Helper function to check if file exists (Node.js fs version)
async function fileExists(filePath: string): Promise<boolean> {
    const fs = require('fs').promises;
    try {
        await fs.access(filePath);
        return true;
    } catch {
        return false;
    }
}