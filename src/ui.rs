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

pub fn tim_display<B: Backend>(f: &mut Frame<B>, tim_msg: &str) {
    let chunks = chunks(f);
    let block = Block::default()
         .title("Timer")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    // Create a new area for the Paragraph that fits inside the Block.
    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)  // Adjust this value to create a margin inside the Block.
        .split(chunks[0])[0];

    let text = vec![
        tui::text::Spans::from(tui::text::Span::raw(tim_msg)),
    ]; 
    let paragraph = tui::widgets::Paragraph::new(text).wrap(tui::widgets::Wrap { trim: true });
    f.render_widget(paragraph, inner_area);
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

