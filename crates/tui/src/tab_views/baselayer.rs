use crate::components::blockchain::Blockchain;
use crate::components::chain_info::ChainInfo;
use crate::components::transaction_table::TransactionTable;
use crate::components::urls::{Container, ContainerUrl, UrlsComponent};
use crate::model::Model;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_content(f: &mut Frame, model: &Model, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Top horizontal rectangle
            Constraint::Min(0),     // Remaining area
        ])
        .split(area);

    let top_area = layout[0];
    let remaining_area = layout[1];

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Left column
            Constraint::Percentage(40), // Right column
        ])
        .split(remaining_area);

    let left_column = main_layout[0];
    let right_column = main_layout[1];

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20), // Upper box
            Constraint::Percentage(80), // Lower box
        ])
        .split(right_column);

    let upper_right = right_layout[0];
    let lower_right = right_layout[1];

    // Render blockchain in the top area with a title and borders
    let blockchain_block = Block::default()
        .borders(Borders::ALL)
        .title("Blockchain [C]");
    let blockchain_inner = blockchain_block.inner(top_area);
    f.render_widget(blockchain_block, top_area);

    let mut blockchain = Blockchain::new(13);
    blockchain.render(f, blockchain_inner);

    let mut transaction_table = TransactionTable::new();
    transaction_table.render(f, left_column);

    let chain_info = ChainInfo::new(
        "Ethereum".to_string(),
        "1".to_string(),
        "ETH".to_string(),
        "18".to_string(),
        "Mainnet".to_string(),
        "L1".to_string(),
        "On-chain".to_string(),
        "EVM".to_string(),
        "L1".to_string(),
        "12 seconds".to_string(),
        "30,000,000".to_string(),
        "15,000,000".to_string(),
    );
    chain_info.render(f, upper_right);

    let containers = vec![
        Container {
            name: "EL Client (reth)".to_string(),
            status: "RUNNING".to_string(),
            urls: vec![
                ContainerUrl {
                    key: "engine-rpc".to_string(),
                    value: "127.0.0.1:61964".to_string(),
                },
                ContainerUrl {
                    key: "metrics".to_string(),
                    value: "http://127.0.0.1:61965".to_string(),
                },
                ContainerUrl {
                    key: "rpc".to_string(),
                    value: "127.0.0.1:61962".to_string(),
                },
                ContainerUrl {
                    key: "tcp-discovery".to_string(),
                    value: "127.0.0.1:61961".to_string(),
                },
                ContainerUrl {
                    key: "udp-discovery".to_string(),
                    value: "127.0.0.1:56899".to_string(),
                },
                ContainerUrl {
                    key: "ws".to_string(),
                    value: "127.0.0.1:61963".to_string(),
                },
            ],
        },
        Container {
            name: "CL Client (lighthouse)".to_string(),
            status: "STOPPED".to_string(),
            urls: vec![
                ContainerUrl {
                    key: "http".to_string(),
                    value: "http://127.0.0.1:61968".to_string(),
                },
                ContainerUrl {
                    key: "metrics".to_string(),
                    value: "http://127.0.0.1:61969".to_string(),
                },
                ContainerUrl {
                    key: "tcp-discovery".to_string(),
                    value: "127.0.0.1:61970".to_string(),
                },
                ContainerUrl {
                    key: "udp-discovery".to_string(),
                    value: "127.0.0.1:63513".to_string(),
                },
            ],
        },
        Container {
            name: "Validator Client (prysm)".to_string(),
            status: "RUNNING".to_string(),
            urls: vec![ContainerUrl {
                key: "metrics".to_string(),
                value: "http://127.0.0.1:61976".to_string(),
            }],
        },
        Container {
            name: "Commit-boost Sidecar".to_string(),
            status: "RUNNING".to_string(),
            urls: vec![
                ContainerUrl {
                    key: "metrics".to_string(),
                    value: "http://127.0.0.1:61948".to_string(),
                },
                ContainerUrl {
                    key: "http".to_string(),
                    value: "http://127.0.0.1:61949".to_string(),
                },
                ContainerUrl {
                    key: "ws".to_string(),
                    value: "ws://127.0.0.1:61950".to_string(),
                },
            ],
        },
        Container {
            name: "Preconfirmation Service (Custom commit-boost module)".to_string(),
            status: "RUNNING".to_string(),
            urls: vec![
                ContainerUrl {
                    key: "metrics".to_string(),
                    value: "http://127.0.0.1:61998".to_string(),
                },
                ContainerUrl {
                    key: "http".to_string(),
                    value: "http://127.0.0.1:61992".to_string(),
                },
                ContainerUrl {
                    key: "ws".to_string(),
                    value: "ws://127.0.0.1:61994".to_string(),
                },
            ],
        },
        Container {
            name: "Telegram Notifier (Custom commit-boost module)".to_string(),
            status: "STOPPED".to_string(),
            urls: vec![
                ContainerUrl {
                    key: "metrics".to_string(),
                    value: "http://127.0.0.1:61988".to_string(),
                },
                ContainerUrl {
                    key: "http".to_string(),
                    value: "http://127.0.0.1:61989".to_string(),
                },
                ContainerUrl {
                    key: "ws".to_string(),
                    value: "ws://127.0.0.1:61990".to_string(),
                },
            ],
        },
    ];

    let urls_component = UrlsComponent::new(containers);
    urls_component.render(f, lower_right);
}
