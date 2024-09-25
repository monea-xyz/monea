use crate::ui::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ctrlc;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod app;
mod components;
mod tab_views;
mod ui;

pub fn open_dashboard() -> Result<(), io::Error> {
    // Setup tui Terminal & Backend
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    // Create app state
    let mut app = app::App::new();

    // Setup Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        // Draw UI
        terminal.draw(|f| ui(f, &app))?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Left => app.change_active_tab(app::MenuItemChange::Decrement),
                KeyCode::Right => app.change_active_tab(app::MenuItemChange::Increment),
                KeyCode::Up => app.change_active_sidebar_item(app::MenuItemChange::Decrement),
                KeyCode::Down => app.change_active_sidebar_item(app::MenuItemChange::Increment),
                KeyCode::Tab => app.change_active_sidebar_item(app::MenuItemChange::Increment),
                KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                    let index = c.to_digit(10).unwrap() as usize - 1;
                    app.change_active_tab(index);
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
