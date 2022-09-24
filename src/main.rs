#![allow(clippy::many_single_char_names)]
#![deny(clippy::disallowed_types)]

use std::cmp::max_by;
use std::cmp::Ordering::Equal;
use std::env;
use std::time::Instant;

#[macro_use]
mod utils;
// mod md5;
mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2020;
mod year2021;
// Needs to be after the year modules.
mod problems;

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
    let t: f64 = elapsed.as_millis() as f64 / 1000.0;
    (t, ans)
}

fn run_problem(year: i64, day: i64) -> f64 {
    if let Some(f) = problems::get_prob(year, day) {
        let contents = problems::get_file_input(year, day, true).unwrap();

        let (part1, part2) = f();
        println!("Day {day}");
        let (t1, ans) = run_part(part1, &contents);
        print_output(1, ans, t1);
        let (t2, ans) = run_part(part2, &contents);
        print_output(2, ans, t2);
        println!();
        t1 + t2
    } else {
        println!("{year} Day {day} not implemented");
        0.0
    }
}

fn print_output(part: usize, output: String, t: f64) {
    print!("Part {}: ", part);
    let lns = output.lines().collect::<Vec<_>>();
    let len = lns.len();

    for (i, ln) in lns.into_iter().enumerate() {
        if i == len - 1 {
            if i == 0 {
                println!("{:>54}  Elapsed time {} seconds", ln, colorize_time(t));
            } else {
                println!("{:62}  Elapsed time {} seconds", ln, colorize_time(t));
            }
        } else {
            println!("{}", ln);
        }
    }
}

fn parse_day(daystr: &str) -> Vec<i64> {
    if daystr.contains('-') {
        let v: Vec<i64> = daystr.splitn(2, '-').map(|x| x.parse().unwrap()).collect();
        (v[0]..=v[1]).collect()
    } else {
        vec![daystr.parse().unwrap()]
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let year = args[0].parse().unwrap();
    let mut days: Vec<i64> = args[1..].iter().flat_map(|x| parse_day(x)).collect();
    if days.is_empty() {
        days = (1..=25).collect();
    }

    let mut total: f64 = 0.0;
    let mut max_day = (0.0, 0);
    for &day in days.iter() {
        let t = run_problem(year, day);
        max_day = max_by(max_day, (t, day), |a, b| {
            a.0.partial_cmp(&b.0).unwrap_or(Equal)
        });
        total += t;
    }
    println!("Max: Day {:2} {:70.3} seconds", max_day.1, max_day.0);
    println!("Total: {total:75.3} seconds");
}
