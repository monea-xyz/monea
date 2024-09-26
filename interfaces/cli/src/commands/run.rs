use clap::Args;
use monea_handlers::commands::run_handler;

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg()]
    pub project_path: Option<String>,
}

pub fn run(args: RunArgs) -> Result<(), Box<dyn std::error::Error>> {
    run_handler::run_handler(args.project_path)
}
