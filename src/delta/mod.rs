use chrono::prelude::*;
use time::Duration;

pub fn parse<T: AsRef<str>>(delta: &Vec<T>) -> Vec<Duration> {
    vec![Duration::zero()]
}
