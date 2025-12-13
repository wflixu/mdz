# mdz-rs

[![Crates.io](https://img.shields.io/crates/v/mdz-rs.svg)](https://crates.io/crates/mdz-rs)
[![Documentation](https://docs.rs/mdz-rs/badge.svg)](https://docs.rs/mdz-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful Rust library for creating and working with **MDZ (Markdown Zip)** files - self-contained archives that bundle Markdown documents with embedded assets.

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
mdz-rs = "1.1.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## 📖 Basic Usage

```rust
use mdz_rs::{pack, unpack};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Pack a markdown file with assets
    pack("document.md", "document.mdz").await?;

    // Unpack MDZ file to extract content and assets
    unpack("document.mdz", Some("output/"))?;

    Ok(())
}
```

## ✨ Features

- 🗜️ **Packaging**: Bundle Markdown with images, videos, audio, and other files
- 🌐 **Smart Downloads**: Automatically download network images with UUID filenames
- 📁 **Local Files**: Intelligently copy local assets with conflict resolution
- 🔗 **Relative Links**: Use relative paths for maximum compatibility
- ⏪ **Backward Compatible**: Handle legacy MDZ files seamlessly
- 📊 **Rich Metadata**: Complete manifest with document and asset information

## 🏗️ MDZ Format Structure

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

## 🔧 API Reference

### Core Functions

#### `pack(markdown_file: &str, output_file: &str) -> Result<()>`
Pack a markdown file and its assets into an MDZ archive.

```rust
use mdz_rs::pack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pack("report.md", "report.mdz").await?;
    println!("Successfully packed report.mdz");
    Ok(())
}
```

#### `unpack(input_file: &str, output_dir: Option<&str>) -> Result<()>`
Unpack an MDZ archive to extract the markdown file and assets.

```rust
use mdz_rs::unpack;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unpack("report.mdz", Some("extracted/"))?;
    println!("Successfully unpacked to extracted/");
    Ok(())
}
```

### Utility Functions

#### `is_url(path: &str) -> bool`
Check if a path is a URL or local file path.

```rust
use mdz_rs::is_url;

assert!(is_url("https://example.com/image.jpg"));
assert!(!is_url("./local/image.png"));
```

## 📦 Dependencies

- **zip**: ZIP archive creation and extraction
- **serde + serde_json**: JSON serialization for manifest files
- **reqwest**: HTTP client for downloading network images
- **tokio**: Async runtime for network operations
- **anyhow**: Error handling
- **url**: URL parsing and manipulation
- **regex**: Markdown link processing

## 🎯 Use Cases

- **Document Sharing**: Create self-contained documents with embedded images
- **Static Site Generation**: Package blog posts with assets
- **Documentation Systems**: Bundle documentation with media files
- **Content Management**: Archive markdown content with resources
- **Educational Materials**: Share lessons with embedded media

## 🧪 Examples

### Example 1: Basic Packing

```rust
use mdz_rs::pack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a markdown file
    std::fs::write("example.md", r#"
# My Document

![Local Image](./images/photo.jpg)
![Remote Image](https://example.com/banner.png)

This is a sample document with both local and remote images.
"#)?;

    // Pack it into MDZ
    pack("example.md", "example.mdz").await?;
    println!("Created example.mdz");

    Ok(())
}
```

### Example 2: Custom Unpacking

```rust
use mdz_rs::unpack;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Unpack to specific directory
    unpack("example.mdz", Some("my_docs/"))?;

    // Check extracted files
    let markdown = std::fs::read_to_string("my_docs/example.md")?;
    println!("Extracted markdown: {}", markdown);

    Ok(())
}
```

### Example 3: Asset Processing

```rust
use mdz_rs::pack;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let md_file = "portfolio.md";
    let output_file = "portfolio.mdz";

    // The pack function will:
    // 1. Parse the markdown for image links
    // 2. Download remote images with UUID names
    // 3. Copy local images to assets directory
    // 4. Update links to use relative paths
    // 5. Create manifest.json with metadata
    // 6. Bundle everything into a ZIP file

    pack(md_file, output_file).await?;

    println!("✅ Created self-contained document!");
    println!("📁 Assets are organized in the archive");
    println!("🔗 Links use relative paths for compatibility");

    Ok(())
}
```

## 🔍 Advanced Usage

### Error Handling

```rust
use mdz_rs::{pack, unpack};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    match pack("missing.md", "output.mdz").await {
        Ok(_) => println!("Packing successful"),
        Err(e) => eprintln!("Packing failed: {}", e),
    }

    Ok(())
}
```

### Asset Detection

The library automatically detects and processes:

- **Images**: PNG, JPG, JPEG, SVG, GIF, WEBP, etc.
- **Videos**: MP4, WEBM, AVI, MOV, etc.
- **Audio**: MP3, WAV, OGG, M4A, etc.
- **Files**: PDF, DOCX, TXT, and any other file types

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📚 Documentation

For complete API documentation, visit [docs.rs/mdz-rs](https://docs.rs/mdz-rs).

For the MDZ format specification, see the main project [README](../README.md).