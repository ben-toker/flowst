use tokio::time::{sleep,Duration,Instant, sleep_until};
use std::{env,io::{stdout,Write}};
//use tokio (async) to actually initiate a timer.
//once timer is up, message is printed.


pub async fn countdown(seconds: u64) {
    let mut stdout = stdout();
    for i in (1..=seconds).rev() {
        write!(&mut stdout, "\r{} seconds remaining ", i).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_secs(1)).await;
    }
    println!("Session complete.");
}


pub async fn start_timer() {

    let mut worktime: u64 = 0;
    let mut resttime: u64 = 0;
    //initialize local variables based on environmental ones
    match env::var("work") {
        Ok(numstr) => {
            worktime = numstr.parse().expect("Failed to parse env str 'work'.");
        },
        Err(e) => eprintln!("Error getting env str 'work': {}",e),
    }
    match env::var("rest") {
        Ok(numstr) => {
            resttime = numstr.parse().expect("Failed to parse env str 'rest'.");
        },
        Err(e) => eprintln!("Error getting env str 'work': {}",e),
    }

    //TODO: change to minutes after testing
    countdown(worktime).await;
    countdown(resttime).await;
  }


