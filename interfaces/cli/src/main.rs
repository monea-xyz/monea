mod authentication;
mod cli;
mod commands;
mod utils;
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
        Some(cli::Commands::Run(args)) => {
            if let Err(e) = commands::run::run(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(cli::Commands::Stop(args)) => {
            if let Err(e) = commands::stop::stop(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(cli::Commands::Auth(args)) => {
            if let Err(e) = commands::auth::auth(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(cli::Commands::Billing(args)) => {
            if let Err(e) = commands::billing::billing(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            println!("No command provided. Use --help for usage information.");
        }
    }
}
