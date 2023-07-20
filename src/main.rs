use flowst::{parse_args, Action};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{Terminal,backend::CrosstermBackend};
use std::{error::Error, io, time::{SystemTime, Duration}};
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
               start_time: SystemTime::now(), 
               work_duration: Duration::new(work, 0), 
               rest_duration: Duration::new(rest, 0),
            };
            
            save_timer(&timer_info)?;

            let mut receiver = timer::start_timer(timer_info.work_duration,timer_info.rest_duration).await;
            if let Some(message) = receiver.recv().await {
                println!("Timer started. {} until break", message);
            }
            Ok(())
        },
        Action::Toggle => {
            let (pause_sender, _) = tokio::sync::mpsc::channel(1);
            timer::pause(pause_sender).await;
            Ok(())
        },
        Action::Reset => {
            reset_timer()?;
            Ok(())
        },
        Action::App => {
            enable_raw_mode()?;
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

            let timer_info = load_timer()?;

            let rem_work = timer_info.work_duration - timer_info.start_time.elapsed().unwrap();
            let rem_rest = (timer_info.work_duration + timer_info.rest_duration) - timer_info.start_time.elapsed().unwrap();


            if timer_info.start_time != SystemTime::now() && (rem_work.as_secs() == 0) && (rem_work.as_secs() == 0) {
                config::reset_timer()?;
            }


            let rec = timer::start_timer(rem_work, rem_rest).await;

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
