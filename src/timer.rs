use tokio::{sync::mpsc,time::Duration};

fn format_seconds(seconds: u64) -> Vec<u64>{
       return vec![(seconds/60),(seconds%60)];   
}


//flushes stdout to update the text in realtime.
async fn countdown(seconds: u64, mut sender: tokio::sync::mpsc::Sender<String>) {
    for i in (0..=(seconds * 60)).rev() {
        let (minutes, seconds) = (format_seconds(i)[0], format_seconds(i)[1]);
        let countdown_string = format!("{} minutes and {} seconds remaining ", minutes, seconds);
        sender.send(countdown_string).await.unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}


pub async fn start_timer(work: u64, rest: u64) {
    //the channel is what passes the message; only sends 1 message
    // the message is either: not stopped or stopped
    let (sender, mut receiver) = mpsc::channel(1);

    tokio::spawn(async move {
        countdown(work, sender.clone()).await;
        countdown(rest, sender).await;
    });

    while let Some(message) = receiver.recv().await {
        println!("{}", message);
    }
}

