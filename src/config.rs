use std::time::{SystemTime, Duration};
use confy;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TimerInfo {
    pub start_time: SystemTime,
    pub work_duration: Duration,
    pub rest_duration: Duration,
}

//Implementing default values for configuration per confy docs.
impl std::default::Default for TimerInfo {
    fn default() -> Self { Self {
            start_time: SystemTime::now(),
            work_duration: Duration::from_secs(25*60), // Default to 25 minutes
            rest_duration: Duration::from_secs(5*60), // Default to 5 minutes
        }
    }
}

pub fn load_timer() -> Result<TimerInfo, std::io::Error> {
    confy::load("timer_state")
}

pub fn save_timer(timer_info: &TimerInfo) -> Result<(), std::io::Error> {
    confy::store("timer_state", timer_info)
}

pub fn reset_timer() -> Result<(), std::io::Error>{
    let default_timer_info = TimerInfo {
        start_time: SystemTime::now(),
        work_duration: Duration::from_secs(25 * 60),
        rest_duration: Duration::from_secs(5 * 60),
    };
    confy::store("timer_state", default_timer_info)
}

