# grain
Godot Rust Addons Installation Ninja

A CLI tool for managing Rust addons in Godot projects. This tool helps you easily install, manage, and update Rust addons for your Godot games.

## Installation

```bash
# Clone the repository
git clone https://github.com/marconvcm/grain.git
cd grain

# Build the project
cargo build --release

# The binary will be available at target/release/grain
```

## Usage

### Initialize a new Godot Rust addon project
```bash
grain init --name my-addon --path ./my-addon-directory
```

### Install a Rust addon from a repository
```bash
grain install some-addon-name
grain install some-addon-name --version 1.2.3
```

### List installed addons
```bash
grain list
grain list --verbose  # Show detailed information
```

### Remove an installed addon
```bash
grain remove addon-name
grain remove addon-name --force  # Skip confirmation
```

### Update installed addons
```bash
grain update --all           # Update all addons
grain update specific-addon  # Update a specific addon
```

### Get help
```bash
grain --help
grain <command> --help  # Get help for a specific command
```

## Development

This project is built with Rust. To contribute:

1. Install Rust: https://rustup.rs/
2. Clone the repository
3. Run `cargo build` to build the project
4. Run `cargo test` to run tests
5. Run `cargo run -- --help` to test the CLI

## License

MIT License - see [LICENSE](LICENSE) file for details.