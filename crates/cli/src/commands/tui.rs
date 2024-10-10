use clap::Args;

#[derive(Args, Debug)]
pub struct TuiArgs {}

pub fn start_tui() -> Result<(), Box<dyn std::error::Error>> {
    monea_tui::open_dashboard()?;
    Ok(())
}
