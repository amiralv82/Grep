# Text Search Tool

This is a versatile text search tool developed in Rust that allows you to search for specific patterns in files and directories. It offers various search options, including case-insensitive searching, non-matching results, and the ability to search within directories.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Usage](#usage)
- [Configuration Options](#configuration-options)
- [Examples](#examples)
- [License](#license)

## Features

- Search for patterns in files and directories.
- Option for case-insensitive searching.
- Find files that do not match the specified pattern.
- Search within directories and their contents.
- Define search depth within directories.
- Parallel searching using multiple threads.
- Utilizes the [regex](https://crates.io/crates/regex) and [walkdir](https://crates.io/crates/walkdir) crates.
- Easy-to-use command-line interface.

## Prerequisites

Before using this tool, ensure you have the following prerequisites:

- Rust and Cargo: [Installation Instructions](https://www.rust-lang.org/learn/get-started)
- Git: [Installation Instructions](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

## Usage

Clone the repository and build the tool using the following commands:

```bash
git clone https://github.com/yourusername/text-search-tool.git
cd text-search-tool
cargo build --release
```

Run the tool with the desired options:

```bash
cargo run [OPTIONS] PATTERN [FILE/DIRECTORY]
```

## Configuration Options

The tool supports various options that customize your search. These options are specified as environment variables:

- `IC`: Perform case-insensitive searching.
- `NON`: Find files and lines that do not match the pattern.
- `DIR`: Search inside directories and find only files with the pattern.
- `IN`: Search inside files found in directories.
- `DPT:[1..n]`: Specify the search depth through directories (default is 1).
- `TH`: Enable parallel searching.
- `NT:[1..n]`: Specify the number of threads for parallel searching (default is 6).

## Examples

### Basic Usage

Search for the pattern "example" in a file:

```bash
cargo run example path/to/yourfile.txt
```

### Case-Insensitive Search

Perform a case-insensitive search for "example" in a file:

```bash
IC=1 cargo run example path/to/yourfile.txt
```

### Non-Matching Results

Find lines in a file that do not contain "example":

```bash
NON=1 cargo run example path/to/yourfile.txt
```

### Search in Directories

Search for files containing "example" within a directory:

```bash
DIR=1 cargo run example path/to/yourdirectory
```

### Search Inside Files in Directories

Search inside files found in directories:

```bash
IN=1 cargo run example path/to/yourdirectory
```

### Parallel Searching

Enable parallel searching with a specified number of threads:

```bash
TH=1 NT:4 cargo run example path/to/yourdirectory
```
