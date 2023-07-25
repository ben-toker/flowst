use tokio::sync::mpsc;
use chrono::Duration;
use crate::config::TimerInfo;
fn format_seconds(seconds: i64) -> Vec<i64>{
       return vec![(seconds/60),(seconds%60)];   
}

pub fn print_time(seconds: i64) -> String {
    let (minutes, seconds) = (format_seconds(seconds)[0], format_seconds(seconds)[1]);
    format!("{} minutes and {} seconds remaining ", minutes, seconds)
}

async fn countdown(seconds: Duration,  sender: tokio::sync::mpsc::Sender<String>) -> Result<(), std::io::Error>{

    for i in (0..=(seconds.num_seconds())).rev() {
        let countdown_string = print_time(i);
        sender.send(countdown_string).await.unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}

pub fn paused(timer_info: &TimerInfo) -> String{
        let start_work_elapsed = chrono::Utc::now().signed_duration_since(timer_info.start_work.unwrap());

        let pause_elapsed = if start_work_elapsed.num_seconds() <=0 {
            timer_info.start_work.unwrap().signed_duration_since(timer_info.pause_time.unwrap())
        } else {
            timer_info.start_rest.unwrap().signed_duration_since(timer_info.pause_time.unwrap())
        };

        print_time(pause_elapsed.num_seconds())

}


pub async fn start_timer(work: Duration, rest: Duration) -> tokio::sync::mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::channel(1);
    tokio::spawn(async move {
        let _ = countdown(work, sender.clone()).await;
        let _ = countdown(rest, sender).await;
    });

    receiver
}


    

