# MDZ - The New File Format for Self-Contained Markdown Documents 🚀

[![Crates.io](https://img.shields.io/crates/v/mdz-rs.svg)](https://crates.io/crates/mdz-rs)
[![Crates.io](https://img.shields.io/crates/v/mdz.svg)](https://crates.io/crates/mdz)
[![VS Code Marketplace](https://img.shields.io/badge/VS%20Code-Marketplace-blue)](https://marketplace.visualstudio.com/items?itemName=wflixu.mdz)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**MDZ** (Markdown Document Zip) is a revolutionary new file format designed specifically for **sharing Markdown documents with embedded assets**. Tired of broken images, missing files, and complex folder structures when sharing your Markdown content? MDZ solves the fundamental challenge of modern content distribution: creating portable, self-contained documents that work perfectly everywhere.

## 🎯 The Problem MDZ Solves

**Traditional Markdown Sharing Problems:**
- ❌ Broken image links when documents are moved or shared
- ❌ Complex folder structures that must be maintained
- ❌ Network dependencies that make offline viewing impossible
- ❌ Asset management headaches for content creators
- ❌ Distribution requires multiple files and careful organization

**MDZ Solution:**
- ✅ **Single File, Complete Story** - Everything embedded in one .mdz file
- ✅ **Universal Portability** - Works anywhere, no dependencies required
- ✅ **Smart Asset Processing** - Automatically handles both local and network resources
- ✅ **Professional Distribution** - Share like a PDF, but with Markdown's flexibility
- ✅ **Future-Proof Content** - Self-contained files that never break over time

## 🌟 Why MDZ is Different

MDZ isn't just another compression tool - it's a **thoughtfully designed file format** that understands how people actually create and share content:

### 📚 **For Content Creators**
- **Bloggers**: Share posts with embedded images without hosting hassles
- **Technical Writers**: Distribute documentation with diagrams and examples
- **Authors**: Send book chapters with media to editors and publishers
- **Educators**: Create lesson plans with embedded resources
- **Developers**: Share READMEs with screenshots and demos

### 🛠️ **For Tool Developers**
- **Standardized Format**: Open specification for consistent implementation
- **Rich Ecosystem**: Rust library, CLI tool, and VS Code extension
- **Extensible Design**: Ready for future enhancements and integrations
- **Cross-Platform**: Works on Windows, macOS, and Linux

## ✨ Features

### 📚 **Core Functionality**
- **🗜️ MDZ Format**: Custom ZIP-based format for bundling Markdown with images, videos, audio, and other files
- **🔧 Rust Library**: High-performance core library ([`mdz-rs`](mdz-rs/)) with async image downloading and smart asset handling
- **💻 CLI Tool**: Full-featured command-line utility ([`mdz`](mdz/)) for packing and unpacking MDZ files
- **⚡ VS Code Extension**: First-class editor integration with export/unpack capabilities ([`mdz-vscode`](mdz-vscode/))

### 🎯 **Smart Features**
- **🌐 Network Images**: Automatically downloads remote images and saves with UUID filenames
- **📁 Local Files**: Intelligent copying of local assets with conflict resolution
- **🔗 Relative Links**: Uses relative paths (`./assets/...`) for maximum compatibility
- **⏪ Backward Compatibility**: Handles legacy MDZ files without filename field
- **📊 Metadata**: Complete manifest.json with asset mapping and document metadata

## 🏗️ Project Structure

```
.
├── LICENSE
├── README.md
├── CLAUDE.md
├── mdz-spec.md              # 📋 MDZ format specification
├── mdz/                      # 💻 CLI tool crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs           # CLI interface
│       └── tests.rs          # CLI tests
├── mdz-rs/                   # 🔧 Core library crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # Main library implementation
│       └── tests.rs          # Unit tests
├── mdz-vscode/               # ⚡ VS Code extension
│   ├── package.json
│   ├── src/
│   │   ├── extension.ts      # Extension entry point
│   │   ├── commands/
│   │   │   ├── export.ts     # Export functionality
│   │   │   └── unpack.ts     # Unpack functionality
│   │   └── utils/
│   │       ├── mdzCli.ts     # CLI integration
│   │       └── fileUtils.ts  # File utilities
│   └── out/                  # Compiled JavaScript
└── tests/                    # 🧪 Test files and demos
    ├── integration_test.rs
    └── demo/
```

### 📦 **Components**
- **[`mdz-rs/`](mdz-rs/)**: Core Rust library ([lib.rs](mdz-rs/src/lib.rs)) - 787 lines
- **[`mdz/`](mdz/)**: CLI tool ([main.rs](mdz/src/main.rs)) - 191 lines
- **[`mdz-vscode/`](mdz-vscode/)**: VS Code extension - 448 lines TypeScript
- **[mdz-spec.md](mdz-spec.md)**: Complete format specification and usage guide

## 💡 Real-World Use Cases

### 📝 **Technical Documentation**
```markdown
<!-- Before: Multiple files to manage -->
documentation/
├── README.md
├── images/
│   ├── architecture.png
│   ├── workflow.png
│   └── deployment.png
└── diagrams/
    ├── sequence.uml
    └── network.graphml

<!-- After: Single file to share -->
documentation.mdz  ← Contains everything!
```

### 🎨 **Blog Post with Rich Media**
```markdown
# My Travel Blog Post

![Local photo](./photos/destination.jpg)
![Map from service](https://maps.example.com/location)
![Embedded video](./videos/tour.mp4)

<!-- MDZ automatically: -->
<!-- • Downloads map image and saves locally -->
<!-- • Bundles all media in the file -->
<!-- • Updates all links to work offline -->
```

### 📚 **Educational Materials**
```markdown
# Programming Tutorial

## Exercise 1
![Screenshot 1](./screenshots/step1.png)

## Video Tutorial
![Tutorial video](./videos/exercise1.mp4)

## Source Code
[Download examples](./code/examples.zip)
```

### 🔄 **Before vs After MDZ**

| Scenario | Before MDZ | After MDZ |
|----------|------------|-----------|
| **Sharing with colleague** | Send zip file + instructions | Send single .mdz file |
| **Uploading to platform** | Multiple files + manual linking | Upload single .mdz file |
| **Backup content** | Track folder + assets separately | One .mdz file contains everything |
| **Offline viewing** | Broken network images | All images work offline |
| **Version control** | Large repos with media assets | Small repos with single files |

### 🆚 **MDZ vs Other Solutions**

| Solution | ✅ Advantages | ❌ Limitations |
|----------|----------------|----------------|
| **MDZ** | Single file, smart processing, offline capable | Requires MDZ-aware tools |
| **ZIP folders** | Universal compatibility | Manual asset management |
| **Git repos** | Version control | Complex setup, requires internet |
| **Static sites** | Beautiful presentation | Hosting dependency, maintenance |
| **PDF export** | Universal viewing | Lost Markdown editing capability |

## 🚀 Installation

### 📦 **From Crates.io**
```sh
# Install CLI tool
cargo install mdz

# Library usage in Cargo.toml
mdz-rs = "1.1.0"
```

### 🔨 **From Source**
```sh
# Clone and build
git clone https://github.com/wflixu/mdz.git
cd mdz
cargo install --path .

# Build library only
cargo build --release -p mdz-rs
```

### ⚡ **VS Code Extension**
```sh
# Install from marketplace
code --install-extension wflixu.mdz

# Or install from VSIX
code --install-extension mdz-vscode/mdz-1.1.0.vsix
```

## 📖 Usage

### 💻 **CLI Tool**
```sh
# Export markdown to MDZ
mdz pack document.md -o document.mdz

# Export with custom name
mdz pack document.md -o my-doc.mdz

# Unpack MDZ to current directory
mdz unpack document.mdz

# Unpack to specific directory
mdz unpack document.mdz -o output/
```

### 🏢 **Rust Library**
```rust
use mdz_rs::{pack, unpack};

// Async pack function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pack("document.md", "document.mdz").await?;
    unpack("document.mdz", Some("output/"))?;
    Ok(())
}
```

### ⚡ **VS Code Extension**
- **Export**: Right-click `.md` file → "Export as MDZ"
- **Unpack**: Right-click `.mdz` file → "Unpack MDZ"
- **Quick Actions**: Export/unpack buttons in editor title bar
- **Commands**: `Ctrl+Shift+P` → "MDZ: Export as MDZ"

## 📄 MDZ File Format

### 🏗️ **Structure**
```
document.mdz (ZIP archive)
├── index.md              # Updated markdown with relative asset links
├── manifest.json         # Metadata and asset mapping
└── assets/               # Organized asset files
    ├── images/
    ├── videos/
    ├── audio/
    └── files/
```

### 🎯 **Key Features**
- ✅ **Standard ZIP**: Can be extracted with any ZIP tool
- ✅ **Relative Links**: Uses `./assets/...` paths for maximum compatibility
- ✅ **Smart Asset Handling**: Auto-download network images, copy local files
- ✅ **UUID Filenames**: Network images use UUID names to avoid conflicts
- ✅ **Metadata Support**: Complete manifest with document and asset information
- ✅ **Backward Compatible**: Handles legacy files (v1.0.0) seamlessly

### 📋 **Specification**
See [mdz-spec.md](mdz-spec.md) for complete format specification and implementation details.

## 🧪 Development

### 🏗️ **Build & Test**
```sh
# Build all components
cargo build

# Run tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Integration tests
cargo test --test integration_test
```

### 🔧 **VS Code Extension Development**
```sh
cd mdz-vscode
pnpm install
pnpm run compile
pnpm exec vsce package
```

### 📊 **Project Stats** (v1.1.0)
- **Core Code**: 1,426 lines (Rust + TypeScript)
- **Tests**: 491 lines (unit + integration tests)
- **Languages**: Rust (1k+ lines), TypeScript (448 lines)
- **Components**: Core library, CLI tool, VS Code extension

## 🤝 Contributing

Contributions are welcome! Please:

1. 🐛 **Bug Reports**: Open issues with detailed description
2. 💡 **Feature Requests**: Suggest improvements with use cases
3. 🔧 **Pull Requests**: Ensure tests pass and code follows project style
4. 📚 **Documentation**: Help improve docs and examples

### 🛠️ **Development Setup**
```sh
git clone https://github.com/wflixu/mdz.git
cd mdz
cargo build
cargo test  # Verify everything works
```

## 📜 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- Built with [Rust](https://rust-lang.org) for performance and safety
- VS Code extension powered by [TypeScript](https://www.typescriptlang.org/)
- Inspired by the need for self-contained Markdown document sharing
- Community feedback and contributions for continuous improvement