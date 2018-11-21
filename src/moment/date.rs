use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse(s: &str) -> Option<Vec<DateTime<Local>>> {
    let args = split(s)?;
    if args.validate() {
        Some(vec![args.into()])
    } else {
        None
    }
}

#[derive(Default, Debug)]
pub(in crate::moment) struct Args {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Args {
    fn new(mut y: i32, m: u32, d: u32) -> Self {
        if 0 < y && y < 100 {
            if y >= 70 {
                y += 1900
            } else {
                y += 2000
            }
        }
        Args {
            year: y,
            month: m,
            day: d,
        }
    }
}

impl Into<DateTime<Local>> for Args {
    fn into(self) -> DateTime<Local> {
        let now = Local::now();
        Local
            .ymd(self.year, self.month, self.day)
            .and_hms(now.hour(), now.minute(), now.second())
    }
}

impl Args {
    pub fn validate(&self) -> bool {
        (1970 < self.year && self.year <= 2050)
            && (0 < self.month && self.month <= 12)
            && (0 < self.day && self.day <= 31)
    }
}

pub(in crate::moment) fn split(s: &str) -> Option<Args> {
    lazy_static! {
        static ref SEP: Regex = Regex::new(r"[\s/\-]").expect("time sep regex pattern");
    }
    let ss: Vec<u32> = try_vec(SEP.split(s).map(|s| s.parse().ok()).collect())?;

    let now = Local::now();
    let (y, m, _d) = (now.year(), now.month(), now.day());

    match ss.len() {
        1 => Some(Args::new(y, m, ss[0])),
        2 => Some(Args::new(y, ss[0], ss[1])),
        3 => Some(Args::new(ss[0] as i32, ss[1], ss[2])),
        _ => None,
    }
}

fn try_vec<T>(v: Vec<Option<T>>) -> Option<Vec<T>> {
    let mut ret = Vec::with_capacity(v.len());
    for item in v {
        match item {
            Some(item) => ret.push(item),
            None => return None,
        }
    }
    Some(ret)
}

/*
fn try_vec<T>(v: Vec<T>) -> Result<Vec<T::Ok>, T::Error>
where
    T: Try,
{
    let mut ret = Vec::with_capacity(v.len());
    for item in v {
        match item.into_result() {
            Ok(item) => ret.push(item),
            Err(err) => return Err(err),
        }
    }
    Ok(ret)
}
*/
