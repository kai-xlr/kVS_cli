use clap::{Parser, Subcommand};

/// CLI definition for kvs — works with clap
#[derive(Parser)]
#[command(name = "kvs", about = "A simple key-value store CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Set a key-value pair
    Set { key: String, value: String },

    /// Get a value by key
    Get { key: String },

    /// Delete a key
    Delete { key: String },

    /// List all keys and values
    List,

    /// Start interactive REPL mode
    Repl,
}
