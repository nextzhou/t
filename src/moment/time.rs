use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse(s: &str) -> Option<Vec<DateTime<Local>>> {
    split(s).and_then(|args| {
        if args.validate() {
            Some(vec![args.into()])
        } else {
            None
        }
    })
}

#[derive(Default, Debug)]
pub(in crate::moment) struct Args {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl Into<DateTime<Local>> for Args {
    fn into(self) -> DateTime<Local> {
        let now = Local::now();
        Local
            .ymd(now.year(), now.month(), now.day())
            .and_hms(self.hour, self.minute, self.second)
    }
}

impl Args {
    pub fn validate(&self) -> bool {
        self.hour < 24 && self.minute < 60 && self.second < 60
    }
}

enum AmPm {
    AM,
    PM,
}

pub(in crate::moment) fn split(s: &str) -> Option<Args> {
    lazy_static! {
        static ref SEP: Regex = Regex::new(r"[\s:]").expect("time sep regex pattern");
    }
    let mut ss = SEP.split(s);
    let mut ret = Args::default();
    let mut am_pm = None;

    macro_rules! take_value {
        ($e: expr) => {
            loop {
                let s = ss.next().or($e)?;
                let ap = parse_am_pm(s);
                if am_pm.is_none() && ap.is_some() {
                    am_pm = ap;
                    continue;
                } else {
                    break s.parse().ok()?;
                }
            }
        };
    }

    ret.hour = take_value!(None);
    ret.minute = take_value!(Some("0"));
    ret.second = {
        let f: f64 = take_value!(Some("0"));
        f as u32
    };

    if am_pm.is_none() {
        if let Some(s) = ss.next() {
            am_pm = Some(parse_am_pm(s)?)
        }
    }

    if let Some(ap) = am_pm {
        match ap {
            AmPm::AM => {
                if ret.hour <= 12 {
                } else {
                    return None;
                }
            }
            AmPm::PM => {
                if ret.hour <= 12 {
                    ret.hour += 12
                } else {
                    return None;
                }
            }
        }
    }
    Some(ret)
}

fn parse_am_pm(s: &str) -> Option<AmPm> {
    lazy_static! {
        static ref AM_PATTERN: Regex = Regex::new(r"(?i)^a\.?m\.?$").expect("am regex pattern");
        static ref PM_PATTERN: Regex = Regex::new(r"(?i)^p\.?m\.?$").expect("pm regex pattern");
    }
    if AM_PATTERN.is_match(s) {
        return Some(AmPm::AM);
    }
    if PM_PATTERN.is_match(s) {
        return Some(AmPm::PM);
    }
    None
}
