use chrono::prelude::*;

pub fn parse(s: &str) -> Option<Vec<DateTime<Local>>> {
    if let Ok(t) = DateTime::parse_from_rfc2822(s) {
        return Some(vec![t.with_timezone(&Local)]);
    }
    if let Ok(t) = DateTime::parse_from_rfc3339(s) {
        return Some(vec![t.with_timezone(&Local)]);
    }
    None
}
