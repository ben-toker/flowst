use tokio::sync::mpsc;
use chrono::Duration;
use crate::config::TimerInfo;
use std::sync::{Arc,atomic::{AtomicBool,Ordering}};
fn format_seconds(seconds: i64) -> Vec<i64>{
       return vec![(seconds/60),(seconds%60)];   
}

pub fn print_time(seconds: i64) -> String {
    let (minutes, seconds) = (format_seconds(seconds)[0], format_seconds(seconds)[1]);
    format!("{} minutes and {} seconds remaining ", minutes, seconds)
}

async fn countdown(seconds: Duration, cancel: Arc<AtomicBool>,  sender: tokio::sync::mpsc::Sender<String>) -> Result<(), std::io::Error>{

    for i in (0..=(seconds.num_seconds())).rev() {
        if cancel.load(Ordering::Relaxed) {
            break;
        }  

        let countdown_string = print_time(i);          

        if let Err(_) = sender.send(countdown_string).await {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}

pub async fn paused(timer_info: TimerInfo, cancel: Arc<AtomicBool>, sender: tokio::sync::mpsc::Sender<String>)-> Result<(),std::io::Error> {
        let start_work_elapsed = chrono::Utc::now().signed_duration_since(timer_info.start_work.unwrap());

        let pause_elapsed = if start_work_elapsed.num_seconds() <= timer_info.work_duration.num_seconds() {
            timer_info.work_duration - timer_info.pause_time.unwrap().signed_duration_since(timer_info.start_work.unwrap())
        } else {
            timer_info.rest_duration - timer_info.pause_time.unwrap().signed_duration_since(timer_info.start_rest.unwrap())
        };
        
        while !timer_info.run_state {
            if cancel.load(Ordering::Relaxed) {
                break;
             }  
            let message = print_time(pause_elapsed.num_seconds());
            if let Err(_) = sender.send(message).await {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    Ok(())

}


pub async fn start_timer() -> (tokio::sync::mpsc::Receiver<String>, Arc<AtomicBool>) {
  let timer_info = crate::config::load_timer().unwrap();
  let cancel = Arc::new(AtomicBool::new(false));
  let cancel1 = cancel.clone();
  let cancel2 = cancel.clone();
  let cancel3 = cancel.clone();

  let start_work_elapsed = chrono::Utc::now().signed_duration_since(timer_info.start_work.unwrap());
  let start_rest_elapsed = chrono::Utc::now().signed_duration_since(timer_info.start_rest.unwrap());

  let work = if start_work_elapsed.num_seconds() <=0 {timer_info.work_duration} else {timer_info.work_duration - start_work_elapsed};
  let rest = if start_rest_elapsed.num_seconds() <=0 {timer_info.rest_duration} else {timer_info.rest_duration - start_rest_elapsed};
        
  let (sender, receiver) = mpsc::channel(1);
  let state = timer_info.run_state;


  if state {
        tokio::spawn(async move {
            let _ = countdown(work, cancel1, sender.clone()).await;
            let _ = countdown(rest, cancel2, sender).await;
        });
    }
    else {
        tokio::spawn(async move {
            let _ = paused(timer_info,cancel3, sender.clone()).await;
        });
    }

    (receiver, cancel)
}


    

