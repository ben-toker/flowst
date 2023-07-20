use tokio::{sync::{mpsc,Mutex},time::Duration};
use std::sync::Arc;

fn format_seconds(seconds: u64) -> Vec<u64>{
       return vec![(seconds/60),(seconds%60)];   
}

#[derive(PartialEq, Clone)]
pub enum TimerState {
    Running,
    Paused,
}

pub enum WorkBreak {
    Work,
    Break,
}


async fn countdown(seconds: Duration, sender: tokio::sync::mpsc::Sender<String>, pause_receiver: Arc<Mutex<mpsc::Receiver<()>>>) -> Result<(), std::io::Error>{
    for i in (0..=(seconds.as_secs())).rev() {
        let (minutes, seconds) = (format_seconds(i)[0], format_seconds(i)[1]);
        let countdown_string = format!("{} minutes and {} seconds remaining ", minutes, seconds);
        sender.send(countdown_string).await.unwrap();

        // Check if a pause signal has been received
        if pause_receiver.lock().await.try_recv().is_ok() {
            while pause_receiver.lock().await.recv().await.is_some() {}
        }
        
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    crate::config::reset_timer()?;   
    Ok(())
}


pub async fn start_timer(work: Duration, rest: Duration) -> tokio::sync::mpsc::Receiver<String> {
    //the channel is what passes the message; only sends 1 message
    // the message is either: not stopped or stopped
    let (sender, receiver) = mpsc::channel(1);
    let (_pause_sender, pause_receiver) = mpsc::channel(1);
    let pause_receiver = Arc::new(Mutex::new(pause_receiver));
       tokio::spawn(async move {
        let _ = countdown(work, sender.clone(), pause_receiver.clone()).await; //ugly but simple
        let _ = countdown(rest,  sender, pause_receiver.clone()).await; //way to ignore return
                                                                        //value
    });
    receiver}

pub async fn pause(pause_sender: mpsc::Sender<()>) {
    pause_sender.send(()).await.unwrap();
}

