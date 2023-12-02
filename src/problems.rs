use advent::make_problems;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::HeaderMap;
use select::document::Document;
use select::predicate::Name;
use serde::Serialize;
use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

fn client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Cookie",
        env::var("AOC_SESSION").unwrap().try_into().unwrap(),
    );
    ClientBuilder::new()
        .user_agent("github.com/dsemi/advent_rust")
        .default_headers(headers)
        .build()
        .expect("Failed to create HTTP client")
}

const RATE_LIMIT: Duration = Duration::from_secs(5);
static LAST: Mutex<Option<Instant>> = Mutex::new(None);

pub fn get_file_input(year: i64, day: i64, download: bool) -> Result<String, impl Error> {
    let path = format!("inputs/{year}/input{day}.txt");
    let input_file = Path::new(&path);
    if !input_file.exists() && download {
        println!("Downloading input for Year {year} Day {day}");
        let mut last = LAST.lock().unwrap();
        let now = Instant::now();
        if last.is_some() && last.unwrap() + RATE_LIMIT > now {
            thread::sleep(last.unwrap() + RATE_LIMIT - now);
        }
        *last = Some(Instant::now());
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let content = client()
            .get(url)
            .send()
            .expect("Problem input fetch failed")
            .error_for_status()
            .expect("Bad HTTP response")
            .bytes()
            .unwrap();
        let parent = input_file.parent().unwrap();
        fs::create_dir_all(parent).expect("Parent folder failed to create");
        let mut f = File::create(input_file).unwrap();
        f.write_all(&content).expect("File failed to write");
    }
    fs::read_to_string(input_file).map(|f| f.trim_end().to_string())
}

#[derive(Serialize)]
struct Answer<'a> {
    level: i64,
    answer: &'a str,
}

pub fn submit_answer(year: i64, day: i64, part: i64, answer: &str) {
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let data = Answer {
        level: part,
        answer,
    };
    let response = client()
        .post(url)
        .form(&data)
        .send()
        .expect("Problem submission failed")
        .error_for_status()
        .expect("Bad HTTP response")
        .text()
        .unwrap();
    let document = Document::from(response.as_str());
    let text = document
        .find(Name("main"))
        .next()
        .expect("Could not find submission response text")
        .text();
    println!("{}", text);
}

trait POutput {
    fn to(&self) -> String;
}

macro_rules! make_poutputs {
    ($($typ:ty),*) => ($(
        impl POutput for $typ {
            fn to(&self) -> String {
                self.to_string()
            }
        }
    )*)
}

make_poutputs!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, String);

impl POutput for &str {
    fn to(&self) -> String {
        self.to_string()
    }
}

impl<T: POutput> POutput for Option<T> {
    fn to(&self) -> String {
        self.as_ref().unwrap().to()
    }
}

impl<T: POutput, E: std::fmt::Debug> POutput for Result<T, E> {
    fn to(&self) -> String {
        self.as_ref().unwrap().to()
    }
}

impl<T: POutput, S: POutput> POutput for (T, S) {
    fn to(&self) -> String {
        format!("{},{}", self.0.to(), self.1.to())
    }
}

impl<T: POutput, S: POutput, U: POutput> POutput for (T, S, U) {
    fn to(&self) -> String {
        format!("{},{},{}", self.0.to(), self.1.to(), self.2.to())
    }
}

impl<const N: usize> POutput for [u8; N] {
    fn to(&self) -> String {
        std::str::from_utf8(self).to()
    }
}

trait PInput<'a> {
    fn un<'b: 'a>(s: &'b str) -> Self;
}

impl<'a> PInput<'a> for &'a str {
    fn un<'b: 'a>(s: &'b str) -> Self {
        s
    }
}

impl<'a> PInput<'a> for &'a [u8] {
    fn un<'b: 'a>(s: &'b str) -> Self {
        s.as_bytes()
    }
}

macro_rules! make_pinputs {
    ($($typ:ty),*) => ($(
        impl<'a> PInput<'a> for $typ {
            fn un<'b: 'a>(s: &'b str) -> Self {
                s.parse().unwrap()
            }
        }
    )*)
}

make_pinputs!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);

impl<'a, T: PInput<'a>> PInput<'a> for Vec<T> {
    fn un<'b: 'a>(s: &'b str) -> Self {
        s.split_whitespace().map(|x| PInput::un(x)).collect()
    }
}

impl<'a, T: PInput<'a> + std::cmp::Ord> PInput<'a> for BTreeSet<T> {
    fn un<'b: 'a>(s: &'b str) -> Self {
        s.split_whitespace().map(|x| PInput::un(x)).collect()
    }
}

type Part<'a> = Box<dyn Fn(&'a str) -> String>;

fn wrap<'a, S, T, F>(f: F) -> Part<'a>
where
    S: PInput<'a>,
    T: POutput,
    F: 'static + Fn(S) -> T,
{
    Box::new(move |x: &'a str| f(PInput::un(x)).to())
}

macro_rules! make_prob {
    ($y:ident, $d:ident) => {
        || (wrap(crate::$y::$d::part1), wrap(crate::$y::$d::part2))
    };
}

make_problems!();

#[cfg(test)]
mod tests {
    use advent::make_tests;
    use once_cell::sync::Lazy;
    use std::error::Error;
    use toml::value::Table;
    use toml::value::Value;

    use super::{get_file_input, get_prob};

    const EXP: &str = include_str!("../test/expectedAnswers.toml");
    static DICT: Lazy<Table> = Lazy::new(|| toml::from_str(EXP).unwrap());

    fn get_expected_solutions(year: i64, day: i64) -> Result<(String, String), String> {
        match &DICT[&year.to_string()][day.to_string()] {
            Value::Table(t) => match (&t["part1"], &t["part2"]) {
                (Value::String(a), Value::String(b)) => Ok((a.to_string(), b.to_string())),
                _ => Err(String::from("Invalid types")),
            },
            _ => Err(String::from("Expected solution not found")),
        }
    }

    make_tests!();
}
