use advent::{detect_problems, make_problems, make_ptypes, make_tests};
use reqwest::blocking::Client;
use std::env;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_file_input(year: i64, day: i64, download: bool) -> String {
    let path = format!("inputs/{}/input{}.txt", year, day);
    let input_file = Path::new(&path);
    if !input_file.exists() && download {
        println!("Downloading input for Year {} Day {}", year, day);
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let client = Client::new();
        let response = client
            .get(&url)
            .header("Cookie", env::var("AOC_SESSION").unwrap())
            .send()
            .unwrap()
            .bytes()
            .unwrap();
        let mut f = File::create(input_file).unwrap();
        f.write_all(&response).expect("File failed to write");
    }
    fs::read_to_string(input_file)
        .expect("Error reading the file")
        .trim_end()
        .to_string()
}

trait PType {
    fn to(&self) -> String;
}

make_ptypes!();

impl<T: PType> PType for Option<T> {
    fn to(&self) -> String {
        self.as_ref().unwrap().to()
    }
}

impl<T: PType, E: Debug> PType for Result<T, E> {
    fn to(&self) -> String {
        self.as_ref().unwrap().to()
    }
}

impl<T: PType, S: PType> PType for (T, S) {
    fn to(&self) -> String {
        format!("{},{}", self.0.to(), self.1.to())
    }
}

impl<T: PType, S: PType, U: PType> PType for (T, S, U) {
    fn to(&self) -> String {
        format!("{},{},{}", self.0.to(), self.1.to(), self.2.to())
    }
}

type Output = Box<dyn Fn(&str) -> String>;

fn wrap<T>(f: &'static dyn Fn(&str) -> T) -> Output
where
    T: PType,
{
    Box::new(move |x| f(x).to())
}

macro_rules! make_prob {
    ($y:ident, $d:ident) => {
        (wrap(&crate::$y::$d::part1), wrap(&crate::$y::$d::part2))
    };
}

detect_problems!();

make_problems!();

#[allow(dead_code)]
fn get_expected_solutions(year: i64, day: i64) -> Option<(String, String)> {
    lazy_static! {
        static ref DICT: json::JsonValue = json::parse(
            &fs::read_to_string(Path::new("test/expectedAnswers.json"))
                .expect("Error reading json file")
        )
        .unwrap();
    }
    match &DICT[year.to_string()][day.to_string()] {
        json::JsonValue::Array(v) => {
            let solns = v.iter().map(|x| x.as_str().unwrap()).collect::<Vec<_>>();
            Some((solns[0].to_string(), solns[1].to_string()))
        }
        _ => None,
    }
}

make_tests!();
