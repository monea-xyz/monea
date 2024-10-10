use clap::Args;
use monea_periphery::commands::run_handler;
use std::path::Path;

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg()]
    pub config_path: Option<String>,
}

pub fn run(args: RunArgs) -> Result<(), Box<dyn std::error::Error>> {
    run_handler::run_handler(args.config_path.as_deref().map(Path::new))
}
