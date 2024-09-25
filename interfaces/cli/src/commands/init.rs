use clap::Args;
use monea_core::commands::init_handler;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[clap()]
    pub project_path: String,

    #[arg(long, short = 'n')]
    pub name: Option<String>,
}

pub fn init(args: InitArgs) -> Result<(), Box<dyn std::error::Error>> {
    init_handler::init_handler(&args.project_path, args.name)
}
