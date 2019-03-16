use chrono::prelude::*;

pub fn parse(s: &str) -> Option<Vec<DateTime<Local>>> {
    type TryTimestamp = fn(&str) -> Option<DateTime<Local>>;
    let parses: Vec<TryTimestamp> = vec![parse_sec, parse_ms, parse_float_sec];
    for p in parses {
        if let Some(t) = p(s) {
            return Some(vec![t]);
        }
    }
    None
}

fn parse_sec(s: &str) -> Option<DateTime<Local>> {
    let sec = s.parse().ok()?;
    let t = Local.timestamp(sec, 0);

    if 2010 <= t.year() && t.year() <= 2030 {
        Some(t)
    } else {
        None
    }
}

fn parse_ms(s: &str) -> Option<DateTime<Local>> {
    let ms: i64 = s.parse().ok()?;
    let t = Local.timestamp(ms / 1000, ms as u32 % 1000);

    if 2010 <= t.year() && t.year() <= 2030 {
        Some(t)
    } else {
        None
    }
}

fn parse_float_sec(s: &str) -> Option<DateTime<Local>> {
    let sec: f64 = s.parse().ok()?;
    let t = Local.timestamp(sec as i64, 0);

    if 2010 <= t.year() && t.year() <= 2030 {
        Some(t)
    } else {
        None
    }
}
