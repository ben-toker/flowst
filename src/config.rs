use std::time::{SystemTime, Duration};
use confy;
use serde::{Serialize,Deserialize};
use serde_with::{serde_as,TimestampSeconds,DurationSeconds};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct TimerInfo {
    #[serde_as(as = "TimestampSeconds")]
    pub start_time: SystemTime,
    #[serde_as(as = "DurationSeconds")]
    pub work_duration: Duration,
    #[serde_as(as = "DurationSeconds")]
    pub rest_duration: Duration,
    pub run_state: bool,
}

//Implementing default values for configuration per confy docs.
impl std::default::Default for TimerInfo {
    fn default() -> Self { Self {
            start_time: SystemTime::now(),
            work_duration: Duration::from_secs(25*60), // Default to 25 minutes
            rest_duration: Duration::from_secs(5*60), // Default to 5 minutes
            run_state: true,
        }
    }
}

pub fn load_timer() -> Result<TimerInfo, confy::ConfyError> {
    confy::load("timer_state", None)
}

pub fn save_timer(timer_info: &TimerInfo) -> Result<(), confy::ConfyError> {
    confy::store("timer_state", None, timer_info)
}

pub fn reset_timer() -> Result<(), confy::ConfyError>{
    let default_timer_info = TimerInfo {
        start_time: SystemTime::now(),
        work_duration: Duration::from_secs(25 * 60),
        rest_duration: Duration::from_secs(5 * 60),
        run_state: true,
    };
    confy::store("timer_state", None, default_timer_info)
}

