use chrono::prelude::*;
use itertools::Itertools;

pub fn parse(s: &str) -> Option<Vec<DateTime<Local>>> {
    let s = s.split_whitespace().take(3).join(" ");
    let t = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f %z").ok()?;
    Some(vec![t.with_timezone(&Local)])
}
