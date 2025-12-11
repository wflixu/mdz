# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

MDZ 是一个用 Rust 实现的自定义 Markdown 文件格式，支持将 Markdown 文档和嵌入的资源（图片、视频、音频、其他文件）打包成单一文件。项目采用 Cargo workspace 管理，包含一个核心库和一个 CLI 工具。

## 常用命令

### 构建和安装
```bash
# 构建整个项目
cargo build

# 构建并安装 CLI 工具
cargo install --path mdz

# 运行测试
cargo test

# 运行检查（快速检查，不构建）
cargo check

# 格式化代码
cargo fmt

# 运行 clippy 检查
cargo clippy
```

### 针对特定 crate 的命令
```bash
# 只构建库
cargo build -p mdz-rs

# 只构建 CLI 工具
cargo build -p mdz

# 只测试库
cargo test -p mdz-rs

# 运行单个测试
cargo test -p mdz-rs it_works
```

## 项目架构

### Workspace 结构
- **根目录**: 包含 Cargo workspace 配置
- **mdz-rs/**: 核心库 crate，提供打包和解包功能
- **mdz/**: CLI 工具 crate，提供命令行接口

### 核心组件
1. **mdz-rs 库**: 实现了 MDZ 格式的核心逻辑
   - `pack()`: 将 Markdown 文件和资源打包成 .mdz 文件
   - `unpack()`: 从 .mdz 文件解包内容
   - 当前为占位实现，需要添加 ZIP 操作和清单文件生成

2. **mdz CLI**: 提供命令行界面
   - 当前为占位实现，需要集成 clap 进行参数解析

### MDZ 文件格式（根据 mdz-spec.md）
- ZIP 压缩格式
- 必须包含：
  - `index.md`: 主要的 Markdown 内容（UTF-8 编码）
  - `manifest.json`: 元数据和资源映射
- 可选目录：
  - `assets/`: 所有嵌入资源的统一目录
    - `assets/images/`: 图片文件（PNG, JPG, SVG 等）
    - `assets/videos/`: 视频文件（MP4, WebM 等）
    - `assets/audio/`: 音频文件（MP3, WAV 等）
    - `assets/files/`: 其他附件文件（PDFs, 文档等）

### 资源引用方式
- 在 Markdown 中使用 `assets://<asset-id>` 引用资源
- 资源 ID 在 manifest.json 中定义

## 依赖项

### mdz-rs 库
- `zip`: ZIP 压缩/解压
- `serde` + `serde_json`: JSON 序列化/反序列化（用于 manifest.json）
- `anyhow`: 错误处理

### mdz CLI
- `clap`: 命令行参数解析（带 derive 特性）
- `anyhow`: 错误处理
- `mdz-rs`: 内部库依赖

## 开发注意事项

1. **Rust 版本**: 项目使用 Rust 2024 版本（edition = "2024"）
2. **编码**: 所有文本文件必须使用 UTF-8 编码
3. **测试**: 库中已有一个基础测试框架
4. **待实现**: pack() 和 unpack() 函数目前是占位实现

## 构建产物

- CLI 工具位于 `target/debug/mdz`（开发版本）
- 库文件位于 `target/debug/libmdz_rs.rlib`
- 使用 `cargo install --path mdz` 安装到本地 cargo bin 目录