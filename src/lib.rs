use clap::{Parser,Subcommand};
use std::sync::{Arc,Mutex};
use crossterm::event::{self,Event, KeyCode};
use std::io;
pub mod ui;
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
    
    let timer_info: config::TimerInfo = config::load_timer().unwrap();
    let run_state: Arc<Mutex<bool>> = Arc::new(Mutex::new(timer_info.run_state.clone()));
    let run_state_clone = run_state.clone();
    std::thread::spawn(move || {
        loop {
            if let Event::Key(key) = event::read().unwrap() {
                if let KeyCode::Char('q') = key.code {
                    tx.send(()).unwrap();
                    break;
                }

                if let KeyCode::Char('p') = key.code {
                    let mut timer_info = config::load_timer().unwrap();
                    
                    //If currently running
                    if timer_info.run_state {
                        // Update pause_time and run_state
                        timer_info.pause_time = Some(chrono::Utc::now());
                        timer_info.run_state = false;

                        let mut run_state = run_state_clone.lock().unwrap();
                        *run_state = false;

                        config::save_timer(&timer_info).unwrap();

                    }
                    //If timer is paused
                    else {
                        let mut timer_info = config::load_timer().unwrap();
                        timer_info.run_state = true;
                        let mut run_state = run_state_clone.lock().unwrap();
                        *run_state = true;

                        let start_work_elapsed = chrono::Utc::now().signed_duration_since(timer_info.start_work.unwrap());

                       if start_work_elapsed.num_seconds() <= timer_info.work_duration.num_seconds() {
                            timer_info.work_duration = timer_info.work_duration - timer_info.pause_time.unwrap().signed_duration_since(timer_info.start_work.unwrap());
                            timer_info.start_work = Some(chrono::Utc::now());
                       } else {
                            timer_info.rest_duration = timer_info.rest_duration - timer_info.pause_time.unwrap().signed_duration_since(timer_info.start_rest.unwrap());
                            timer_info.start_rest = Some(chrono::Utc::now());
                        };
                            
                        config::save_timer(&timer_info).unwrap();

                    }
                }
            }
        }
    });

    loop {
        let timer_message =  receiver.recv().await.unwrap_or_else(|| String::from("Please enter timer."));
    
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
