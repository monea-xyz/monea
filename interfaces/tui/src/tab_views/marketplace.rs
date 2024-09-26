use crate::model::Model;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_content(f: &mut Frame, model: &Model, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    let title = Paragraph::new("Rollup Module Marketplace")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let modules = vec![
        "ZK Rollup Module",
        "Optimistic Rollup Module",
        "Data Availability Module",
        "Fraud Proof Module",
        "Sequencer Module",
    ];

    let items: Vec<ListItem> = modules
        .iter()
        .map(|&module| ListItem::new(module).style(Style::default().fg(Color::White)))
        .collect();

    let modules_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Available Modules"),
        )
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Black))
        .highlight_symbol("> ");

    // TODO convert to stateful_widget
    // f.render_stateful_widget(modules_list, chunks[1], &mut app.marketplace_state);
    f.render_widget(modules_list, chunks[1]);
}
