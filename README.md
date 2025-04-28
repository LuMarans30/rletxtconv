# Conway's Game of Life File Converter

A _blazingly-fast_ Rust-based tool to convert between plaintext (`.cells`) and Run-Length Encoded (RLE) formats for Conway's Game of Life patterns.

## Features

- Convert between `.cells` (plaintext) and `.rle` formats.
- Automatic format detection for input files, regardless of file extension

## Installation

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

### Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/LuMarans30/rletxtconv.git && cd rletxtconv
   ```
   
2. Build the project:
   ```bash
   cargo build --release
   ```

The binary will be located at `target/release/rletxtconv`.

### Pre-built binaries

Alternatively, you can find the binaries in the [releases](https://github.com/LuMarans30/rletxtconv/releases) page.

## Usage

Convert a file to the RLE format:
```bash
rletxtconv --input input.cells --output output.rle
```

Force overwrite if the output file exists:
```bash
rletxtconv --input input.rle --output output.cells --force
```
