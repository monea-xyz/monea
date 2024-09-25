use crate::app::{App, TabType};
use crate::tab_views::{baselayer, marketplace, rollup, settings};
use dark_light::detect;
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    // Set the background color of the entire terminal to black if light mode
    let system_theme = detect();
    let background_style = match system_theme {
        dark_light::Mode::Light => Style::default().bg(Color::Black),
        _ => Style::default(),
    };

    f.render_widget(Block::default().style(background_style), f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    let titles: Vec<String> = app
        .get_tabs()
        .iter()
        .enumerate()
        .map(|(i, view)| match &view.tab_type {
            TabType::Baselayer => format!(" Baselayer [{}] ", i + 1),
            TabType::Rollup(name) => format!(" {} [{}] ", name, i + 1),
            TabType::Marketplace => format!(" Marketplace [{}] ", i + 1),
            TabType::Settings => format!(" Settings [{}] ", i + 1),
        })
        .collect();

    let tabs = Tabs::new(
        titles
            .into_iter()
            .enumerate()
            .map(|(i, title)| {
                let (first, rest) = title.split_at(1);
                Span::styled(
                    format!("{}{}", first, rest),
                    if i == app.active_tab_index() {
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::UNDERLINED)
                    } else {
                        Style::default()
                    },
                )
            })
            .collect::<Vec<Span>>(),
    )
    .block(Block::default().borders(Borders::ALL).title("Views"))
    .select(app.active_tab_index())
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .bg(Color::Gray)
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
    );

    f.render_widget(tabs, chunks[0]);

    // Remove the inner_chunks layout and directly use chunks[1] for the main view
    let main_view = Block::default().title("Main View").borders(Borders::ALL);
    f.render_widget(main_view, chunks[1]);

    // Render the main content based on the current view
    render_main_view_content(f, app, chunks[1]);
}

fn render_main_view_content(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    match app.active_tab().tab_type {
        TabType::Baselayer => baselayer::render_content(f, app, area),
        TabType::Rollup(_) => rollup::render_content(f, app, area),
        TabType::Marketplace => marketplace::render_content(f, app, area),
        TabType::Settings => settings::render_content(f, app, area),
    }
}
