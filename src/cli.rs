use std::io::Error;

use crate::prelude;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Savepod")]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Scan,
    Backup,
    Restore,
}

fn handle_commands() {
    println!("Nothing implemented yet!");
}

fn handle_version() {
    let version = &*prelude::VERSION;
    println!("Savepod version {version}");
}

pub fn run() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        Some(_) => handle_commands(),
        _ => handle_version(),
    }
    Ok(())
}
