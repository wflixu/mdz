# mdz

A Rust library and CLI tool for defining, packaging, and unpacking a custom Markdown-based file format with embedded image resources.

## Features

- **Custom File Format**: Defines a new format for bundling Markdown content and its associated images.
- **Rust Library**: Core logic implemented as a reusable Rust library ([`mdz-rs`](mdz-rs/)).
- **CLI Tool**: Command-line utility ([`mdz`](mdz/)) for packing and unpacking `.mdz` files.
- **Easy Integration**: Designed for use in other Rust projects or as a standalone tool.

## Project Structure

```
.
├── LICENSE
├── mdz-spec.md
├── mdz/         # CLI tool crate
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── mdz-rs/      # Library crate
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

- [`mdz/`](mdz/): CLI tool implementation ([main.rs](mdz/src/main.rs))
- [`mdz-rs/`](mdz-rs/): Core library ([lib.rs](mdz-rs/src/lib.rs))
- [mdz-spec.md](mdz-spec.md): File format specification

## Installation

Build and install the CLI tool locally:

```sh
cargo install --path mdz
```

Or add the library to your Rust project:

```toml
[dependencies]
mdz-rs = { path = "path/to/mdz-rs" }
```

## Usage

### As a Library

Import and use the core functionality in your Rust code. See [mdz-rs/src/lib.rs](mdz-rs/src/lib.rs) for API details.

### As a CLI Tool

- **Pack** a Markdown file and images into an `.mdz` archive:

    ```sh
    mdz pack input.md images/ output.mdz
    ```

- **Unpack** an `.mdz` archive:

    ```sh
    mdz unpack input.mdz output_dir/
    ```

## File Format

- Supports standard Markdown syntax.
- Bundles Markdown content and all referenced images into a single portable file.
- See [mdz-spec.md](mdz-spec.md) for detailed specification.

## Contributing

Contributions are welcome! Please open issues or submit pull requests.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for