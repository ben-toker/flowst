use tui::layout::Rect;
#[allow(unused_imports)]
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};



pub fn chunks<B: Backend>(f: &mut Frame<B>) -> Vec<Rect>{
let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(60),
            ].as_ref()
        )
        .split(f.size());
chunks
}

pub fn tim_display<B: Backend>(f: &mut Frame<B>, mut receiver: tokio::sync::mpsc::Receiver<String>) -> tui::widgets::Paragraph {
    let chunks = chunks(f);
    let block = Block::default()
         .title("Timer")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let paragraph = tui::widgets::Paragraph::new("No current timer.");
    if let Some(countdown_string) = receiver.blocking_recv() {
        let text = vec![
            tui::text::Spans::from(tui::text::Span::raw(&countdown_string)),
        ]; 
    let paragraph = tui::widgets::Paragraph::new(text).wrap(tui::widgets::Wrap { trim: true });
    }
    paragraph
}

pub fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = chunks(f);

 
    //timer logic to be put into block

    let block = Block::default()
         .title("Controls")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
    let block = Block::default()
         .title("Configs")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
}

