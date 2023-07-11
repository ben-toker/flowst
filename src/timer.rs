use tokio::time::{sleep,Duration};
use std::io::{stdout,Write};

fn format_seconds(seconds: u64) -> Vec<u64>{
       return vec![(seconds/60),(seconds%60)];   
}


//flushes stdout to update the text in realtime.
async fn countdown(seconds: u64) {
    let mut stdout = stdout();
    for i in (0..=(seconds * 60)).rev() {
        let (minutes,seconds) = (format_seconds(i)[0],format_seconds(i)[1]);
          write!(&mut stdout,"\r{} minutes and {} seconds remaining ",minutes, seconds).unwrap();
          stdout.flush().unwrap();
          sleep(Duration::from_secs(1)).await;
        }
}

pub async fn start_timer(work: u64, rest: u64) {
    println!("Starting work session.");
    countdown(work).await;
    println!("Session ended. Break starting.");
    countdown(rest).await;
}

