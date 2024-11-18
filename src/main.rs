use std::cmp::max_by;
use std::cmp::Ordering::Equal;
use std::env;
use std::time::{Duration, Instant};

mod utils;

mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2020;
mod year2021;
mod year2022;
mod year2023;
// Needs to be after the year modules.
mod problems;

use utils::parsers::*;

fn colorize_time(n: f64) -> String {
    let color = if n < 0.5 {
        "\x1b[32m"
    } else if n < 1.0 {
        "\x1b[33m"
    } else {
        "\x1b[31m"
    };
    format!("{color}{n:.3}\x1b[0m")
}

fn run_part(f: fn(&str) -> String, input: &str) -> (String, Duration) {
    let start = Instant::now();
    let ans = f(input);
    let elapsed = start.elapsed();
    (ans, elapsed)
}

fn run_problem(year: i64, day: i64) -> Option<(Duration, String, String)> {
    if let Some((part1, part2)) = problems::get_prob(year, day) {
        let contents = problems::get_file_input(year, day, true).unwrap();

        println!("Day {day}");
        let (ans1, t1) = run_part(part1, &contents);
        print_output(1, &ans1, t1);
        let (ans2, t2) = run_part(part2, &contents);
        print_output(2, &ans2, t2);
        println!();
        Some((t1 + t2, ans1, ans2))
    } else {
        println!("{year} Day {day} not implemented");
        None
    }
}

fn print_output(part: usize, output: &str, t: Duration) {
    let t = colorize_time(t.as_secs_f64());
    println!("Part {part}: {output:>54}  Elapsed time {t} seconds");
}

fn parse_day(daystr: &str) -> Vec<i64> {
    if let Some((lo, hi)) = daystr.split_once('-') {
        (lo.i64()..=hi.i64()).collect()
    } else {
        vec![daystr.i64()]
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args[0] == "submit" {
        assert_eq!(args.len(), 3);
        let year = args[1].i64();
        let day = args[2].i64();
        if let Some((_, a1, a2)) = run_problem(year, day) {
            let (part, ans) = if a2.is_empty() || a2 == "0" {
                (1, a1)
            } else {
                (2, a2)
            };
            problems::submit_answer(year, day, part, &ans);
        }
        return;
    }
    let year = args[0].i64();
    let mut days: Vec<i64> = args[1..].iter().flat_map(|x| parse_day(x)).collect();
    if days.is_empty() {
        days = (1..=25).collect();
    }

    let mut total = Duration::ZERO;
    let mut max = (Duration::ZERO, 0);
    for &day in days.iter() {
        if let Some((t, _, _)) = run_problem(year, day) {
            max = max_by(max, (t, day), |a, b| a.0.partial_cmp(&b.0).unwrap_or(Equal));
            total += t;
        }
    }
    println!("Max: Day {:2} {:70.3} seconds", max.1, max.0.as_secs_f64());
    println!("Total: {:75.3} seconds", total.as_secs_f64());
}
