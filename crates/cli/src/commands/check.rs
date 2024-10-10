use clap::Args;
use monea_periphery::commands::check_handler;

#[derive(Args, Debug)]
pub struct CheckArgs {
    #[arg(help = "Path to the monea.config.yaml file")]
    pub config_path: String,
}

pub fn check(args: CheckArgs) -> Result<(), Box<dyn std::error::Error>> {
    check_handler::check_handler(&args.config_path)
}
