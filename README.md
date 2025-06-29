# ClassLink Checker

[![CI](https://github.com/herring101/classlink-checker/workflows/CI/badge.svg)](https://github.com/herring101/classlink-checker/actions)
[![Coverage](https://codecov.io/gh/herring101/classlink-checker/branch/main/graph/badge.svg)](https://codecov.io/gh/herring101/classlink-checker)
[![Crates.io](https://img.shields.io/crates/v/classlink-checker.svg)](https://crates.io/crates/classlink-checker)

A powerful Rust-based command-line tool that analyzes class links and relationships in Markdown documentation files. Quickly identify isolated classes, analyze dependency patterns, and generate comprehensive statistics about your documentation structure.

## Features

- 🔍 **Smart Class Detection**: Automatically detects class definitions from Markdown headers
- 🔗 **Link Analysis**: Identifies both explicit markdown links and implicit class references
- 📊 **Comprehensive Statistics**: Provides detailed reports on class relationships
- 🏝️ **Isolation Detection**: Finds classes with no incoming or outgoing links
- 📈 **Multiple Output Formats**: Support for human-readable text and JSON output
- 🚀 **Fast & Efficient**: Built in Rust for maximum performance
- 🔄 **Recursive Scanning**: Optionally scan entire directory trees

## Installation

### From Releases (Recommended)

Download the latest binary from the [releases page](https://github.com/herring101/classlink-checker/releases):

```bash
# Linux/macOS
curl -L https://github.com/herring101/classlink-checker/releases/latest/download/classlink-checker-linux-amd64 -o classlink-checker
chmod +x classlink-checker
sudo mv classlink-checker /usr/local/bin/

# Windows
# Download classlink-checker-windows-amd64.exe from releases
```

### From Source

```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/herring101/classlink-checker.git
cd classlink-checker
cargo build --release
```

### From Crates.io

```bash
cargo install classlink-checker
```

## Quick Start

### Basic Usage

```bash
# Analyze current directory
classlink-checker .

# Analyze specific directory
classlink-checker ./docs

# Recursive analysis
classlink-checker -r ./project-docs

# JSON output for programmatic use
classlink-checker -o json ./docs
```

### Example Output

```text
=== Class Link Analysis Report ===

📊 Overall Statistics:
  Total Classes: 6
  Isolated Classes: 1

🏝️  Isolated Classes (no links):
  - IsolatedClass

🔗 Class Link Counts:
  📦 UserManager: 4 outgoing, 1 incoming
  📦 Logger: 0 outgoing, 4 incoming
  📦 AuthenticationService: 2 outgoing, 1 incoming
  📦 ValidationService: 1 outgoing, 1 incoming
  📦 DatabaseConnection: 1 outgoing, 1 incoming

🔝 Most Linking Class: UserManager (4 outgoing links)
🎯 Most Linked Class: Logger (4 incoming links)
```

## Command Line Options

```bash
USAGE:
    classlink-checker [OPTIONS] <PATH>

ARGS:
    <PATH>    Path to the directory containing Markdown files

OPTIONS:
    -h, --help                 Print help information
    -o, --output <FORMAT>      Output format: text, json [default: text]
    -r, --recursive            Recursively scan subdirectories
    -V, --version              Print version information
```

## Supported Patterns

The tool recognizes several patterns for class links:

### 1. Markdown Links
```markdown
# UserService

This service depends on [DatabaseConnection](DatabaseConnection.md) for data access.
```

### 2. Code References
```markdown
# UserManager

```rust
struct UserManager {
    db: DatabaseConnection,
    logger: Logger,
}
```

### 3. Inline Class References
```markdown
# AuthenticationService

The AuthenticationService works with UserManager to handle login operations.
```

## JSON Output Format

When using `-o json`, the tool outputs structured data:

```json
{
  "total_classes": 6,
  "isolated_classes": ["IsolatedClass"],
  "class_link_counts": {
    "UserManager": {
      "outgoing_links": 4,
      "incoming_links": 1
    },
    "Logger": {
      "outgoing_links": 0,
      "incoming_links": 4
    }
  },
  "most_linked_class": ["Logger", 4],
  "most_linking_class": ["UserManager", 4]
}
```

## Use Cases

- **Documentation Auditing**: Find orphaned or isolated classes in your docs
- **Architecture Analysis**: Understand class relationships and dependencies
- **Technical Debt**: Identify overly coupled or unused components
- **CI/CD Integration**: Automated documentation quality checks
- **Refactoring Planning**: Visualize impact of architectural changes

## Development

### Prerequisites

- Rust 1.70+ (2021 edition)
- Git

### Building

```bash
git clone https://github.com/herring101/classlink-checker.git
cd classlink-checker
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --verbose --all-features
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Future Enhancements

- **Multi-language Support**: Python, C#, TypeScript class detection
- **Visualization**: Generate dependency graphs and diagrams
- **Advanced Analytics**: Circular dependency detection, complexity metrics
- **Integration**: IDE plugins, webhook support
- **Export Formats**: CSV, GraphML, PlantUML outputs

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI powered by [clap](https://github.com/clap-rs/clap)
- Regular expressions via [regex](https://github.com/rust-lang/regex)
- Developed following TDD principles and Martin Fowler's refactoring guidelines

## Support

- 📚 [Documentation](https://github.com/herring101/classlink-checker/wiki)
- 🐛 [Issue Tracker](https://github.com/herring101/classlink-checker/issues)
- 💬 [Discussions](https://github.com/herring101/classlink-checker/discussions)

---

**Made with ❤️ by the ClassLink Checker team**