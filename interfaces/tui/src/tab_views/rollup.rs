use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_content(f: &mut Frame, app: &App, area: Rect) {
    let paragraph = Paragraph::new("content".to_string())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.active_tab().title.clone()),
        );
    f.render_widget(paragraph, area);
}
