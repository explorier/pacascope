// UI components for the TUI interface
pub mod app;
pub mod dashboard;
pub mod charts;
pub mod widgets;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_main_layout(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.size());

    // Header
    let header = Paragraph::new("🦙 PacaScope v1.0 | Paper Trading Monitor | Live")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Footer
    let footer = Paragraph::new("q:quit | tab:switch | r:refresh | a:alerts | s:settings")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
} 