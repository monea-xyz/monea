use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

pub fn prompt_login_method<'a>(methods: &'a [&'a str]) -> Result<&'a str, io::Error> {
    // TODO prompts are bugged out rn, so just return the first method (email/password)
    return Ok(methods[0]);

    let mut selected = 0;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    loop {
        // Clear the screen and move cursor to top-left corner
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        println!("Select login method:");

        for (i, method) in methods.iter().enumerate() {
            if i == selected {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Yellow),
                    Print("-> "),
                    ResetColor,
                    Print(method),
                    Print("\n")
                )?;
            } else {
                println!("   {}", method);
            }
        }
        stdout.flush()?;

        // Use crossterm's event::read() to capture input events
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < methods.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    execute!(stdout, LeaveAlternateScreen)?;
                    return Ok(methods[selected]);
                }
                KeyCode::Char('q') => {
                    execute!(stdout, LeaveAlternateScreen)?;
                    return Err(io::Error::new(io::ErrorKind::Interrupted, "User quit"));
                }
                _ => {}
            }
        }
    }
}

pub fn prompt_input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn prompt_password(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let password = rpassword::read_password()?;
    Ok(password)
}
