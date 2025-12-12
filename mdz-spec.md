# Markdown Zip Format (MDZ) Specification - v1.0.0

## Overview

**MDZ** (Markdown Zip) is an open archive format designed for bundling Markdown documents together with their related assets (images, videos, audio, attachments). It solves the limitation of standalone Markdown files that cannot embed or reliably reference external media, offering a portable, structured, and extensible document format.

The MDZ format is inspired by established document container formats like DOCX and EPUB, and utilizes the ZIP archive standard for packaging.

---

## File Extension and MIME Type

* **File extension**: `.mdz`
* **Recommended MIME type**: `application/x-mdz`

---

## Structure of an MDZ File

An MDZ file is a **ZIP** archive with a predefined directory structure:

```
archive.mdz (ZIP archive)
├── index.md           # Main Markdown content
├── manifest.json      # Metadata and asset mapping (required)
└── assets/            # Directory for all embedded assets (optional)
    ├── images/
    │   └── image1.png
    ├── videos/
    │   └── video1.mp4
    ├── audio/
    │   └── audio1.mp3
    └── files/
        └── document.pdf
```

### Required Components

| File            | Description                                       |
| --------------- | ------------------------------------------------- |
| `index.md`      | The primary Markdown document (UTF-8 encoded)     |
| `manifest.json` | JSON metadata describing assets and configuration |

### Optional Components

| Directory  | Description                                      |
| ---------- | ------------------------------------------------ |
| `assets/`  | Contains all embedded assets organized by type   |
- `assets/images/` - Image files (PNG, JPG, SVG, etc.)
- `assets/videos/` - Video files (MP4, WebM, etc.)
- `assets/audio/` - Audio files (MP3, WAV, etc.)
- `assets/files/` - Other file attachments (PDFs, docs, etc.) |

---

## manifest.json Schema

```json
{
  "version": "1.0.0",
  "title": "Document Title",
  "author": null,
  "date": "2025-12-12",
  "filename": "document.md",
  "assets": [
    {
      "id": "img1",
      "path": "assets/images/image1.png",
      "type": "image",
      "alt": "An example image"
    },
    {
      "id": "97f524af-9d6d-4b22-8e3e-838c7ae15733",
      "path": "assets/images/97f524af-9d6d-4b22-8e3e-838c7ae15733.png",
      "type": "image",
      "alt": "Downloaded network image"
    },
    {
      "id": "vid1",
      "path": "assets/videos/video1.mp4",
      "type": "video",
      "title": "Intro Video"
    }
  ]
}
```

### Field Definitions

| Field      | Type   | Required | Description                                         |
| ---------- | ------ | -------- | --------------------------------------------------- |
| `version`  | string | yes      | Specification version for compatibility             |
| `title`    | string | yes      | Title of the document (defaults to filename)        |
| `author`   | string | no       | Name of the document author                         |
| `date`     | string | no       | Publication or creation date (ISO 8601 recommended) |
| `filename` | string | yes      | Original filename of the markdown document          |
| `assets`   | array  | no       | List of embedded assets                             |

#### Asset Object Fields

| Field   | Type   | Required | Description                                            |
| ------- | ------ | -------- | ------------------------------------------------------ |
| `id`    | string | yes      | Unique identifier used for reference in index.md       |
| `path`  | string | yes      | Relative path to the asset file within the archive     |
| `type`  | string | yes      | Type of asset: `image`, `video`, `audio`, `file`, etc. |
| `alt`   | string | no       | Alternative text (for images)                          |
| `title` | string | no       | Title or caption                                       |

---

## Referencing Assets in index.md

To reference an asset within the Markdown content, use the following URI scheme:

```
assets://<asset-id>
```

**Example usage:**

```markdown
![Example Image](assets://img1)

[Watch Intro Video](assets://vid1)
```

Renderers or parsers MUST resolve `assets://` URIs by referring to `manifest.json`.

---

## Compression Method

* Compression: **DEFLATE** (ZIP default)
* Encoding: All text files (index.md, manifest.json) MUST be UTF-8 encoded

---

## Versioning

* The `manifest.json` includes a `version` field corresponding to the MDZ specification version.
* Future versions MAY introduce new fields but MUST maintain backward compatibility for core fields.

---

## Compatibility and Extensibility

* **Backward Compatibility**: Parsers should ignore unknown fields in `manifest.json`.
* **Forward Compatibility**: Producers SHOULD NOT remove required fields in future versions.
* **Custom Extensions**: May be added using namespaced keys, e.g., `x-myapp-feature`.

---

## Future Considerations

* Support for multiple Markdown files per archive (e.g., `/documents/` directory)
* Table of contents or navigation map in manifest.json
* Digital signatures for integrity verification
* Optional encryption (using AES or other standards)
* Asset categorization and tagging system
* Nested directory support within assets folder

---

## License

This specification is published under the MIT License.

---

## Implementation: MDZ CLI Tool

A reference implementation of the MDZ format is provided as a command-line tool with the following features:

### Pack Command

```bash
# Basic usage - output to input_file.mdz
mdz pack <input_file>

# Specify output path
mdz pack <input_file> --output <output_file>
mdz pack <input_file> -o <output_file>
```

**Features:**
- Automatically downloads network images and saves them with UUID filenames
- Copies local images to assets directory
- Updates all image references to use `assets://` protocol
- Supports PNG, JPG, SVG, and other image formats
- Handles download failures gracefully (keeps original links)

### Unpack Command

```bash
# Unpack to MDZ file's parent directory (default)
mdz unpack <input_file>

# Unpack to specific directory
mdz unpack <input_file> --output <directory>
mdz unpack <input_file> -o <directory>
```

**Features:**
- Restores original markdown filename
- Converts `assets://` links to relative paths
- Maintains directory structure
- Preserves all asset files

### Asset Handling

**Network Images:**
- Downloaded asynchronously with UUID filenames: `{uuid}.extension`
- On download failure: original URL is preserved
- Supported protocols: HTTP, HTTPS

**Local Files:**
- Copied with original filenames + counter to avoid conflicts
- Resolved relative to markdown file location
- Supports both relative and absolute paths

**Example Transformation:**
```markdown
# Before packing:
![local](./images/local.png)
![network](https://example.com/image.jpg)

# After packing (stored in index.md):
![local](assets://assets/images/local.png)
![network](assets://assets/images/12345678-1234-5678-9abc-123456789def.jpg)

# After unpacking:
![local](assets/images/local.png)
![network](assets/images/12345678-1234-5678-9abc-123456789def.jpg)
```

---

**Author**: Li Xu
**Initial Release**: June 2025
**Repository**: [https://github.com/](https://github.com/)<your-org>/mdz-spec
