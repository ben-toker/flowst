use flowst::{parse_args, Action};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{Terminal,backend::CrosstermBackend};
use std::{error::Error, io};
mod timer;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();

    match &args.command {
        Action::Start(arg)=> {
            timer::start_timer(arg.work.into(), arg.rest.into()).await;
            Ok(())
        },
        Action::App => {
            enable_raw_mode()?;
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

            let res = flowst::run_app(&mut terminal);
            // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }
}
}
