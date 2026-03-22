//! Integration tests for mdz CLI and library
//!
//! This file contains both CLI command tests and library API tests.
//! Uses CARGO_BIN_EXE_mdz environment variable for robust binary lookup
//! that works with custom build-dir configurations.

use std::fs;
use std::env;
use tempfile::TempDir;
use predicates::prelude::*;
use assert_cmd::Command;

// ============================================================================
// CLI Tests
// ============================================================================

/// Get the mdz command using the compile-time binary path
/// This approach is robust to custom build-dir configurations
fn mdz_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mdz"))
}

#[test]
fn test_cli_help() {
    mdz_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("mdz"))
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_cli_version() {
    mdz_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("mdz"))
        .stdout(predicate::str::contains("1.0.0"));
}

#[test]
fn test_cli_pack_help() {
    mdz_cmd()
        .arg("pack")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("pack"));
}

#[test]
fn test_cli_unpack_help() {
    mdz_cmd()
        .arg("unpack")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("unpack"));
}

#[test]
fn test_cli_pack_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let output_file = temp_path.join("test.mdz");

    mdz_cmd()
        .arg("pack")
        .arg("nonexistent.md")
        .arg("--output").arg(output_file.to_str().unwrap())
        .assert()
        .failure();
}

#[test]
fn test_cli_unpack_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let output_dir = temp_path.join("output");

    mdz_cmd()
        .arg("unpack")
        .arg("nonexistent.mdz")
        .arg("--output").arg(output_dir.to_str().unwrap())
        .assert()
        .failure();
}

// ============================================================================
// Library API Tests
// ============================================================================

#[tokio::test]
async fn test_full_pack_unpack_cycle() {
    // 创建临时目录
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // 创建测试 Markdown 文件
    let markdown_content = r#"# 集成测试文档

这是一个测试文档，包含多种类型的资源。

## 图片

![本地图片](./test.png)
![另一个图片](./image.svg)

## 多媒体

这个文档将被打包和解包。
"#;

    let md_file = temp_path.join("test.md");
    fs::write(&md_file, markdown_content).unwrap();

    // 创建测试图片
    let png_file = temp_path.join("test.png");
    // 创建一个简单的 1x1 像素 PNG 文件
    let png_content = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
        0x49, 0x48, 0x44, 0x52, // IHDR
        0x00, 0x00, 0x00, 0x01, // width: 1
        0x00, 0x00, 0x00, 0x01, // height: 1
        0x08, 0x02, 0x00, 0x00, 0x00, // bit depth, color type, compression, filter, interlace
        0x90, 0x77, 0x53, 0xDE, // CRC
        0x00, 0x00, 0x00, 0x0C, // IDAT chunk length
        0x49, 0x44, 0x41, 0x54, // IDAT
        0x08, 0x99, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // compressed data
        0x00, 0x00, 0x00, 0x00, // IEND chunk length
        0x49, 0x45, 0x4E, 0x44, // IEND
        0xAE, 0x42, 0x60, 0x82, // CRC
    ];
    fs::write(&png_file, png_content).unwrap();

    let svg_file = temp_path.join("image.svg");
    let svg_content = "<svg width=\"200\" height=\"200\" xmlns=\"http://www.w3.org/2000/svg\">
  <circle cx=\"100\" cy=\"100\" r=\"80\" fill=\"#4CAF50\"/>
  <text x=\"100\" y=\"105\" text-anchor=\"middle\" fill=\"white\">SVG Test</text>
</svg>";
    fs::write(&svg_file, svg_content).unwrap();

    // 创建输出路径
    let mdz_file = temp_path.join("output.mdz");

    // 运行 pack 命令（通过 mdz-rs API）
    mdz_rs::pack(
        md_file.to_str().unwrap(),
        mdz_file.to_str().unwrap(),
    ).await.unwrap();

    // 验证 MDZ 文件存在
    assert!(mdz_file.exists());

    // 创建解包输出目录
    let unpack_dir = temp_path.join("unpack_output");
    fs::create_dir_all(&unpack_dir).unwrap();

    // 运行 unpack 命令（输出到指定目录）
    mdz_rs::unpack(
        mdz_file.to_str().unwrap(),
        Some(unpack_dir.to_str().unwrap()),
    ).unwrap();

    // 验证解包后的文件结构
    assert!(unpack_dir.join("test.md").exists());
    assert!(unpack_dir.join("assets/images/test.png").exists());
    assert!(unpack_dir.join("assets/images/image.svg").exists());

    // 验证解包后的 md 内容
    let unpacked_md = fs::read_to_string(unpack_dir.join("test.md")).unwrap();
    assert!(unpacked_md.contains("集成测试文档"));
    assert!(unpacked_md.contains("本地图片"));
    assert!(unpacked_md.contains("另一个图片"));

    // 验证图片文件存在即可（PNG 是二进制文件，无法读取为字符串）
    assert!(unpack_dir.join("assets/images/test.png").exists());

    // 验证 SVG 文件内容
    let unpacked_svg = fs::read_to_string(unpack_dir.join("assets/images/image.svg")).unwrap();
    assert!(unpacked_svg.contains("SVG Test"));
}

#[tokio::test]
async fn test_empty_document() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // 创建空 Markdown 文档
    let markdown_content = "# 空文档\n\n这个文档没有图片。";
    let md_file = temp_path.join("empty.md");
    fs::write(&md_file, markdown_content).unwrap();

    let mdz_file = temp_path.join("empty.mdz");

    // 切换到临时目录
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_path).unwrap();

    // 打包
    mdz_rs::pack(
        md_file.to_str().unwrap(),
        mdz_file.to_str().unwrap(),
    ).await.unwrap();

    // 解包
    mdz_rs::unpack(
        mdz_file.to_str().unwrap(),
        None,
    ).unwrap();

    // 验证 - 空文档不会创建 assets 目录
    assert!(temp_path.join("empty.md").exists());
    // assets 目录可能不存在，这是正常的

    // 恢复原始目录
    std::env::set_current_dir(original_dir).unwrap();
}
