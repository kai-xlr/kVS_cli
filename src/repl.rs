use cli_kvs::KvStore;
use std::io::{self, BufRead, Write};

pub fn run_repl(mut store: KvStore) -> anyhow::Result<()> {
    println!("Entering kvs REPL. Type 'help' for commands or 'quit' to exit.");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }

        let cmd = parts[0];

        match cmd {
            "set" => {
                if parts.len() < 3 {
                    println!("Usage: set <key> <value>");
                    continue;
                }
                let key = parts[1];
                let value = parts[2..].join(" ");
                store.set(key, value);
                println!("OK");
            }
            "get" => {
                if parts.len() < 2 {
                    println!("Usage: get <key>");
                    continue;
                }
                let key = parts[1];
                match store.get(key) {
                    Some(value) => println!("{value}"),
                    None => println!("(nil)"),
                }
            }
            "delete" | "del" => {
                if parts.len() < 2 {
                    println!("Usage: delete <key>");
                    continue;
                }
                let key = parts[1];
                match store.delete(key) {
                    Ok(()) => println!("Deleted"),
                    Err(e) => println!("{e}"),
                }
            }
            "list" => {
                if store.is_empty() {
                    println!("(empty)");
                } else {
                    for (key, value) in store.iter() {
                        println!("{key} = {value}");
                    }
                }
            }
            "help" => {
                println!("Available commands:");
                println!("  set <key> <value>  - Set a key-value pair");
                println!("  get <key>          - Get a value by key");
                println!("  delete <key>       - Delete a key-value pair");
                println!("  list               - List all key-value pairs");
                println!("  help               - Show this help message");
                println!("  quit/exit          - Exit the REPL");
            }
            "quit" | "exit" => {
                println!("Bye.");
                return Ok(());
            }
            other => println!("Unknown command: {other}. Type 'help' for available commands."),
        }

        stdout.flush()?;
    }

    Ok(())
}
