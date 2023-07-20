use tokio::{sync::mpsc,time::Duration};

fn format_seconds(seconds: u64) -> Vec<u64>{
       return vec![(seconds/60),(seconds%60)];   
}

pub enum WorkBreak {
    Work,
    Break,
}


async fn countdown(seconds: Duration, sender: tokio::sync::mpsc::Sender<String>) -> Result<(), std::io::Error>{
    for i in (0..=(seconds.as_secs())).rev() {
        let (minutes, seconds) = (format_seconds(i)[0], format_seconds(i)[1]);
        let countdown_string = format!("{} minutes and {} seconds remaining ", minutes, seconds);
        sender.send(countdown_string).await.unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    //crate::config::reset_timer()?;   
    Ok(())
}


pub async fn start_timer(work: Duration, rest: Duration) -> tokio::sync::mpsc::Receiver<String> {
    //the channel is what passes the message; only sends 1 message
    // the message is either: not stopped or stopped
    let (sender, receiver) = mpsc::channel(1);
       tokio::spawn(async move {
        let _ = countdown(work, sender.clone()).await; //ugly but simple
        let _ = countdown(rest,  sender).await; //way to ignore return
                                                                        //value
    });
    receiver}

    
pub async fn pause(pause_sender: mpsc::Sender<()>) {
    pause_sender.send(()).await.unwrap();
}

