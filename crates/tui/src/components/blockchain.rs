use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct Blockchain {
    blocks: usize,
}

impl Blockchain {
    pub fn new(blocks: usize) -> Self {
        Blockchain { blocks }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block_size = 14; // Fixed block size
        let spacing = 1;
        let total_width = self.blocks * (block_size + spacing) - spacing;

        let blockchain_area = Rect::new(
            area.x + 2,
            area.y + 1,
            total_width.min((area.width - 4).into()) as u16,
            (block_size - (block_size / 2) - 1).try_into().unwrap(),
        );

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                (0..self.blocks)
                    .map(|_| Constraint::Length(block_size.try_into().unwrap()))
                    .collect::<Vec<Constraint>>(),
            )
            .spacing(spacing.try_into().unwrap());

        let blocks = horizontal_layout.split(blockchain_area);

        for (i, chunk) in blocks.into_iter().enumerate() {
            let is_last_block = i + 1 == self.blocks;
            let is_second_to_last = i + 1 == self.blocks - 1;

            let block = if is_last_block {
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .style(Style::default().fg(Color::White))
            } else if is_second_to_last {
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .style(Style::default().fg(Color::Yellow))
            } else {
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
            };

            let block = Paragraph::new(if i + 1 == 1 {
                "GENESIS".to_string()
            } else {
                (i + 1).to_string()
            })
            .alignment(ratatui::layout::Alignment::Center)
            .block(block);
            f.render_widget(block, *chunk);

            if i < self.blocks - 1 {
                let connector = Block::default()
                    .borders(Borders::TOP)
                    .style(Style::default().fg(Color::White));
                f.render_widget(
                    connector,
                    Rect::new(
                        chunk.right(),
                        chunk.y + (block_size / 4) as u16,
                        spacing as u16,
                        1,
                    ),
                );
            }
        }
    }
}
