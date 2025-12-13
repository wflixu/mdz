# Change Log

All notable changes to the "mdz" extension will be documented in this file.

## [1.1.0] - 2024-12-13

### Fixed

- Add backward compatibility for MDZ files missing `filename` field in manifest.json
- Fix unpacking of older MDZ files created with version 1.0.0
- Update Manifest structure to handle optional filename field

## [1.0.0] - 2024-12-13

### Features

- Export markdown files (.md) to MDZ format with a single click
- Unpack MDZ files back to markdown with organized assets
- Editor title bar integration with quick action buttons
- Context menu integration for right-click operations
- Command palette support with intelligent filtering
- Progress tracking for long-running operations
- Output panel for detailed command execution logs
- Customizable settings for CLI path and behavior
- File overwrite confirmation dialogs
- Automatic file explorer refresh after operations
- Smart filename suggestions for exports

### Requirements

- Requires MDZ CLI tool to be installed (`cargo install mdz`)
- VS Code 1.84.0 or higher

### Configuration Options

- `mdz.cliPath`: Custom path to MDZ CLI executable
- `mdz.autoShowOutput`: Toggle automatic output panel display
- `mdz.confirmOverwrite`: Toggle overwrite confirmation dialogs

### Commands Added

- `mdz.exportFile`: Export current markdown file as MDZ
- `mdz.exportAs`: Export markdown with custom filename
- `mdz.unpackFile`: Unpack MDZ file to current directory

### Menu Integration

- Editor title: Export/Unpack buttons for supported file types
- Explorer context: Right-click options for .md and .mdz files
- Command palette: Intelligent filtering based on active file type