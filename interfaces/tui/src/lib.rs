use crate::message::Message;
use crate::model::Model;
use crate::poller::start_polling;
use crate::ui::ui;
use crate::update::update;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ctrlc;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::{Duration, Instant};
use update::MenuItemChange;

mod components;
mod message;
mod model;
mod poller;
mod tab_views;
mod ui;
mod update;

pub fn open_dashboard() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create initial model
    let mut model = Model::new();

    // Setup event loop
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    let (tx, rx) = channel();
    start_polling(tx.clone());

    loop {
        // Draw UI
        terminal.draw(|f| ui(f, &model))?;

        // Handle input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => {
                        update(&mut model, Message::TabChanged(MenuItemChange::Decrement))
                    }
                    KeyCode::Right => {
                        update(&mut model, Message::TabChanged(MenuItemChange::Increment))
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                        let index = c.to_digit(10).unwrap() as usize - 1;
                        update(
                            &mut model,
                            Message::TabChanged(MenuItemChange::Index(index)),
                        );
                    }
                    _ => {}
                }
            }
        }

        // Handle tick
        if last_tick.elapsed() >= tick_rate {
            update(&mut model, Message::Tick);
            last_tick = Instant::now();
        }

        // Handle messages from the poller
        if let Ok(message) = rx.try_recv() {
            update(&mut model, message);
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
