# MDZ - Markdown Zip VS Code Extension

[![Version](https://vsmarketplacebadges.dev/version-short/wflixu.mdz.svg)](https://marketplace.visualstudio.com/items?itemName=wflixu.mdz)
[![Installs](https://vsmarketplacebadges.dev/installs-short/wflixu.mdz.svg)](https://marketplace.visualstudio.com/items?itemName=wflixu.mdz)
[![Rating](https://vsmarketplacebadges.dev/rating-short/wflixu.mdz.svg)](https://marketplace.visualstudio.com/items?itemName=wflixu.mdz)

A Visual Studio Code extension for working with [MDZ (Markdown Zip)](https://github.com/wflixu/mdz) files. Export your markdown documents with embedded images and assets to a single MDZ file, and unpack MDZ archives back to markdown with assets.

## Features

- 📦 **Export Markdown to MDZ**: Convert your markdown files with images and attachments to a single MDZ archive
- 📂 **Unpack MDZ Files**: Extract MDZ archives back to markdown with organized asset folders
- 🎯 **Context Menu Integration**: Right-click on files to export/unpack directly from the explorer
- 🚀 **Editor Integration**: Quick actions in the editor title bar for easy access
- ⚡ **Progress Tracking**: Real-time progress indicators for long operations
- 🔧 **Customizable Settings**: Configure paths and behavior to fit your workflow

## Requirements

This extension requires the [MDZ CLI tool](https://github.com/wflixu/mdz) to be installed:

```bash
cargo install mdz
```

## Installation

1. Install the [MDZ CLI tool](https://github.com/wflixu/mdz)
2. Install this extension from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=wflixu.mdz)
3. Restart VS Code

## Usage

### Export Markdown to MDZ

**Method 1: Editor Title Bar**
- Open a markdown file (`.md`)
- Click the export icon in the editor title bar

**Method 2: Command Palette**
- Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
- Type "Export as MDZ" and select the command
- Choose either quick export or custom name

**Method 3: Right-Click Menu**
- Right-click on a markdown file in the explorer
- Select "Export as MDZ"

### Unpack MDZ Files

**Method 1: Editor Title Bar**
- Open an MDZ file (`.mdz`)
- Click the unpack icon in the editor title bar

**Method 2: Command Palette**
- Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
- Type "Unpack MDZ" and select the command

**Method 3: Right-Click Menu**
- Right-click on an MDZ file in the explorer
- Select "Unpack MDZ"

## Settings

You can configure the extension through VS Code settings:

- `mdz.cliPath`: Custom path to the MDZ CLI executable (if not in system PATH)
- `mdz.autoShowOutput`: Automatically show the output panel when executing commands (default: true)
- `mdz.confirmOverwrite`: Show confirmation dialog before overwriting existing files (default: true)

### Example Configuration

```json
{
  "mdz.cliPath": "/usr/local/bin/mdz",
  "mdz.autoShowOutput": true,
  "mdz.confirmOverwrite": true
}
```

## File Structure

When you export a markdown file to MDZ:

```
document.mdz
├── index.md           # Your markdown content with updated relative paths
├── manifest.json      # Metadata and asset mapping
└── assets/            # Embedded assets
    ├── images/
    ├── videos/
    ├── audio/
    └── files/
```

When you unpack an MDZ file, it restores:
- The original markdown file
- The assets folder with all embedded media
- Proper relative links between them

## About MDZ Format

MDZ (Markdown Zip) is a format for bundling markdown documents with their embedded resources. It solves the problem of sharing standalone markdown files that include images, videos, and other assets.

Key benefits:
- ✅ Self-contained documents
- ✅ Preserved relative paths
- ✅ Compatible with standard ZIP tools
- ✅ Organized asset structure
- ✅ Metadata support

## Issues and Feedback

If you encounter any issues or have suggestions, please:
- Check the [Issues page](https://github.com/wflixu/mdz/issues)
- Create a new issue with details about your problem
- Include your OS, VS Code version, and steps to reproduce

## Contributing

Contributions are welcome! Please see the [main project repository](https://github.com/wflixu/mdz) for more information.

## License

This extension is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.