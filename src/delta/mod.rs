use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Neg};
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Duration {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
}

impl FromStr for Duration {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref YEAR: Regex = Regex::new(r"^(\d+)(?i)y(?:ear)?$").expect("year regex");
            static ref MONTH: Regex = Regex::new(r"^(\d+)((?i:month)|M)$").expect("month regex");
            static ref DAY: Regex = Regex::new(r"^(\d+)(?i)d(?:ay)?$").expect("day regex");
            static ref HOUR: Regex = Regex::new(r"^(\d+)(?i)h(?:our)?$").expect("hour regex");
            static ref MINUTE: Regex = Regex::new(r"^(\d+)((?i:minute)|m)$").expect("minute regex");
            static ref SECOND: Regex = Regex::new(r"^(\d+)(?i)s(?:econd)?$").expect("second regex");
        }
        if let Some(m) = YEAR.captures_iter(s).next() {
            return Ok(Duration {
                year: m.get(1).expect("regex match 1").as_str().parse()?,
                ..Duration::default()
            });
        }
        if let Some(m) = MONTH.captures_iter(s).next() {
            return Ok(Duration {
                month: m.get(1).expect("regex match 1").as_str().parse()?,
                ..Duration::default()
            });
        }
        if let Some(m) = DAY.captures_iter(s).next() {
            return Ok(Duration {
                day: m.get(1).expect("regex match 1").as_str().parse()?,
                ..Duration::default()
            });
        }
        if let Some(m) = HOUR.captures_iter(s).next() {
            return Ok(Duration {
                hour: m.get(1).expect("regex match 1").as_str().parse()?,
                ..Duration::default()
            });
        }
        if let Some(m) = MINUTE.captures_iter(s).next() {
            return Ok(Duration {
                minute: m.get(1).expect("regex match 1").as_str().parse()?,
                ..Duration::default()
            });
        }
        if let Some(m) = SECOND.captures_iter(s).next() {
            return Ok(Duration {
                second: m.get(1).expect("regex match 1").as_str().parse()?,
                ..Duration::default()
            });
        }
        Err(Box::new(ParseError {}))
    }
}

#[derive(Debug)]
struct ParseError {}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("parse duration error")
    }
}
impl ::std::error::Error for ParseError {}

impl Add<DateTime<Local>> for &Duration {
    type Output = DateTime<Local>;
    fn add(self, rhs: DateTime<Local>) -> Self::Output {
        let (y, m, d, h, min, s) = (
            rhs.year(),
            rhs.month(),
            rhs.day(),
            rhs.hour(),
            rhs.minute(),
            rhs.second(),
        );
        let (mut y, mut m) = (y + self.year, m as i32 + self.month);
        if m <= 0 {
            m += 12;
            y -= 1;
        } else if m > 12 {
            m -= 12;
            y += 1;
        }
        let t = Local.ymd(y, m as u32, d).and_hms(h, min, s);
        t + time::Duration::days(self.day as i64)
            + time::Duration::hours(self.hour as i64)
            + time::Duration::minutes(self.minute as i64)
            + time::Duration::seconds(self.second as i64)
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        self.year += rhs.year;
        self.month += rhs.month;
        self.day += rhs.day;
        self.hour += rhs.hour;
        self.minute += rhs.minute;
        self.second += rhs.second;
    }
}

impl Neg for Duration {
    type Output = Duration;
    fn neg(self) -> Self::Output {
        Duration {
            year: -self.year,
            month: -self.month,
            day: -self.day,
            hour: -self.hour,
            minute: -self.minute,
            second: -self.second,
        }
    }
}

pub fn parse<T: AsRef<str>>(deltas: &Vec<T>) -> Option<Duration> {
    let mut ret = Duration::default();
    for delta in deltas {
        let delta = delta.as_ref();
        if delta.is_empty() {
            continue;
        }
        let d = single_parse(&delta[1..])?;
        if is_add(delta.as_bytes()[0])? {
            ret += d;
        } else {
            ret += -d;
        }
    }
    Some(ret)
}

fn single_parse(s: &str) -> Option<Duration> {
    lazy_static! {
        static ref DURATION: Regex =
            Regex::new(r"(?i)\d+(y(ear)?|m(onth)|d(ay)?|h(our)?|m(inute)?|s(econd)?)")
                .expect("duration regex pattern");
        static ref FULL_DURATION: Regex =
            Regex::new(r"^(\s*(?i)\d+(y(ear)?|m(onth)|d(ay)?|h(our)?|m(inute)?|s(econd)?)\s*)*$")
                .expect("full duration regex pattern");
    }
    if !FULL_DURATION.is_match(s) {
        return None;
    }
    let mut ret = Duration::default();
    for capture in DURATION.captures_iter(s) {
        let d: Duration = capture
            .get(0)
            .expect("duration regex match 0")
            .as_str()
            .parse()
            .ok()?;
        ret += d;
    }
    Some(ret)
}

fn is_add(sign: u8) -> Option<bool> {
    match sign {
        b'+' => Some(true),
        b'-' => Some(false),
        _ => None,
    }
}
