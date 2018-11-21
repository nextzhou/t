use self::delta::Duration;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeSet;
use std::env;

mod delta;
mod moment;

const TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

fn main() {
    if should_show_help(&env::args().nth(1).unwrap_or(String::new())) {
        show_help_msg();
        return;
    }
    let args = split_args(env::args().skip(1));
    let moments = moment::parse(&args.moment);
    let delta = delta::parse(&args.delta).unwrap_or(Duration::default());

    let results: BTreeSet<_> = moments
        .iter()
        .map(|&t| {
            let t = &delta + t;
            format!("{}\t{}", t.format(TIME_FORMAT), t.timestamp())
        })
        .collect();

    for s in results {
        println!("{}", s);
    }
}

#[derive(Debug)]
struct Args {
    moment: String,
    delta: Vec<String>,
}

fn split_args<T, I>(args: T) -> Args
where
    T: IntoIterator<Item = I>,
    I: Into<String> + AsRef<str>,
{
    let mut in_moment = true;
    let (moment, delta): (Vec<_>, _) = args.into_iter().map(Into::into).partition(|arg| {
        in_moment &= !is_delta(&arg);
        in_moment
    });

    let moment = moment.join(" ");

    let mut group_idx = 0;
    let delta_groups = delta.into_iter().group_by(|s| {
        if is_delta(&s) {
            group_idx += 1
        };
        group_idx
    });
    let delta = delta_groups
        .into_iter()
        .map(|(_, mut group)| group.join(" "))
        .collect();
    Args { moment, delta }
}

fn is_delta<T: AsRef<str>>(s: &T) -> bool {
    lazy_static! {
        static ref OFFSET: Regex =
            Regex::new(r"^[\+-]\d{4}$").expect("time zone offset regex pattern");
    }
    let s = s.as_ref();
    (s.starts_with('+') || s.starts_with('-')) && !OFFSET.is_match(s)
}

fn should_show_help(s: &str) -> bool {
    match s.trim() {
        "-h" | "--help" | "help" => true,
        _ => false,
    }
}

fn show_help_msg() {
    let msg = r"Timestamp conversion tool

Usage: t [time point] [+|- <duration>...]...

Time point format: all the usual format 😎

Duration format: <Number><Unit>
    Available duration units: y[ear], M[onth], d[ay], h[our], m[inute], s[econd]
";
    println!("{}", msg)
}
