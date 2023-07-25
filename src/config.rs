use chrono::{DateTime, Utc,Duration};
use chrono::serde::ts_seconds_option as to_tsopt;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as,DurationSeconds};
use confy;
use std::fs;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct TimerInfo {
    #[serde_as(as = "DurationSeconds<i64>")]
    pub work_duration: Duration,
    #[serde_as(as = "DurationSeconds<i64>")]
    pub rest_duration: Duration,
    #[serde(with = "to_tsopt")]
    pub start_work: Option<DateTime<Utc>>,
    #[serde(with = "to_tsopt")]
    pub start_rest: Option<DateTime<Utc>>,
    #[serde(with = "to_tsopt")]
    pub pause_time: Option<DateTime<Utc>>,

    pub run_state: bool,
}

impl std::default::Default for TimerInfo {
    fn default() -> Self { Self {
            start_work: Some(Utc::now()),
            start_rest: Some(Utc::now() + chrono::Duration::minutes(25)),
            work_duration: Duration::minutes(25), // Default to 25 minutes
            rest_duration: Duration::minutes(5), // Default to 5 minutes
            pause_time: Some(Utc::now()),
            run_state: false,
        }
    }
}

pub fn load_timer() -> Result<TimerInfo, confy::ConfyError> {
    confy::load("timer_state", None)
}

pub fn save_timer(timer_info: &TimerInfo) -> Result<(), confy::ConfyError> {
    confy::store("timer_state", None, timer_info)
}

pub fn reset_timer() {
   // Define the path of the configuration file
let config_path = confy::get_configuration_file_path("timer_state", None);

// Delete the configuration file
match fs::remove_file(&config_path.unwrap()) {
    Ok(_) => println!("Configuration file deleted successfully."),
    Err(err) => println!("Failed to delete configuration file: {}", err),
}

}
