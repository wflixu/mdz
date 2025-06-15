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
├── media/             # Directory for embedded media files (optional)
│   ├── image1.png
│   └── video1.mp4
└── attachments/       # Directory for attached files (optional)
```

### Required Components

| File            | Description                                       |
| --------------- | ------------------------------------------------- |
| `index.md`      | The primary Markdown document (UTF-8 encoded)     |
| `manifest.json` | JSON metadata describing assets and configuration |

### Optional Components

| Directory      | Description                                             |
| -------------- | ------------------------------------------------------- |
| `media/`       | Contains images, videos, audio, or other embedded media |
| `attachments/` | Contains non-media attachments (e.g., PDFs)             |

---

## manifest.json Schema

```json
{
  "version": "1.0.0",
  "title": "Document Title",
  "author": "Author Name",
  "date": "2025-06-15",
  "assets": [
    {
      "id": "img1",
      "path": "media/image1.png",
      "type": "image",
      "alt": "An example image"
    },
    {
      "id": "vid1",
      "path": "media/video1.mp4",
      "type": "video",
      "title": "Intro Video"
    }
  ]
}
```

### Field Definitions

| Field     | Type   | Required | Description                                         |
| --------- | ------ | -------- | --------------------------------------------------- |
| `version` | string | yes      | Specification version for compatibility             |
| `title`   | string | yes      | Title of the document                               |
| `author`  | string | no       | Name of the document author                         |
| `date`    | string | no       | Publication or creation date (ISO 8601 recommended) |
| `assets`  | array  | no       | List of embedded assets                             |

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

---

## License

This specification is published under the MIT License.

---

**Author**: Li Xu
**Initial Release**: June 2025
**Repository**: [https://github.com/](https://github.com/)<your-org>/mdz-spec
