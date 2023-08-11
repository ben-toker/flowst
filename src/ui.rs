use tui::layout::Rect;
#[allow(unused_imports)]
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span,Spans,Text},
    widgets::{Block, BorderType, Borders,Paragraph,List,ListItem,ListState,Wrap},
    Frame, Terminal,
};

pub fn chunks<B: Backend>(f: &mut Frame<B>) -> Vec<Rect>{
let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ].as_ref()
        )
        .split(f.size());

let lower_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .margin(1)
    .constraints(
        [
        Constraint::Percentage(20),
        Constraint::Percentage(80),
        ].as_ref()
    )
    .split(chunks[1]);

vec![chunks[0],lower_chunks[0], lower_chunks[1], chunks[2]]
}

pub fn tim_display<B: Backend>(f: &mut Frame<B>, tim_msg: &str) {
    let chunks = chunks(f);
    let block = Block::default()
         .title("Timer")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)  
        .split(chunks[0])[0];

    let style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::ITALIC);

    let text = vec![
        tui::text::Spans::from(Span::styled(tim_msg,style)),
    ]; 
    let paragraph = Paragraph::new(text).wrap(Wrap { trim:  true });
    f.render_widget(paragraph, inner_area);
}

pub fn config_display<B: Backend>(f: &mut Frame<B>, selected: &mut tui::widgets::ListState) {
    let chunks = chunks(f);

  //Config block
    let block = Block::default()
         .title("Configs")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                     Constraint::Percentage(10),
                     Constraint::Percentage(90),
        ].as_ref())
        .margin(1)
        .split(chunks[1])[1];
    
    let items = [ListItem::new("25 : 5"),ListItem::new("50 : 10"),ListItem::new("60 : 15")];
      let conflist = List::new(items)
        .block(Block::default().title("Timers").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
            .add_modifier(Modifier::ITALIC)
            .add_modifier(Modifier::BOLD)
            .fg(Color::Blue)
            )
        .highlight_symbol(">>");

    f.render_stateful_widget(conflist,inner_area, selected);

}

pub fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = chunks(f); 
    
  
    //Welcome logo

    let block = Block::default()
         .title("Welcome!")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[2]);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)  
        .split(chunks[2])[0];

    let style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

     let logo = Text::styled(r#" 
   __ _                   _   
  / _| |                 | |  
 | |_| | _____      _____| |_ 
 |  _| |/ _ \ \ /\ / / __| __|
 | | | | (_) \ V  V /\__ \ |_ 
 |_| |_|\___/ \_/\_/ |___/\__|                                                     
"#, style);
     let paragraph = Paragraph::new(logo).wrap(Wrap{ trim: false});
    
    f.render_widget(paragraph, inner_area);

    //Controls block
    let block = Block::default()
         .title("Controls")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[3]);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)  
        .split(chunks[3])[0];

    let text = vec![
        Spans::from(Span::styled("q - quit | p - pause/resume",style)),
    ]; 
    let paragraph = Paragraph::new(text).wrap(Wrap { trim:  true });
    f.render_widget(paragraph, inner_area);

}

