use tokio::sync::mpsc;
use chrono::Duration;

fn format_seconds(seconds: i64) -> Vec<i64>{
       return vec![(seconds/60),(seconds%60)];   
}

//will turn seconds into minutes (ex: 5 seconds -> 5 minutes)
async fn countdown(seconds: Duration,  sender: tokio::sync::mpsc::Sender<String>) -> Result<(), std::io::Error>{

    for i in (0..=(seconds.num_seconds())).rev() {
        let (minutes, seconds) = (format_seconds(i)[0], format_seconds(i)[1]);
        let countdown_string = format!("{} minutes and {} seconds remaining ", minutes, seconds);
        sender.send(countdown_string).await.unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}




pub async fn start_timer(work: Duration, rest: Duration, run_state: bool) -> tokio::sync::mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::channel(1);
    let sender2 = sender.clone();



    println!("work {} and rest {}", work.num_seconds(), rest.num_seconds());

    if !run_state {
        tokio::spawn(async move {
            let countdown_string: String;
            if work.num_seconds() != 0 {
                let (minutes, seconds) = (format_seconds(work.num_seconds())[0], format_seconds(work.num_seconds())[1]);
                countdown_string = format!("{} minutes and {} seconds remaining ", minutes, seconds);
            }
            else {
                let (minutes, seconds) = (format_seconds(rest.num_seconds())[0], format_seconds(rest.num_seconds())[1]);
                countdown_string = format!("{} minutes and {} seconds remaining ", minutes, seconds);
            }
            sender2.send(countdown_string).await.unwrap();
        });
    } else {
        tokio::spawn(async move {
            let _ = countdown(work, sender.clone()).await;
            let _ = countdown(rest, sender).await;
        });
    }

    receiver
}


    

