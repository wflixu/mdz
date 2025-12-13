import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

export function getActiveEditor(): vscode.TextEditor | undefined {
    return vscode.window.activeTextEditor;
}

export function getCurrentFilePath(): string | undefined {
    const editor = getActiveEditor();
    return editor?.document.uri.fsPath;
}

export function getFileUri(uri?: vscode.Uri): vscode.Uri | undefined {
    if (uri) {
        return uri;
    }

    const currentFile = getCurrentFilePath();
    return currentFile ? vscode.Uri.file(currentFile) : undefined;
}

export function getOutputPath(inputPath: string, customName?: string): string {
    const parsedPath = path.parse(inputPath);

    if (customName) {
        const customParsed = path.parse(customName);
        if (customParsed.ext) {
            // User provided full filename
            return path.join(parsedPath.dir, customName);
        } else {
            // User provided name without extension
            return path.join(parsedPath.dir, `${customName}.mdz`);
        }
    }

    // Default: same name with .mdz extension
    return path.join(parsedPath.dir, `${parsedPath.name}.mdz`);
}

export function fileExists(filePath: string): boolean {
    try {
        return fs.existsSync(filePath);
    } catch {
        return false;
    }
}

export function isMarkdownFile(filePath: string): boolean {
    return filePath.toLowerCase().endsWith('.md');
}

export function isMdzFile(filePath: string): boolean {
    return filePath.toLowerCase().endsWith('.mdz');
}

export async function confirmOverwrite(filePath: string): Promise<boolean> {
    if (!fileExists(filePath)) {
        return true;
    }

    const config = vscode.workspace.getConfiguration('mdz');
    const confirmOverwrite = config.get<boolean>('confirmOverwrite', true);

    if (!confirmOverwrite) {
        return true;
    }

    const fileName = path.basename(filePath);
    const result = await vscode.window.showWarningMessage(
        `File "${fileName}" already exists. Do you want to overwrite it?`,
        { modal: true },
        'Overwrite',
        'Cancel'
    );

    return result === 'Overwrite';
}

export function refreshFileExplorer(): void {
    vscode.commands.executeCommand('workbench.files.action.refreshFilesExplorer');
}

export function openFileInEditor(filePath: string): Thenable<vscode.TextEditor> {
    const uri = vscode.Uri.file(filePath);
    return vscode.window.showTextDocument(uri);
}