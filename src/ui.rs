use ratatui::layout::Rect;
#[allow(unused_imports)]
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn chunks(f: &mut Frame) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(70),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.area());

    let lower_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);

    vec![chunks[0], lower_chunks[0], lower_chunks[1], chunks[2]]
}

pub fn tim_display(f: &mut Frame, tim_msg: &str) {
    let chunks = chunks(f);
    let block = Block::default().title("Timer").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(chunks[0])[0];

    let style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::ITALIC);

    let text = vec![Line::from(Span::styled(tim_msg, style))];
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
    f.render_widget(paragraph, inner_area);
}

pub fn config_display(f: &mut Frame, selected: &mut ListState) {
    let chunks = chunks(f);

    //Config block
    let block = Block::default().title("Configs").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .margin(1)
        .split(chunks[1])[1];

    let items = [
        ListItem::new("25 : 5"),
        ListItem::new("50 : 10"),
        ListItem::new("60 : 15"),
    ];
    let conflist = List::new(items)
        .block(Block::default().title("Timers").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        )
        .highlight_symbol(">>");

    f.render_stateful_widget(conflist, inner_area, selected);
}

pub fn ui(f: &mut Frame) {
    let chunks = chunks(f);

    //Welcome block

    let block = Block::default().title("Welcome!").borders(Borders::ALL);
    f.render_widget(block, chunks[2]);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(chunks[2])[0];

    let style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let logo = Text::styled(
        r#" 
   __ _                   _   
  / _| |                 | |  
 | |_| | _____      _____| |_ 
 |  _| |/ _ \ \ /\ / / __| __|
 | | | | (_) \ V  V /\__ \ |_ 
 |_| |_|\___/ \_/\_/ |___/\__|                                                     
"#,
        style,
    );
    let paragraph = Paragraph::new(logo).wrap(Wrap { trim: false });

    f.render_widget(paragraph, inner_area);

    //Controls block
    let block = Block::default().title("Controls").borders(Borders::ALL);
    f.render_widget(block, chunks[3]);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(chunks[3])[0];

    let text = vec![Line::from(Span::styled(
        "q - quit | p - pause/resume",
        style,
    ))];
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
    f.render_widget(paragraph, inner_area);
}
