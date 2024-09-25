use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct ChainInfo {
    network_name: String,
    chain_id: String,
    gas_token: String,
    gas_token_decimals: String,
    network_type: String,
    settlement: String,
    data_availability: String,
    execution: String,
    category: String,
    block_time: String,
    gas_limit: String,
    gas_target: String,
}

impl ChainInfo {
    pub fn new(
        network_name: String,
        chain_id: String,
        gas_token: String,
        gas_token_decimals: String,
        network_type: String,
        settlement: String,
        data_availability: String,
        execution: String,
        category: String,
        block_time: String,
        gas_limit: String,
        gas_target: String,
    ) -> Self {
        ChainInfo {
            network_name,
            chain_id,
            gas_token,
            gas_token_decimals,
            network_type,
            settlement,
            data_availability,
            execution,
            category,
            block_time,
            gas_limit,
            gas_target,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).title("Chain Info");
        let inner_area = block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner_area);

        let left_items = vec![
            ("Network Name", &self.network_name),
            ("Chain ID", &self.chain_id),
            ("Gas Token", &self.gas_token),
            ("Gas Token Decimals", &self.gas_token_decimals),
            ("Type", &self.network_type),
            ("Settlement", &self.settlement),
        ];

        let right_items = vec![
            ("Data Availability", &self.data_availability),
            ("Execution (VM)", &self.execution),
            ("Category", &self.category),
            ("Block Time", &self.block_time),
            ("Gas Limit", &self.gas_limit),
            ("Gas Target", &self.gas_target),
        ];

        let left_text: Vec<Line> = left_items
            .into_iter()
            .map(|(key, value)| {
                Line::from(vec![
                    Span::styled(format!("{}: ", key), Style::default().fg(Color::Yellow)),
                    Span::raw(value),
                ])
            })
            .collect();

        let right_text: Vec<Line> = right_items
            .into_iter()
            .map(|(key, value)| {
                Line::from(vec![
                    Span::styled(format!("{}: ", key), Style::default().fg(Color::Yellow)),
                    Span::raw(value),
                ])
            })
            .collect();

        let left_paragraph = Paragraph::new(left_text).block(Block::default());
        let right_paragraph = Paragraph::new(right_text).block(Block::default());

        f.render_widget(block, area);
        f.render_widget(left_paragraph, chunks[0]);
        f.render_widget(right_paragraph, chunks[1]);
    }
}
