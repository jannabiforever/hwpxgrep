# HWPXGrep

**HWPXGrep** is a high-performance tool written in Rust for searching text within HWPX files. It leverages caching to enable fast and efficient text search, minimizing the need to repeatedly extract and parse the file's XML structure.

---

## Features

- **High Performance**: Built with Rust for optimal speed and efficiency.
- **Text Search**: Quickly search for text within HWPX files.
- **Automatic Caching**: Generates and updates cache files to speed up repeated searches.
- **Flexible File Support**: Parses the HWPX (ZIP-based) structure to extract content.
- **Efficient Change Detection**: Ensures cache is up-to-date by checking file modifications.

---

## How It Works

1. **Extract**: Decompress the HWPX file to access its XML structure.
2. **Parse**: Analyze `content.xml` to extract text data.
3. **Cache**: Save extracted text in a lightweight format for reuse.
4. **Search**: Use cached data to perform fast text lookups without reprocessing the original file.

---

## Installation

### Prerequisites

- Rust and Cargo (https://www.rust-lang.org/tools/install)

### Build and Run

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/hwpxgrep.git
cd hwpxgrep
cargo build --release
```

Run the executable:

```bash
./target/release/hwpxgrep
```

---

## Usage

### Generate Cache

To create or update a cache for an HWPX file:

```bash
hwpxgrep cache path/to/file.hwpx
```

### Search Text

To search for a specific term in the HWPX file:

```bash
hwpxgrep search "your keyword" path/to/file.hwpx
```

### Example

```bash
hwpxgrep cache example.hwpx
hwpxgrep search "document title" example.hwpx
```

---

## Cache Structure

### Plain Text Format

Simple text-based cache for minimal storage:

```plaintext
Title: Document Title
Body: This is the content of the document.
```

### JSON Format

Structured cache for advanced querying:

```json
{
  "hash": "sha256-file-hash",
  "content": "This is the document content.",
  "metadata": {
    "title": "Document Title",
    "keywords": ["document", "content"]
  }
}
```

---

## Roadmap

- [ ] Implement advanced caching strategies.
- [ ] Add support for multi-threaded search.
- [ ] Support for additional output formats.

---

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any suggestions or improvements.

---

## License

This project is licensed under the MIT License. See `LICENSE` for details.
