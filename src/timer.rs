use crate::config::TimerInfo;
use chrono::Duration;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc;
use tokio::time::{Duration as TokioDuration, Instant};

fn format_seconds(seconds: i64) -> (i64, i64) {
    ((seconds / 60), (seconds % 60))
}

pub fn print_time(seconds: i64) -> String {
    let (minutes, seconds) = format_seconds(seconds);
    format!("{} minutes and {} seconds remaining ", minutes, seconds)
}

async fn countdown(
    seconds: Duration,
    cancel: Arc<AtomicBool>,
    sender: tokio::sync::mpsc::Sender<String>,
) -> Result<(), std::io::Error> {
    let start = Instant::now();
    let end = start + TokioDuration::from_secs(seconds.num_seconds() as u64);

    while Instant::now() < end {
        if cancel.load(Ordering::Relaxed) {
            break;
        }

        let remaining = end - Instant::now();
        let countdown_string = print_time(remaining.as_secs() as i64);

        if sender.send(countdown_string).await.is_err() {
            break;
        }

        tokio::time::sleep(TokioDuration::from_secs(1)).await;
    }

    if !cancel.load(Ordering::Relaxed) {
        //This check ensures that the bell plays at the end of the
        //timer; otherwise, it would play anytime you quit the app.

        // initiates the terminal bell with message.
        print!("\x07Timer finished!")
    }
    Ok(())
}

pub async fn paused(
    timer_info: TimerInfo,
    cancel: Arc<AtomicBool>,
    sender: tokio::sync::mpsc::Sender<String>,
) -> Result<(), std::io::Error> {
    let start_work_elapsed =
        chrono::Utc::now().signed_duration_since(timer_info.start_work.unwrap());

    let pause_elapsed =
        if start_work_elapsed.num_seconds() <= timer_info.work_duration.num_seconds() {
            timer_info.work_duration
                - timer_info
                    .pause_time
                    .unwrap()
                    .signed_duration_since(timer_info.start_work.unwrap())
        } else {
            timer_info.rest_duration
                - timer_info
                    .pause_time
                    .unwrap()
                    .signed_duration_since(timer_info.start_rest.unwrap())
        };

    if !timer_info.run_state {
        loop {
            if cancel.load(Ordering::Relaxed) {
                break;
            }
            let message = print_time(pause_elapsed.num_seconds());
            if sender.send(message).await.is_err() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 0 seconds should produce (0 minutes, 0 seconds) — the identity case.
    #[test]
    fn test_format_seconds_zero() {
        assert_eq!(format_seconds(0), (0, 0));
    }

    // 60 seconds is exactly one minute with no remainder.
    #[test]
    fn test_format_seconds_exactly_one_minute() {
        assert_eq!(format_seconds(60), (1, 0));
    }

    // 61 seconds should split into 1 minute and 1 second.
    #[test]
    fn test_format_seconds_mixed() {
        assert_eq!(format_seconds(61), (1, 1));
    }

    // Values under 60 should produce 0 minutes and the full value as seconds.
    #[test]
    fn test_format_seconds_seconds_only() {
        assert_eq!(format_seconds(45), (0, 45));
    }

    // 0 seconds remaining should format as "0 minutes and 0 seconds remaining ".
    #[test]
    fn test_print_time_zero() {
        assert_eq!(print_time(0), "0 minutes and 0 seconds remaining ");
    }

    // Exactly 60 seconds should display as 1 minute, 0 seconds.
    #[test]
    fn test_print_time_one_minute() {
        assert_eq!(print_time(60), "1 minutes and 0 seconds remaining ");
    }

    // 61 seconds should display as 1 minute and 1 second.
    #[test]
    fn test_print_time_mixed() {
        assert_eq!(print_time(61), "1 minutes and 1 seconds remaining ");
    }

    // Large values (e.g. 1 hour) should display minutes correctly with no overflow into a higher unit.
    #[test]
    fn test_print_time_large() {
        assert_eq!(print_time(3600), "60 minutes and 0 seconds remaining ");
    }
}

pub async fn start_timer() -> (tokio::sync::mpsc::Receiver<String>, Arc<AtomicBool>) {
    let timer_info = crate::config::load_timer().unwrap();
    let cancel = Arc::new(AtomicBool::new(false));

    let start_work_elapsed =
        chrono::Utc::now().signed_duration_since(timer_info.start_work.unwrap());
    let start_rest_elapsed =
        chrono::Utc::now().signed_duration_since(timer_info.start_rest.unwrap());

    let work = if start_work_elapsed.num_seconds() <= 0 {
        timer_info.work_duration
    } else {
        timer_info.work_duration - start_work_elapsed
    };
    let rest = if start_rest_elapsed.num_seconds() <= 0 {
        timer_info.rest_duration
    } else {
        timer_info.rest_duration - start_rest_elapsed
    };

    // Check if both work and rest durations are <= 0
    if work.num_seconds() <= 0 && rest.num_seconds() <= 0 {
        let (_, _receiver) = mpsc::channel(1);
        return (_receiver, cancel); // Return immediately without spawning tasks
    }

    let cancel1 = cancel.clone();
    let cancel2 = cancel.clone();
    let cancel3 = cancel.clone();

    let (sender, receiver) = mpsc::channel(1);
    let state = timer_info.run_state;

    if state {
        tokio::spawn(async move {
            let _ = countdown(work, cancel1, sender.clone()).await;
            let _ = countdown(rest, cancel2, sender).await;
        });
    } else {
        tokio::spawn(async move {
            let _ = paused(timer_info, cancel3, sender.clone()).await;
        });
    }

    (receiver, cancel)
}
