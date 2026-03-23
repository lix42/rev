pub mod comment_editor;
pub mod comment_panel;
pub mod diff_view;
pub mod file_panel;
pub mod mode_selector;
pub mod status_bar;

use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;

pub fn render(frame: &mut Frame, _state: &AppState) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(area);

    // Main content area
    let body = Paragraph::new("rev — TUI code review tool")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" rev "));
    frame.render_widget(body, chunks[0]);

    // Status bar
    let status = Paragraph::new(Line::from(" Press q to quit"))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));
    frame.render_widget(status, chunks[1]);
}
