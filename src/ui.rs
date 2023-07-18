use tui::layout::Rect;
#[allow(unused_imports)]
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span,Spans,Text},
    widgets::{Block, BorderType, Borders,Paragraph,Wrap},
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

    let text = vec![
        tui::text::Spans::from(tui::text::Span::raw(tim_msg)),
    ]; 
    let paragraph = Paragraph::new(text).wrap(Wrap { trim:  true });
    f.render_widget(paragraph, inner_area);
}



pub fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = chunks(f);

 
    //timer logic to be put into block

    let block = Block::default()
         .title("Configs")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    //Welcome logo

    let block = Block::default()
         .title("Welcome!")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[2]);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)  // Adjust this value to create a margin inside the Block.
        .split(chunks[2])[0];

    let style = Style::default()
        .fg(Color::Cyan)
        .bg(Color::Black);

     let logo = Text::styled(r#"                                                          
                                                              
          ,--,                                        ___     
  .--., ,--.'|                                      ,--.'|_   
,--.'  \|  | :     ,---.           .---.            |  | :,'  
|  | /\/:  : '    '   ,'\         /. ./|  .--.--.   :  : ' :  
:  : :  |  ' |   /   /   |     .-'-. ' | /  /    '.;__,'  /   
:  | |-,'  | |  .   ; ,. :    /___/ \: ||  :  /`./|  |   |    
|  : :/||  | :  '   | |: : .-'.. '   ' .|  :  ;_  :__,'| :    
|  |  .''  : |__'   | .; :/___/ \:     ' \  \    `. '  : |__  
'  : '  |  | '.'|   :    |.   \  ' .\     `----.   \|  | '.'| 
|  | |  ;  :    ;\   \  /  \   \   ' \ | /  /`--'  /;  :    ; 
|  : \  |  ,   /  `----'    \   \  |--" '--'.     / |  ,   /  
|  |,'   ---`-'              \   \ |      `--'---'   ---`-'   
`--'                          '---"                           
"#, style);
     let paragraph = Paragraph::new(logo).wrap(Wrap{ trim: false});
    
    f.render_widget(paragraph, inner_area);

    let block = Block::default()
         .title("Controls")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[3]);
}

