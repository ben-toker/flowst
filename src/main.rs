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
        let mut receiver = timer::start_timer(arg.work.into(), arg.rest.into()).await;
        while let Some(message) = receiver.recv().await {
           print!("{}", message);
        }
        Ok(())
        },
        Action::App => {
            enable_raw_mode()?;
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;
            
            let rec = timer::start_timer(25, 5).await;
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
