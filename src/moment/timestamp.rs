use chrono::prelude::*;

pub fn parse(s: &str) -> Option<Vec<DateTime<Local>>> {
    let sec = s.parse().ok()?;
    let t = Local.timestamp(sec, 0);

    if 2010 <= t.year() && t.year() <= 2030 {
        Some(vec![t])
    } else {
        None
    }
}
