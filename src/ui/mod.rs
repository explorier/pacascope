// UI components for the TUI interface
pub mod app;
pub mod dashboard;
pub mod charts;
pub mod widgets;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph, Tabs},
    text::{Span, Line},
    Frame,
};

use crate::ui::app::App;

pub fn render_main_layout(f: &mut Frame, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Main content area
            Constraint::Length(3), // Footer
        ].as_ref())
        .split(size);

    // Header
    let header = Paragraph::new("🦙 PacaScope v1.0 | Paper Trading Monitor | Live")
        .block(Block::default().borders(Borders::ALL).style(Style::default()));
    f.render_widget(header, chunks[0]);

    // Tabs
    let tab_titles: Vec<Line> = app.strategies.keys().map(|name| Line::from(Span::raw(name.clone()))).collect();
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().title("Strategies").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .divider(Span::raw("|"));
    f.render_widget(tabs, chunks[1]);
    
    // Main Content - Strategy Overviews
    let overview_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34), // Handles rounding
        ].as_ref())
        .margin(1)
        .split(chunks[2]);
        
    let strategy_keys: Vec<_> = app.strategies.keys().cloned().collect();
    for i in 0..overview_chunks.len() {
        if let Some(key) = strategy_keys.get(i) {
            if let Some(data) = app.strategies.get(key) {
                let widget = crate::ui::widgets::create_strategy_widget(data);
                f.render_widget(widget, overview_chunks[i]);
            }
        }
    }

    // Footer
    let footer = Paragraph::new("q:quit | tab:switch | r:refresh | a:alerts | s:settings")
        .block(Block::default().borders(Borders::ALL).style(Style::default()));
    f.render_widget(footer, chunks[3]);
} 