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

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(_) => println!("Not implemented yet!"),
        _ => {
            let version = &*savepod_lib::VERSION;
            println!("Lib version is: {version}");
        }
    }
}
