use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct ContainerUrl {
    pub key: String,
    pub value: String,
}

pub struct Container {
    pub name: String,
    pub status: String,
    pub urls: Vec<ContainerUrl>,
}

pub struct UrlsComponent {
    containers: Vec<Container>,
}

impl UrlsComponent {
    pub fn new(containers: Vec<Container>) -> Self {
        UrlsComponent { containers }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).title("URLs");
        let inner_area = block.inner(area);

        let mut lines = Vec::new();

        for container in &self.containers {
            let status_style = match container.status.as_str() {
                "RUNNING" => Style::default().fg(Color::Green),
                "STOPPED" => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::Yellow),
            };

            lines.push(Line::from(vec![
                Span::styled(
                    &container.name,
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - "),
                Span::styled(&container.status, status_style),
            ]));

            for url in &container.urls {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(&url.key, Style::default().fg(Color::Cyan)),
                    Span::raw(": "),
                    Span::raw(&url.value),
                ]));
            }

            lines.push(Line::from(""));
        }

        let urls_paragraph = Paragraph::new(lines).block(Block::default());

        f.render_widget(block, area);
        f.render_widget(urls_paragraph, inner_area);
    }
}
