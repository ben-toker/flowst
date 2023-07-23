use chrono::{Utc, Duration};
use flowst::{parse_args, Action};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{Terminal,backend::CrosstermBackend};
use std::{error::Error, io};

mod config;
use flowst::timer;

use self::config::{TimerInfo, load_timer, save_timer, reset_timer};

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();

    match &args.command {
        Action::Start(arg)=> {
            let work = arg.work.into();
            let rest = arg.rest.into();

            let timer_info = TimerInfo {
                start_work: Some(Utc::now()),
                work_duration: Duration::minutes(work),
                rest_duration: Duration::minutes(rest),
                start_rest: Some(Utc::now() + Duration::minutes(work)),
                run_state: true
            };
            
            save_timer(&timer_info)?;

            let mut rec = timer::start_timer(timer_info.work_duration,timer_info.rest_duration,timer_info.run_state).await;
            if let Some(message) = rec.recv().await {
                println!("Timer started. {} until break", message);
            }
            

            Ok(())
        },
        Action::Toggle => {
            let timer_info = load_timer()?;
            let new_timer: TimerInfo;
            if timer_info.run_state {
                new_timer = TimerInfo {
                    run_state: false,
                    ..timer_info
                }
            }
            else {
                new_timer = TimerInfo {
                    run_state: true,
                    ..timer_info
                }
            }

            save_timer(&new_timer)?;

            Ok(())
        },
        Action::Reset => {
            reset_timer();
            Ok(())
        },
        Action::App => {
            enable_raw_mode()?;
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

        let timer_info = load_timer()?;

        let start_work_elapsed = Utc::now().signed_duration_since(timer_info.start_work.unwrap());
        let start_rest_elapsed = Utc::now().signed_duration_since(timer_info.start_rest.unwrap());

            let rem_work = if start_work_elapsed.num_seconds() <=0 {timer_info.work_duration} else {timer_info.work_duration - start_work_elapsed};
    
            let rem_rest = if start_rest_elapsed.num_seconds() <=0 {timer_info.rest_duration} else {timer_info.rest_duration - start_rest_elapsed};
   
            let rec = timer::start_timer(rem_work,rem_rest,timer_info.run_state).await;
            flowst::run_app(&mut terminal,rec).await?;


            // restore terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
           terminal.show_cursor()?;


            Ok(())
        }
    }
}
