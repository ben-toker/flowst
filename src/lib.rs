use clap::{Parser,Subcommand};
use crossterm::event::{self,Event, KeyCode};
use std::io;
mod ui;

#[allow(unused_imports)]
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};



#[derive(Parser, Debug)]
#[command(
    name = "flowst",
    author = "Ben Toker <btoker.dev>",
    version = "1.0",
    about = r#"                                                            
                                                              
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

    Basic Pomodoro (Flow) timer in Rust."#,
    long_about = None,
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    ///Starts the timer. Uses -w and -r flags.
    Start(TimeArgs),
    App,
}

#[derive(Parser, Debug)]
pub struct TimeArgs {
    /// Work time.
    #[arg(short, long, default_value = "25")]
    pub work: u32,
    /// Rest time.
    #[arg(short, long, default_value = "5")]
    pub rest: u32,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut receiver: tokio::sync::mpsc::Receiver<String>) -> io::Result<()> {
    loop {
        let timer_message = receiver.recv().await.unwrap_or_else(|| String::from("No current timer."));
        terminal.draw(|f| {
            ui::tim_display(f, &timer_message);
            ui::ui(f);
        })?;

        //q for quit 
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}


