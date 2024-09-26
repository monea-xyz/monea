use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::model::Model;

pub fn render_content(f: &mut Frame, model: &Model, area: ratatui::layout::Rect) {
    // let content = match model
    //     .active_tab()
    //     .and_then(|items| items.get(model.active_sidebar_item_index()))
    //     .map(String::as_str)
    // {
    //     Some("Account") => "Account Settings",
    //     Some("General") => "General Settings",
    //     Some("Network") => "Network Settings",
    //     Some("Advanced") => "Advanced Settings",
    //     _ => "Unknown Settings Content",
    // };
    // let paragraph = Paragraph::new(content)
    //     .style(Style::default().fg(Color::White))
    //     .block(Block::default().borders(Borders::ALL).title("Settings"));
    // f.render_widget(paragraph, area);
    f.render_widget(Block::default(), area);
}
