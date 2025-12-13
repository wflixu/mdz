import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export async function checkMdzCli(): Promise<boolean> {
    try {
        const config = vscode.workspace.getConfiguration('mdz');
        const customPath = config.get<string>('cliPath');

        const command = customPath || 'mdz';
        await execAsync(`${command} --version`);
        return true;
    } catch (error) {
        return false;
    }
}

export function getMdzCliPath(): string {
    const config = vscode.workspace.getConfiguration('mdz');
    return config.get<string>('cliPath') || 'mdz';
}

export async function executeMdzCommand(command: string, cwd?: string): Promise<{ stdout: string; stderr: string }> {
    return new Promise((resolve, reject) => {
        const config = vscode.workspace.getConfiguration('mdz');
        const autoShowOutput = config.get<boolean>('autoShowOutput', true);
        const mdzPath = getMdzCliPath();

        const fullCommand = `${mdzPath} ${command}`;
        const outputChannel = vscode.window.createOutputChannel('MDZ');

        if (autoShowOutput) {
            outputChannel.show();
        }

        outputChannel.appendLine(`> ${fullCommand}`);

        exec(fullCommand, { cwd }, (error, stdout, stderr) => {
            if (stdout) {
                outputChannel.appendLine(stdout);
            }

            if (stderr) {
                outputChannel.appendLine(`Error: ${stderr}`);
            }

            if (error) {
                outputChannel.appendLine(`Command failed with code: ${error.code}`);
                reject(error);
            } else {
                outputChannel.appendLine('Command completed successfully');
                resolve({ stdout, stderr });
            }
        });
    });
}

export function showProgress<T>(
    title: string,
    task: (progress: vscode.Progress<{ message?: string; increment?: number }>) => Promise<T>
): Thenable<T> {
    return vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title,
            cancellable: false
        },
        task
    );
}