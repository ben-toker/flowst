use tokio::time::{sleep,Duration,Instant, sleep_until};
use std::env;
//use tokio (async) to actually initiate a timer.
//once timer is up, message is printed.


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
   let workdeadline = Instant::now() + Duration::from_secs(worktime);
   sleep_until(workdeadline).await;
   println!("{} seconds have elapsed. Rest: {} seconds and counting", worktime, resttime);
   let restdeadline = Instant::now() + Duration::from_secs(resttime);
   sleep_until(restdeadline).await;
   println!("{} seconds have elapsed. Session is now over.", resttime);

}


