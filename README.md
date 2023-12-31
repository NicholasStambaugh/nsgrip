# NSGrip

[![Build Status](https://travis-ci.org/nicholasstambaugh/nsgrip.svg?branch=master)](https://travis-ci.org/nicholasstambaugh/nsgrip) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Recreating `grep` Command-Line bash expression for native Windows/mac users.

# Example
```bash

cargo run -- Word poem.txt > output.txt

```

# About
NSGrip is a Rust command-line utility that provides powerful text-searching capabilities within files. 

It is designed to be fast, efficient, and easy to use. 

This project is under active development, and contributions are welcome!

## Features

- **Text Search:** Search for a specific query within the content of files.

- **Error Handling:** Graceful handling of errors to ensure a smooth user experience.

- **Cross-Platform:** Works on Windows, macOS, and Linux.

## Table of Contents

- [Installation](#installation)

- [Usage](#usage)

- [Configuration](#configuration)

- [Contributing](#contributing)

- [License](#license)

## Installation

### Prerequisites

- Rust: [Installation Guide](https://www.rust-lang.org/tools/install)

### Build from Source

Clone the repository:

```bash

git clone https://github.com/nicholasstambaugh/nsgrep.git

cd nsgrep

```

Build the project:

```bash

cargo build --release

```

The binary will be available in the `target/release` directory.

## Usage

NSGrep is a command-line tool. Here are some examples of how to use it:

```bash

# Basic Usage
# 'to' can be any argument (user input)

cargo run -- to poem.txt > output.txt

```
This will output the rows of text that contain 'to' into `output.txt`

For more detailed usage instructions, refer to the [Wiki](https://github.com/your-username/nsgrep/wiki).

## Configuration

NSGrep uses a configuration structure to handle its settings. You can configure the search parameters through command-line arguments.

## Contributing

We welcome contributions from the community. To contribute to NSGrep, follow these steps:

1\. Fork the repository.

2\. Create a feature branch: `git checkout -b feature-name`.

3\. Commit your changes: `git commit -am 'Add feature'`.

4\. Push to the branch: `git push origin feature-name`.

5\. Submit a pull request.
