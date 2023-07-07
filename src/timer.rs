use tokio::time::{sleep,Duration};
use std::io::{stdout,Write};

//flushes stdout to update the text in realtime.
pub async fn countdown(seconds: u64) {
    let mut stdout = stdout();
    for i in (0..=seconds).rev() {
          write!(&mut stdout, "\r{} seconds remaining ",i).unwrap();
          stdout.flush().unwrap();
          sleep(Duration::from_secs(1)).await;
        }
    println!("Session complete.");
}

pub async fn start_timer(work: u64, rest: u64) {
    println!("Starting work session.");
    countdown(work).await;
    println!("Session ended. Break starting.");
    countdown(rest).await;
}

