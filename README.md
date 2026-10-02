# ptree

ptree is a CLI tool for viewing and exploring project structure. It allows you to:

- Display project structure as a tree.
- Explore a specific directory or limit the displayed depth.
- Apply `.gitignore` and custom ignore rules.
- Filter entries by type or file extension.
- Include hidden files and directories when needed.
- Display project statistics.
- Output the structure in tree, compact, or JSON formats.

The tool is designed as a simple filesystem utility with sensible defaults for common project files, directories, and generated artifacts.

## Technologies

- [Rust](https://www.rust-lang.org/) — systems programming language used to build the CLI.
- [Clap](https://docs.rs/clap/latest/clap/) — command-line argument parsing.
- [Serde](https://serde.rs/) — serialization support for JSON output.
- [ignore](https://docs.rs/ignore/latest/ignore/) — `.gitignore`-compatible ignore rules.

## Build & Installation

1. Clone the repository

```bash
git clone https://github.com/NikitaBerezhnyj/ptree.git
cd ptree
```

2. Build the project

```bash
cargo build --release
```

3. Install the binary file (optional):

```bash
cargo install --path .
```

## Usage

Run `ptree` from a project directory:

```bash
ptree
```

Or provide a specific path:

```bash
ptree src
```

### Depth

Limit the displayed directory depth:

```bash
ptree --depth 3
```

### Ignore rules

`.gitignore` rules are applied automatically.

To disable them:

```bash
ptree --no-gitignore
```

Add custom ignore patterns:

```bash
ptree --ignore "*.generated.dart"
ptree --ignore "*.dart" "*.yaml"
ptree --ignore "*.dart,*.yaml"
```

Multiple patterns can be separated by spaces or commas.

When using multiple values, specify `PATH` before `--ignore`:

```bash
ptree ./my_project --ignore "*.dart" "*.yaml"
```

### Filters

Show only directories:

```bash
ptree --dirs-only
```

Show only files:

```bash
ptree --files-only
```

Filter files by extension:

```bash
ptree --ext dart
ptree --ext dart yaml
ptree --ext dart,yaml
```

Multiple extensions can be separated by spaces or commas.

### Hidden files

Include hidden files and directories:

```bash
ptree --hidden
```

### Statistics

Display project statistics:

```bash
ptree --stats
```

### Output formats

Choose the output format with `--format`:

```bash
ptree --format tree
ptree --format compact
ptree --format json
```

- `tree` — hierarchical tree representation.
- `compact` — flat list of project paths.
- `json` — machine-readable project structure.

### Configuration

Create a default `.ptreerc` configuration file:

```bash
ptree --init
```

The file is created in the target directory and is not overwritten if it already exists.

`.ptreerc` uses TOML:

```toml
depth = 3
hidden = false
gitignore = true
stats = false
dirs_only = false
files_only = false
format = "tree"
ext = ["rs", "toml"]
ignore = ["target", "*.generated.rs"]
```

Available options:

- `depth` — maximum directory depth.
- `hidden` — show hidden files and directories.
- `gitignore` — apply `.gitignore` rules.
- `stats` — show project statistics.
- `dirs_only` / `files_only` — filter by entry type.
- `format` — `tree`, `compact`, or `json`.
- `ext` — file extensions to include.
- `ignore` — additional ignore patterns.

CLI options take priority over `.ptreerc`. `--ext` replaces the configured list, while `--ignore` adds to it.

By default, ptree searches for `.ptreerc` from the target directory up through its parent directories. Configuration can be disabled with:

```bash
ptree --no-config
```

Or loaded from a specific file:

```bash
ptree --config path/to/.ptreerc
```

### Short options

Most frequently used options have short aliases:

| Option         | Short |
| -------------- | ----- |
| `--depth`      | `-d`  |
| `--ignore`     | `-i`  |
| `--dirs-only`  | `-D`  |
| `--files-only` | `-f`  |
| `--ext`        | `-e`  |
| `--hidden`     | `-H`  |
| `--stats`      | `-s`  |

For example:

```bash
ptree -d 3
ptree -i "*.dart" "*.yaml"
ptree -f
ptree -e dart yaml
ptree -H
ptree -s
```

## License & Community Guidelines

- [MIT License](LICENSE) — project license.
- [Code of Conduct](CODE_OF_CONDUCT.md) — expected behavior for contributors.
- [Contributing Guide](CONTRIBUTING.md) — how to help the project.
- [Security Policy](SECURITY.md) — reporting security issues.
