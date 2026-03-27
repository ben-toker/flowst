use chrono::serde::ts_seconds_option as to_tsopt;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};
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

impl Default for TimerInfo {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            start_work: Some(now),
            start_rest: Some(now + chrono::Duration::minutes(25)),
            work_duration: Duration::minutes(25),
            rest_duration: Duration::minutes(5),
            pause_time: Some(now),
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

pub fn save_cstm(work: i64, rest: i64) -> Result<(), confy::ConfyError> {
    let timer_info = TimerInfo {
        start_work: Some(Utc::now()),
        start_rest: Some(Utc::now() + chrono::Duration::minutes(work)),
        work_duration: Duration::minutes(work),
        rest_duration: Duration::minutes(rest),
        pause_time: Some(Utc::now()),
        run_state: false,
    };
    save_timer(&timer_info)
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

#[cfg(test)]
mod tests {
    use super::*;

    // The standard pomodoro defaults are 25 min work, 5 min rest.
    #[test]
    fn test_default_durations() {
        let info = TimerInfo::default();
        assert_eq!(info.work_duration, Duration::minutes(25));
        assert_eq!(info.rest_duration, Duration::minutes(5));
    }

    // A freshly created timer should not be running until explicitly started.
    #[test]
    fn test_default_run_state_is_false() {
        let info = TimerInfo::default();
        assert!(!info.run_state);
    }

    // start_rest must be exactly work_duration after start_work so the rest
    // phase begins at the right time. Both timestamps share the same `now`,
    // so the offset should be exact — not approximate.
    #[test]
    fn test_default_start_rest_is_work_duration_after_start_work() {
        let info = TimerInfo::default();
        let offset = info.start_rest.unwrap() - info.start_work.unwrap();
        assert_eq!(offset, info.work_duration);
    }

    // Verifies the full serde round-trip using TOML, which is the format confy
    // uses on disk. Timestamps are compared at second granularity because
    // ts_seconds_option intentionally drops sub-second precision.
    #[test]
    fn test_serialization_roundtrip() {
        let info = TimerInfo::default();
        let serialized = toml::to_string(&info).unwrap();
        let restored: TimerInfo = toml::from_str(&serialized).unwrap();
        assert_eq!(info.work_duration, restored.work_duration);
        assert_eq!(info.rest_duration, restored.rest_duration);
        assert_eq!(info.run_state, restored.run_state);
        assert_eq!(info.start_work.map(|t| t.timestamp()), restored.start_work.map(|t| t.timestamp()));
        assert_eq!(info.start_rest.map(|t| t.timestamp()), restored.start_rest.map(|t| t.timestamp()));
        assert_eq!(info.pause_time.map(|t| t.timestamp()), restored.pause_time.map(|t| t.timestamp()));
    }
}
