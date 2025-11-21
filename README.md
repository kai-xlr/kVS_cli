# CLI Key-Value Store (cli_kvs)

A simple, fast, and efficient command-line key-value store written in Rust. This project provides both a CLI interface for one-off commands and an interactive REPL mode for continuous operations.

## Features

- **In-memory storage**: Fast HashMap-based key-value storage
- **CLI interface**: Execute single commands directly from the command line
- **Interactive REPL**: Enter an interactive mode for multiple operations
- **Simple API**: Easy-to-use library interface
- **Type-safe**: Leverages Rust's type system for safety
- **Well-documented**: Comprehensive documentation for all public APIs

## Installation

### From Source

```bash
git clone https://github.com/yourusername/cli_kvs.git
cd cli_kvs
cargo build --release
```

The binary will be available at `target/release/cli_kvs`.

### Install Locally

```bash
cargo install --path .
```

## Usage

### CLI Commands

#### Set a key-value pair

```bash
cli_kvs set mykey myvalue
# Output: OK
```

#### Get a value by key

```bash
cli_kvs get mykey
# Output: myvalue
```

#### Delete a key

```bash
cli_kvs delete mykey
# Output: Deleted
```

#### List all key-value pairs

```bash
cli_kvs list
# Output:
# key1 = value1
# key2 = value2
```

### Interactive REPL Mode

Enter interactive mode for continuous operations:

```bash
cli_kvs repl
```

Available REPL commands:
- `set <key> <value>` - Set a key-value pair (supports multi-word values)
- `get <key>` - Get a value by key
- `delete <key>` - Delete a key-value pair
- `list` - List all key-value pairs
- `help` - Show help message
- `quit` or `exit` - Exit the REPL

#### Example REPL Session

```
$ cli_kvs repl
Entering kvs REPL. Type 'help' for commands or 'quit' to exit.
set name Alice
OK
set greeting Hello World
OK
get name
Alice
list
name = Alice
greeting = Hello World
delete name
Deleted
quit
Bye.
```

## Library Usage

You can also use `cli_kvs` as a library in your own Rust projects.

### Add to Cargo.toml

```toml
[dependencies]
cli_kvs = { path = "../cli_kvs" }  # Or use version from crates.io
```

### Example

```rust
use cli_kvs::KvStore;

fn main() {
    let mut store = KvStore::new();
    
    // Set values
    store.set("name", "Alice");
    store.set("age", "30");
    
    // Get values
    if let Some(name) = store.get("name") {
        println!("Name: {}", name);
    }
    
    // Delete values
    match store.delete("age") {
        Ok(()) => println!("Deleted successfully"),
        Err(e) => println!("Error: {}", e),
    }
    
    // Iterate over all entries
    for (key, value) in store.iter() {
        println!("{} = {}", key, value);
    }
}
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_set_and_get
```

## Development

### Project Structure

```
cli_kvs/
├── src/
│   ├── lib.rs      # Core KvStore implementation
│   ├── main.rs     # CLI entry point
│   ├── cli.rs      # CLI argument parsing
│   └── repl.rs     # REPL implementation
├── tests/
│   └── cli_tests.rs # Integration tests
├── Cargo.toml      # Project manifest
└── README.md       # This file
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Generate documentation
cargo doc --open
```

## Limitations

- **In-memory only**: Data is not persisted to disk
- **Single-threaded**: No concurrent access support
- **No transactions**: Operations are not atomic across multiple keys
- **String keys/values only**: Does not support arbitrary data types

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling
