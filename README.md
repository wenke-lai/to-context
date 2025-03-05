# To Context

A simple Rust CLI tool that collects and organizes the content of all files in a directory into a specific format.

## Features

- Recursively scans all files in a specified directory
- Organizes file contents into an AI context format
- Supports excluding specific files and directories (like `.git`, `node_modules`, etc.)
- Supports using wildcards to exclude file types (like `*.pyc`, `*.log`, etc.)
- Output filename is `{directory_name}.context.txt`

## Installation

### Method 1: Build from source

1. Make sure Rust and Cargo are installed (if not, [install Rust](https://www.rust-lang.org/tools/install) first)

2. Clone this repository

   ```bash
   git clone https://github.com/wenke-lai/to-context.git
   cd to-context
   ```

3. Build

   ```bash
   cargo build --release
   ```

4. Install to your local bin directory

   ```bash
   cp target/release/to-context ~/.local/bin/
   ```

   Make sure `~/.local/bin` is in your PATH environment variable. If not, add it:

   ```bash
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

### Method 2: Quick installation (Cargo)

```bash
cargo install --git https://github.com/your-username/context-collector.git
```

## Usage

```bash
to-context <directory_path>
```

For example:

```bash
to-context ~/my-project
```

This will generate a `my-project.context.txt` file containing the content of all files in the directory, formatted as follows:

```
# relative/path/1
File content 1

# relative/path/2
File content 2
```

## Exclusion Rules

By default, the following types of files and directories are excluded:

- Directories: `.git`, `node_modules`, `target`, `.idea`, `.vscode`, `dist`, `build`, `.venv`
- Files: `.DS_Store`, `Thumbs.db`, `uv.lock`
- File types: `*.pyc`, `*.class`, `*.o`, `*.so`, `*.dll`, `*.exe`, `*.jar`, `*.zip`, `*.tar.gz`, `*.log`

To customize exclusion rules, modify the `get_exclude_patterns()` function in the source code.

## Contributing

Contributions via Pull Requests or Issues are welcome to improve this tool.
