use anyhow::Result;
use std::io::{self, Write};

pub fn yn_confirm(prompt: &str) -> Result<bool> {
    println!("{} (y/n)", prompt);
    let mut input = String::new();
    print!("Enter y/n: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase() == "y")
}
