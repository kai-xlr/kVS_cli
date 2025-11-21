mod cli;
mod repl;

use clap::Parser;
use cli::{Cli, Command};
use cli_kvs::KvStore;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut store = KvStore::new();

    match cli.command {
        Command::Set { key, value } => {
            store.set(key, value);
            println!("OK");
        }
        Command::Get { key } => match store.get(&key) {
            Some(value) => println!("{value}"),
            None => println!("(nil)"),
        },
        Command::Delete { key } => match store.delete(&key) {
            Ok(()) => println!("Deleted"),
            Err(e) => println!("{e}"),
        },
        Command::List => {
            if store.is_empty() {
                println!("(empty)");
            } else {
                for (key, value) in store.iter() {
                    println!("{key} = {value}");
                }
            }
        }
        Command::Repl => {
            repl::run_repl(store)?;
        }
    }

    Ok(())
}
