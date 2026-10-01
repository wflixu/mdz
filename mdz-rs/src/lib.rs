//! # MDZ - Markdown Zip Library
//!
//! A Rust library for creating and working with MDZ (Markdown Zip) files.
//! MDZ is a ZIP-based archive format that bundles Markdown documents with their
//! embedded assets (images, videos, audio, and other files) into a single, portable file.
//!
//! ## Features
//!
//! - **Automatic Asset Processing**: Download network images and copy local files
//! - **Smart Link Resolution**: Convert absolute paths to relative paths for maximum compatibility
//! - **UUID-based Naming**: Use UUIDs for downloaded assets to avoid conflicts
//! - **Backward Compatibility**: Handle legacy MDZ files seamlessly
//! - **Rich Metadata**: Complete manifest with document and asset information
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use mdz_rs::{pack, unpack};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Pack a markdown file with assets
//!     pack("document.md", "document.mdz").await?;
//!
//!     // Unpack MDZ file to extract content and assets
//!     unpack("document.mdz", Some("output/"))?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## MDZ Format Structure
//!
//! ```text
//! document.mdz (ZIP archive)
//! ├── index.md              # Updated markdown with relative asset links
//! ├── manifest.json         # Metadata and asset mapping
//! └── assets/               # Organized asset files
//!     ├── images/
//!     ├── videos/
//!     ├── audio/
//!     └── files/
//! ```
//!
//! ## Asset Handling
//!
//! The library automatically processes:
//!
//! - **Network Images**: Downloaded asynchronously with UUID filenames
//! - **Local Files**: Copied with conflict resolution (counter suffixes)
//! - **Link Updates**: Markdown links are updated to use relative paths (`./assets/...`)
//! - **Metadata**: Complete manifest.json with asset mapping and document info

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use url::Url;

// 包含测试模块
#[cfg(test)]
mod tests;

/// Represents an embedded asset within an MDZ archive.
///
/// Assets are stored in the manifest.json file and reference actual files
/// within the archive's assets/ directory.
///
/// # Examples
///
/// ```json
/// {
///   "id": "image1",
///   "path": "assets/images/image1.png",
///   "type": "image",
///   "alt": "Example image",
///   "title": "A sample image"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    /// Unique identifier for the asset, used for internal reference
    pub id: String,

    /// Relative path to the asset file within the MDZ archive
    pub path: String,

    /// Type of the asset (image, video, audio, file)
    #[serde(rename = "type")]
    pub asset_type: String,

    /// Alt text for images (accessibility)
    pub alt: Option<String>,

    /// Title or description of the asset
    pub title: Option<String>,
}

/// Manifest structure for MDZ files.
///
/// The manifest.json file contains metadata about the document and all embedded assets.
/// It's stored in the root of the MDZ archive and provides information about the
/// original document structure.
///
/// # Examples
///
/// ```json
/// {
///   "version": "1.1.0",
///   "title": "My Document",
///   "author": "John Doe",
///   "date": "2025-12-13",
///   "filename": "document.md",
///   "assets": [...]
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// MDZ specification version (e.g., "1.1.0")
    pub version: String,

    /// Document title
    pub title: String,

    /// Document author (optional)
    pub author: Option<String>,

    /// Creation or modification date (ISO 8601 format, optional)
    pub date: Option<String>,

    /// Original markdown filename (optional since v1.1.0)
    pub filename: Option<String>,

    /// Subject of the document (optional document property)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Summary or abstract (optional document property)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Keywords (optional document property)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    /// Language as a BCP 47 tag such as "fr" or "en-GB" (optional document property)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Licence, preferably an SPDX identifier such as "CC-BY-4.0" (optional document property)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// List of all embedded assets
    pub assets: Vec<Asset>,
}

/// Determines if a given path is a URL or a local file path.
///
/// This function uses URL parsing to determine if the provided string represents
/// a valid URL (HTTP, HTTPS, FTP, etc.) or a local file system path.
///
/// # Arguments
///
/// * `path` - The path string to check
///
/// # Returns
///
/// Returns `true` if the path is a valid URL, `false` otherwise.
///
/// # Examples
///
/// ```
/// use mdz_rs::is_url;
///
/// assert!(is_url("https://example.com/image.jpg"));
/// assert!(is_url("http://localhost:8080/file.pdf"));
/// assert!(!is_url("./local/image.png"));
/// assert!(!is_url("/absolute/path/file.jpg"));
/// ```
pub fn is_url(path: &str) -> bool {
    Url::parse(path).is_ok()
}

/// Get asset type based on file extension
fn get_asset_type(path: &Path) -> String {
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match extension.as_deref() {
        Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("bmp") |
        Some("svg") | Some("webp") | Some("ico") => "image".to_string(),
        Some("mp4") | Some("avi") | Some("mov") | Some("wmv") | Some("webm") |
        Some("mkv") | Some("flv") => "video".to_string(),
        Some("mp3") | Some("wav") | Some("ogg") | Some("flac") | Some("aac") => "audio".to_string(),
        _ => "file".to_string(),
    }
}

/// Get appropriate subdirectory for asset type
fn get_asset_subdir(asset_type: &str) -> &'static str {
    match asset_type {
        "image" => "images",
        "video" => "videos",
        "audio" => "audio",
        _ => "files",
    }
}

/// Download an image from URL to local file
async fn download_image(url: &str, dest_path: &Path) -> Result<()> {
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to download image: {}", response.status()));
    }

    let content = response.bytes().await?;

    // Create parent directories if they don't exist
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(dest_path, content)?;
    Ok(())
}

/// Copy local file to destination
fn copy_local_file(src_path: &Path, dest_path: &Path) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(src_path, dest_path)?;
    Ok(())
}

/// Extract image URLs and paths from Markdown content
fn extract_images_from_markdown(content: &str) -> Vec<(String, Option<String>)> {
    let mut images = Vec::new();

    // Regex patterns for different Markdown image syntaxes
    let patterns = vec![
        // ![alt](url) - standard Markdown image
        r#"\!\[([^\]]*)\]\(([^)]+)\)"#,
        // <img src="url" alt="alt"> - HTML img tag
        r#"<img[^>]+src=["']([^"']+)["'][^>]*>"#,
    ];

    // Pre-compile regex for extracting alt attribute
    let alt_regex = Regex::new(r#"alt=["']([^"']*)["']"#).unwrap();

    for pattern in patterns {
        let regex = Regex::new(pattern).unwrap();
        for captures in regex.captures_iter(content) {
            match pattern {
                p if p.starts_with(r#"\!\["#) => {
                    // Markdown syntax
                    let alt = captures.get(1).map(|m| m.as_str().to_string());
                    let url = captures.get(2).unwrap().as_str().to_string();
                    images.push((url, alt));
                }
                p if p.starts_with(r#"<img"#) => {
                    // HTML img tag
                    let url = captures.get(1).unwrap().as_str().to_string();
                    // Try to extract alt attribute from the full match
                    let full_match = captures.get(0).unwrap().as_str();
                    let alt = alt_regex.captures(full_match)
                        .and_then(|c| c.get(1))
                        .map(|m| m.as_str().to_string());
                    images.push((url, alt));
                }
                _ => {}
            }
        }
    }

    images
}

/// Update markdown content to use relative paths to assets
fn update_markdown_links(content: &str, assets: &[(Asset, String)]) -> Result<String> {
    let mut updated_content = content.to_string();

    // Create a mapping from original URLs to new relative paths (./assets/...)
    let mut url_mapping: HashMap<String, String> = HashMap::new();

    for (asset, original_url) in assets {
        let new_url = format!("./{}", asset.path);
        url_mapping.insert(original_url.clone(), new_url);
    }

    // Update Markdown image links: ![alt](url)
    let markdown_regex = Regex::new(r#"(\!\[([^\]]*)\]\()([^)]+)\)"#).unwrap();
    updated_content = markdown_regex.replace_all(&updated_content, |caps: &regex::Captures| {
        let alt = caps.get(2).unwrap().as_str();     // alt text
        let url = caps.get(3).unwrap().as_str();     // url

        if let Some(new_url) = url_mapping.get(url) {
            format!("![{}]({})", alt, new_url)
        } else {
            format!("![{}]({})", alt, url)
        }
    }).to_string();

    // Update HTML img tags: <img src="url" ...>
    let html_regex = Regex::new(r#"(<img[^>]+src=["'])([^"']+)(["'][^>]*>)"#).unwrap();
    updated_content = html_regex.replace_all(&updated_content, |caps: &regex::Captures| {
        let prefix = caps.get(1).unwrap().as_str();
        let url = caps.get(2).unwrap().as_str();
        let suffix = caps.get(3).unwrap().as_str();

        if let Some(new_url) = url_mapping.get(url) {
            format!("{}{}{}", prefix, new_url, suffix)
        } else {
            format!("{}{}{}", prefix, url, suffix)
        }
    }).to_string();

    Ok(updated_content)
}

/// Packs a Markdown file and its assets into an MDZ archive.
///
/// This function reads a markdown file, extracts all referenced assets, downloads
/// network images, copies local files, updates links to use relative paths, and
/// bundles everything into a single MDZ file.
///
/// # Arguments
///
/// * `markdown_file` - Path to the source markdown file
/// * `output_file` - Path where the MDZ file should be created
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if packing fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The markdown file cannot be read
/// - Network images cannot be downloaded
/// - Local files cannot be copied
/// - The MDZ file cannot be created
///
/// # Examples
///
/// ```rust,no_run
/// use mdz_rs::pack;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     pack("document.md", "document.mdz").await?;
///     println!("Successfully packed document.mdz");
///     Ok(())
/// }
/// ```
///
/// # Asset Processing
///
/// The function automatically handles:
/// - **Network Images**: Downloaded with UUID filenames (e.g., `12345678-... .jpg`)
/// - **Local Files**: Copied with conflict resolution (adds counter suffixes)
/// - **Link Updates**: Updated to use relative paths (`./assets/...`)
/// - **Directory Structure**: Organized by type in `assets/` subdirectories
pub async fn pack(
    markdown_file: &str,
    output_file: &str,
) -> Result<()> {
    // Read the markdown file
    let markdown_path = Path::new(markdown_file);
    let markdown_content = fs::read_to_string(markdown_path)?;

    // Extract images from markdown
    let extracted_images = extract_images_from_markdown(&markdown_content);

    // Create a temporary directory for the MDZ structure
    let temp_dir = std::env::temp_dir().join(format!("mdz_assets_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir)?;

    // Create assets directory structure
    let assets_dir = temp_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;
    let images_dir = assets_dir.join("images");
    let videos_dir = assets_dir.join("videos");
    let audio_dir = assets_dir.join("audio");
    let files_dir = assets_dir.join("files");
    fs::create_dir_all(&images_dir)?;
    fs::create_dir_all(&videos_dir)?;
    fs::create_dir_all(&audio_dir)?;
    fs::create_dir_all(&files_dir)?;

    let mut assets = Vec::new();
    let mut asset_counter = HashMap::new();
    let mut assets_with_original_urls = Vec::new();

    // Process each image
    for (image_url, alt_text) in extracted_images {
        // Skip if it's already an assets:// URL
        if image_url.starts_with("assets://") {
            continue;
        }

        // Determine if it's a URL or local path
        let is_remote = is_url(&image_url);

        // Determine file extension first
        let extension = if is_remote {
            Path::new(&image_url)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("png")
        } else {
            Path::new(&image_url)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("png")
        };

        // Generate UUID-based filename for remote images, keep original for local files
        let (_filename, _asset_path_in_archive, should_process) = if is_remote {
            let uuid_filename = format!("{}.{}", uuid::Uuid::new_v4(), extension);
            let asset_type = get_asset_type(Path::new(&uuid_filename));
            let subdir = get_asset_subdir(&asset_type);
            let path = format!("assets/{}/{}", subdir, uuid_filename);
            let asset_dest_dir = match asset_type.as_str() {
                "image" => &images_dir,
                "video" => &videos_dir,
                "audio" => &audio_dir,
                _ => &files_dir,
            };
            let final_asset_path = asset_dest_dir.join(&uuid_filename);

            // Try to download the remote image
            match download_image(&image_url, &final_asset_path).await {
                Ok(()) => {
                    let asset = Asset {
                        id: uuid_filename.clone(),
                        path: path.clone(),
                        asset_type,
                        alt: alt_text.clone(),
                        title: None,
                    };
                    (uuid_filename, path, Some((asset, image_url.clone())))
                }
                Err(e) => {
                    // Download failed, skip this image and keep original link
                    eprintln!("Warning: Failed to download image '{}': {}. Keeping original link.", image_url, e);
                    continue;
                }
            }
        } else {
            // Local file - use original filename logic
            let base_name = Path::new(&image_url)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("image")
                .to_string();

            // Ensure unique ID
            let count = asset_counter.entry(base_name.clone()).or_insert(0);
            let asset_id = if *count == 0 {
                base_name.clone()
            } else {
                format!("{}_{}", base_name, count)
            };
            *count += 1;

            let local_filename = format!("{}.{}", asset_id, extension);
            let asset_type = get_asset_type(Path::new(&local_filename));
            let subdir = get_asset_subdir(&asset_type);
            let path = format!("assets/{}/{}", subdir, local_filename);
            let asset_dest_dir = match asset_type.as_str() {
                "image" => &images_dir,
                "video" => &videos_dir,
                "audio" => &audio_dir,
                _ => &files_dir,
            };
            let final_asset_path = asset_dest_dir.join(&local_filename);

            // Copy local file
            let full_image_path = if Path::new(&image_url).is_absolute() {
                PathBuf::from(&image_url)
            } else {
                markdown_path.parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join(&image_url)
            };

            match copy_local_file(&full_image_path, &final_asset_path) {
                Ok(()) => {
                    let asset = Asset {
                        id: asset_id,
                        path: path.clone(),
                        asset_type,
                        alt: alt_text.clone(),
                        title: None,
                    };
                    (local_filename, path, Some((asset, image_url.clone())))
                }
                Err(e) => {
                    // Copy failed, skip this image and keep original link
                    eprintln!("Warning: Failed to copy file '{}': {}. Keeping original link.", image_url, e);
                    continue;
                }
            }
        };

        // Add asset to lists if processing was successful
        if let Some((asset, original_url)) = should_process {
            assets_with_original_urls.push((asset.clone(), original_url));
            assets.push(asset);
        }
    }

    // Create manifest
    let original_filename = markdown_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("index.md")
        .to_string();

    let manifest = Manifest {
        version: "1.0.0".to_string(),
        title: markdown_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string(),
        author: None,
        date: Some(chrono::Utc::now().date_naive().to_string()),
        filename: Some(original_filename.clone()),
        subject: None,
        description: None,
        keywords: None,
        language: None,
        license: None,
        assets,
    };

    // Write manifest.json
    let manifest_path = temp_dir.join("manifest.json");
    fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;

    // Update markdown content with assets:// links and write index.md
    let updated_markdown_content = update_markdown_links(&markdown_content, &assets_with_original_urls)?;
    let index_path = temp_dir.join("index.md");
    fs::write(&index_path, updated_markdown_content)?;

    // Create ZIP file
    create_zip_file(&temp_dir, output_file)?;

    // Clean up temporary directory
    fs::remove_dir_all(&temp_dir)?;

    Ok(())
}

/// Create a ZIP file from the given directory
fn create_zip_file(source_dir: &Path, output_file: &str) -> Result<()> {
    use zip::{ZipWriter, write::FileOptions};
    use std::io::Write;
    use std::fs::File;

    let file = File::create(output_file)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::<'_, ()>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Add all files to the ZIP
    for entry in walkdir::WalkDir::new(source_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let name = path.strip_prefix(source_dir)?;
            let name_str = name.to_str().ok_or_else(|| anyhow!("Invalid path"))?;

            zip.start_file(name_str, options)?;

            let mut f = fs::File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}

/// Convert assets:// links to local relative paths
fn convert_assets_to_local(content: &str) -> String {
    let mut updated_content = content.to_string();

    // Update Markdown image links: ![alt](assets://path)
    let markdown_regex = Regex::new(r#"(\!\[([^\]]*)\]\()assets://([^)]+)\)"#).unwrap();
    updated_content = markdown_regex.replace_all(&updated_content, |caps: &regex::Captures| {
        let alt = caps.get(2).unwrap().as_str();
        let asset_path = caps.get(3).unwrap().as_str();
        format!("![{}]({})", alt, asset_path)
    }).to_string();

    // Update HTML img tags: <img src="assets://path" ...>
    let html_regex = Regex::new(r#"(<img[^>]+src=["'])assets://([^"']+)(["'][^>]*>)"#).unwrap();
    updated_content = html_regex.replace_all(&updated_content, |caps: &regex::Captures| {
        let prefix = caps.get(1).unwrap().as_str();
        let asset_path = caps.get(2).unwrap().as_str();
        let suffix = caps.get(3).unwrap().as_str();
        format!("{}{}{}", prefix, asset_path, suffix)
    }).to_string();

    updated_content
}

/// Unpacks an MDZ archive to extract the markdown file and embedded assets.
///
/// This function reads a MDZ file, extracts the manifest to determine the original
/// filename, and extracts all files to the specified directory. It handles both
/// v1.1.0 (with filename field) and v1.0.0 (without filename field) MDZ files.
///
/// # Arguments
///
/// * `input_file` - Path to the MDZ file to unpack
/// * `output_dir` - Directory where files should be extracted (None for parent directory)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if unpacking fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The MDZ file cannot be opened or is not a valid ZIP archive
/// - The manifest.json file is missing or malformed
/// - Files cannot be extracted to the output directory
///
/// # Examples
///
/// ```rust,no_run
/// use mdz_rs::unpack;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Unpack to current directory
///     unpack("document.mdz", None)?;
///
///     // Unpack to specific directory
///     unpack("document.mdz", Some("output/"))?;
///
///     println!("Successfully unpacked MDZ file");
///     Ok(())
/// }
/// ```
///
/// # Backward Compatibility
///
/// This function handles MDZ files created with different specification versions:
/// - **v1.1.0+**: Uses the filename field from manifest.json
/// - **v1.0.0**: Derives filename from the MDZ file basename
pub fn unpack(input_file: &str, output_dir: Option<&str>) -> Result<()> {
    use zip::ZipArchive;
    use std::fs::File;
    use std::io::{Read, Write};

    let file = File::open(input_file)?;
    let mut archive = ZipArchive::new(file)?;

    // Read manifest first to get original filename
    let manifest_content = {
        let mut manifest_file = archive.by_name("manifest.json")?;
        let mut content = String::new();
        manifest_file.read_to_string(&mut content)?;
        content
    };

    let manifest: Manifest = serde_json::from_str(&manifest_content)?;
    let output_md_filename = manifest.filename.unwrap_or_else(|| {
        // 向后兼容：如果没有 filename 字段，从输入文件路径推导
        let input_path = Path::new(input_file);
        input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("index")
            .to_string() + ".md"
    });

    // Determine output directory
    let base_output_path = Path::new(output_dir.unwrap_or("."));

    // Extract all files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let filepath = file.name().to_string();  // Clone the string to avoid borrow issues

        // Skip manifest.json as it's only used internally
        if filepath == "manifest.json" {
            continue;
        }

        // For index.md, use the original filename and convert assets:// links
        let relative_path = if filepath == "index.md" {
            &output_md_filename
        } else {
            &filepath
        };

        let outpath = base_output_path.join(relative_path);

        if filepath.ends_with('/') {
            fs::create_dir_all(outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            // Handle text files (Markdown) differently from binary files (images)
            if filepath == "index.md" {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                content = convert_assets_to_local(&content);
                let mut outfile = fs::File::create(outpath)?;
                outfile.write_all(content.as_bytes())?;
            } else {
                // For binary files, copy directly
                let mut outfile = fs::File::create(outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
    }

    Ok(())
}

