use std::cmp::max_by;
use std::cmp::Ordering::Equal;
use std::env;
use std::time::Instant;

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

fn run_part<'b>(f: Box<dyn Fn(&'b str) -> String + 'b>, input: &'b str) -> (f64, String) {
    let start = Instant::now();
    let ans = f(input);
    let elapsed = start.elapsed();
    let t: f64 = elapsed.as_micros() as f64; // / 1000000.0;
    (t, ans)
}

fn run_problem(year: i64, day: i64) -> Option<(f64, String, String)> {
    if let Some(f) = problems::get_prob(year, day) {
        let contents = problems::get_file_input(year, day, true).unwrap();

        let (part1, part2) = f();
        println!("Day {day}");
        let (t1, ans1) = run_part(part1, &contents);
        print_output(1, &ans1, t1);
        let (t2, ans2) = run_part(part2, &contents);
        print_output(2, &ans2, t2);
        println!();
        Some((t1 + t2, ans1, ans2))
    } else {
        println!("{year} Day {day} not implemented");
        None
    }
}

fn print_output(part: usize, output: &str, t: f64) {
    let t = colorize_time(t);
    println!("Part {}: {:>54}  Elapsed time {} seconds", part, output, t);
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

    let mut total: f64 = 0.0;
    let mut max_day = (0.0, 0);
    for &day in days.iter() {
        if let Some((t, _, _)) = run_problem(year, day) {
            max_day = max_by(max_day, (t, day), |a, b| {
                a.0.partial_cmp(&b.0).unwrap_or(Equal)
            });
            total += t;
        }
    }
    println!("Max: Day {:2} {:70.3} seconds", max_day.1, max_day.0);
    println!("Total: {total:75.3} seconds");
}
