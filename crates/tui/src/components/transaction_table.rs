use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

pub struct Transaction {
    hash: String,
    from: String,
    to: String,
    block: u64,
    status: String,
}

pub struct TransactionTable {
    transactions: Vec<Transaction>,
    state: TableState,
}

impl TransactionTable {
    pub fn new() -> Self {
        TransactionTable {
            transactions: generate_dummy_transactions(14),
            state: TableState::default(),
        }
    }

    pub fn update(&mut self, new_transaction: Transaction) {
        self.transactions.push(new_transaction);
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let header_cells = ["Hash", "From", "To", "Block", "Status"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
        let header = Row::new(header_cells)
            .style(Style::default())
            .height(1)
            .bottom_margin(1);

        let rows = self.transactions.iter().map(|t| {
            Row::new(vec![
                Cell::from(t.hash.as_str()),
                Cell::from(t.from.as_str()),
                Cell::from(t.to.as_str()),
                Cell::from(t.block.to_string()),
                Cell::from(t.status.as_str()),
            ])
        });

        let table = Table::new(
            rows.collect::<Vec<_>>(),
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(10),
                Constraint::Percentage(15),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Recent Transactions [T]"),
        );

        f.render_stateful_widget(table, area, &mut self.state);
    }
}

// This function can be used for testing or initial data population
pub fn generate_dummy_transactions(count: usize) -> Vec<Transaction> {
    let base_transactions = vec![
        Transaction {
            hash: "0x1234...abcd".to_string(),
            from: "0xABCD...5678".to_string(),
            to: "0xEFGH...9012".to_string(),
            block: 1000,
            status: "Confirmed".to_string(),
        },
        Transaction {
            hash: "0x5678...efgh".to_string(),
            from: "0xIJKL...3456".to_string(),
            to: "0xMNOP...7890".to_string(),
            block: 1001,
            status: "Pending".to_string(),
        },
        Transaction {
            hash: "0x90ab...cdef".to_string(),
            from: "0xQRST...1234".to_string(),
            to: "0xUVWX...5678".to_string(),
            block: 1002,
            status: "Confirmed".to_string(),
        },
    ];

    let mut transactions = Vec::new();
    for i in 0..count {
        for transaction in &base_transactions {
            let new_transaction = Transaction {
                hash: transaction.hash.clone(),
                from: transaction.from.clone(),
                to: transaction.to.clone(),
                block: transaction.block + i as u64 * 3,
                status: transaction.status.clone(),
            };
            transactions.push(new_transaction);
        }
    }

    transactions
}
