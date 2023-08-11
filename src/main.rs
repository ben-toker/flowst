use chrono::{Utc, Duration};
use flowst::{parse_args, Action};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{Terminal,backend::CrosstermBackend};
use std::{error::Error, io};

use flowst::timer;

use flowst::config::{TimerInfo, load_timer, save_timer, reset_timer};

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
                pause_time: Some(Utc::now()),
                run_state: true
            };
            
            save_timer(&timer_info)?;

            let (mut rec, _) = timer::start_timer().await;
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

            let (rec, cancel) = timer::start_timer().await;
       
        flowst::run_app(&mut terminal, cancel, rec).await?;

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
