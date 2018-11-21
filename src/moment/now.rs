use chrono::prelude::*;

pub fn parse(s: &str) -> Option<Vec<DateTime<Local>>> {
    if s.is_empty() {
        Some(vec![Local::now()])
    } else {
        None
    }
}
