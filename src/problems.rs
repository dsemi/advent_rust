use advent::make_problems;
use lazy_static::lazy_static;
use reqwest::blocking::Client;
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

const RATE_LIMIT: Duration = Duration::from_secs(5);

pub fn get_file_input(year: i64, day: i64, download: bool) -> Result<String, impl Error> {
    let path = format!("inputs/{}/input{}.txt", year, day);
    let input_file = Path::new(&path);
    if !input_file.exists() && download {
        println!("Downloading input for Year {} Day {}", year, day);
        lazy_static! {
            static ref LAST: Mutex<Option<Instant>> = Mutex::new(None);
        }
        let mut last = LAST.lock().unwrap();
        let now = Instant::now();
        if last.is_some() && last.unwrap() + RATE_LIMIT > now {
            thread::sleep(last.unwrap() + RATE_LIMIT - now);
        }
        *last = Some(Instant::now());
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let content = Client::new()
            .get(&url)
            .header("Cookie", env::var("AOC_SESSION").unwrap())
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

impl<T: POutput> POutput for Option<T> {
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
    use lazy_static::lazy_static;
    use std::error::Error;

    use super::{get_file_input, get_prob};

    const EXP: &'static str = include_str!("../test/expectedAnswers.json");

    fn get_expected_solutions(year: i64, day: i64) -> Result<(String, String), String> {
        lazy_static! {
            static ref DICT: json::JsonValue = json::parse(EXP).unwrap();
        }
        match &DICT[year.to_string()][day.to_string()] {
            json::JsonValue::Array(v) => {
                let solns = v.iter().map(|x| x.as_str().unwrap()).collect::<Vec<_>>();
                Ok((solns[0].to_string(), solns[1].to_string()))
            }
            _ => Err(String::from("Expected solution not found")),
        }
    }

    make_tests!();
}
