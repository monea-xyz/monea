use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::model::Model;

pub fn render_content(f: &mut Frame, model: &Model, area: Rect) {
    let paragraph = Paragraph::new("content".to_string())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(model.active_tab().title.clone()),
        );
    f.render_widget(paragraph, area);
}
