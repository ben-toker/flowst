use clap::{Parser,Subcommand};
use crossterm::event::{self,Event, KeyCode};
use std::{io, thread::sleep};
pub mod ui;
pub mod timer;
pub mod config;

#[allow(unused_imports)]
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders,ListState},
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

enum Message {
    Quit,
    UpdateTimer,
}



pub async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut receiver: tokio::sync::mpsc::Receiver<String>) -> io::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let (selected_tx, selected_rx) = std::sync::mpsc::channel();
    
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    std::thread::spawn(move || {
        let mut selected = 0;
        loop {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => {
                        tx.send(Message::Quit).unwrap();
                    },
                    KeyCode::Char('p') => {
                        let mut timer_info = config::load_timer().unwrap();
                        
                        //If currently running
                        if timer_info.run_state {
                            // Update pause_time and run_state
                            timer_info.pause_time = Some(chrono::Utc::now());
                            timer_info.run_state = false;

                            config::save_timer(&timer_info).unwrap();

                            //Update message
                            tx.send(Message::UpdateTimer).unwrap();
                        }
                        //If timer is paused
                        else {
                            let mut timer_info = config::load_timer().unwrap();
                            timer_info.run_state = true;
                   
                            let start_work_elapsed = chrono::Utc::now().signed_duration_since(timer_info.start_work.unwrap());

                           if start_work_elapsed.num_seconds() <= timer_info.work_duration.num_seconds() {
                                timer_info.work_duration = timer_info.work_duration - timer_info.pause_time.unwrap().signed_duration_since(timer_info.start_work.unwrap());
                                timer_info.start_work = Some(chrono::Utc::now());
                           } else {
                                timer_info.rest_duration = timer_info.rest_duration - timer_info.pause_time.unwrap().signed_duration_since(timer_info.start_rest.unwrap());
                                timer_info.start_rest = Some(chrono::Utc::now());
                         };
                                
                            config::save_timer(&timer_info).unwrap();

                            //Update message
                            tx.send(Message::UpdateTimer).unwrap();

                        }
                    },

                    KeyCode::Up => {
                       if selected > 0 {
                            selected -= 1;
                       }
                       selected_tx.send(selected).unwrap();
                    },

                    KeyCode::Down => {
                        if selected < 2 {
                            selected += 1;
                        }
                        selected_tx.send(selected).unwrap();
                    },

                    _ => {},

            }
            }
        }
    });
    
    let mut timer_message: String;
    let mut changed = false;

    
    loop {
        let run_state = config::load_timer().unwrap().run_state;
        timer_message = if !changed {
            receiver.recv().await.unwrap_or_else(|| String::from("Please enter timer."))
        } else {
            if !run_state { 
                receiver.recv().await.unwrap_or_else(|| String::from("Please enter timer."))
            }
            else {
                "Paused.".to_string()
            }
        };
        match rx.try_recv() {
            Ok(Message::Quit) => break,
            Ok(Message::UpdateTimer) => {
                changed = true;
            },
            Err(_) => {},
        }

        if let Ok(selected) = selected_rx.try_recv() {
            list_state.select(Some(selected));
        }

        terminal.draw(|f| {
            ui::ui(f);
            ui::tim_display(f, &timer_message);
            ui::config_display(f, &mut list_state);
        })?;

      
       
    }

    Ok(())
}
