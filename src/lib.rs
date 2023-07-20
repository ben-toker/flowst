use clap::{Parser,Subcommand};
use crossterm::event::{self,Event, KeyCode};
use std::io;
mod ui;
pub mod timer;
pub mod config;

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
    Toggle,
    App,
    Reset,
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
   let (tx, rx) = std::sync::mpsc::channel();
    
    std::thread::spawn(move || {
        loop {
            if let Event::Key(key) = event::read().unwrap() {
                if let KeyCode::Char('q') = key.code {
                    tx.send(()).unwrap();
                    break;
                }
            }
        }
    });

    loop {
        let timer_message = receiver.recv().await.unwrap_or_else(|| String::from("Please enter timer."));
        terminal.draw(|f| {
            ui::tim_display(f, &timer_message);
            ui::ui(f);
        })?;
    
        if rx.try_recv().is_ok() {
            break;
        }
    }

    Ok(())
}


