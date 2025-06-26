// Custom TUI widgets. 

use crate::data::StrategyData;
use ratatui::{
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line, Text},
};

/// Creates a widget to display the overview for a single strategy.
pub fn create_strategy_widget<'a>(strategy_data: &'a StrategyData) -> Paragraph<'a> {
    let text = Text::from(vec![
        Line::from(vec![
            Span::styled(format!("${:.2}", strategy_data.portfolio_value), Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("Buying Power: "),
            Span::styled(format!("${:.2}", strategy_data.buying_power), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::raw("Day Trades: "),
            Span::raw(format!("{}/3", strategy_data.day_trades_used)),
        ]),
        Line::from(vec![
            Span::raw("Positions: "),
            Span::raw(format!("{}", strategy_data.positions.len())),
        ]),
    ]);

    Paragraph::new(text)
        .block(
            Block::default()
                .title(Span::styled(
                    strategy_data.name.as_str(),
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
        )
} 