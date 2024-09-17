mod cli;
mod commands;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        Some(cli::Commands::Init(args)) => {
            if let Err(e) = commands::init::init(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(cli::Commands::Run { project_path }) => {
            println!(
                "Run command not implemented yet. Project path: {:?}",
                project_path
            );
        }
        Some(cli::Commands::Stop { full }) => {
            println!("Stop command not implemented yet. Full stop: {}", full);
        }
        None => {
            println!("No command provided. Use --help for usage information.");
        }
    }
}
