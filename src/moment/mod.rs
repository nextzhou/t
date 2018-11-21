use chrono::prelude::*;
use lazy_static::lazy_static;

mod date;
mod date_time;
mod go;
mod now;
mod standard;
mod time;
mod timestamp;

type TryMoment = fn(s: &str) -> Option<Vec<DateTime<Local>>>;

lazy_static! {
    static ref PARSERS: Vec<TryMoment> = vec![
        timestamp::parse,
        now::parse,
        time::parse,
        date::parse,
        date_time::parse,
        standard::parse,
        go::parse,
    ];
}

pub fn parse<T: AsRef<str>>(s: &T) -> Vec<DateTime<Local>> {
    PARSERS
        .iter()
        .map(|p| p(s.as_ref()).unwrap_or(vec![]))
        .flatten()
        .collect()
}
