use chrono::prelude::*;
use crate::moment::{date, time};
use itertools::Itertools;

pub fn parse(s: &str) -> Option<Vec<DateTime<Local>>> {
    macro_rules! try_parse {
        ($t: ident => $e: expr) => {
            match $t::split(&$e.join(" ")) {
                Some(args) => {
                    if args.validate() {
                        args
                    } else {
                        continue;
                    }
                }
                None => continue,
            }
        };
    }

    let ss = s.split_whitespace().collect_vec();

    let mut ret = Vec::new();
    for idx in 0..ss.len() {
        let date_args = try_parse!(date => ss[..idx]);
        let time_args = try_parse!(time => ss[idx..]);
        ret.push(
            Local
                .ymd(date_args.year, date_args.month, date_args.day)
                .and_hms(time_args.hour, time_args.minute, time_args.second),
        );
    }

    if let Some(t) = parse_case1(s) {
        ret.push(t);
    }

    Some(ret)
}

// 2018-11-21/11:21:11
fn parse_case1(s: &str) -> Option<DateTime<Local>> {
    let ss: Vec<_> = s.split('/').collect();
    if ss.len() == 2 {
        let date_args = date::split(&ss[0])?;
        let time_args = time::split(&ss[1])?;
        if date_args.validate() && time_args.validate() {
            Some(
                Local
                    .ymd(date_args.year, date_args.month, date_args.day)
                    .and_hms(time_args.hour, time_args.minute, time_args.second),
            )
        } else {
            None
        }
    } else {
        None
    }
}
