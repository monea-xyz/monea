use clap::Args;
use monea_handlers::commands::stop_handler;

#[derive(Args, Debug)]
pub struct StopArgs {
    #[arg()]
    pub project_path: Option<String>,

    #[arg(short, long)]
    pub layer: String,
}

pub fn stop(args: StopArgs) -> Result<(), Box<dyn std::error::Error>> {
    stop_handler::stop_handler(args.project_path, args.layer)
}
