extern crate proc_macro;
use itertools::Itertools;
use proc_macro::TokenStream;
use regex::Regex;
use std::fs;
use syn;

#[proc_macro]
pub fn make_ptypes(_item: TokenStream) -> TokenStream {
    let to_strings = vec![
        "i8", "i16", "i32", "i64", "i128",
        "u8", "u16", "u32", "u64", "u128",
        "usize", "String"
    ];
    to_strings.iter()
        .map(|x| format!("impl PType for {} {{
                            fn to(&self) -> String {{
                              self.to_string()
                            }}
                          }}", x))
        .join("\n").parse().unwrap()
}

#[proc_macro]
pub fn make_problems(_item: TokenStream) -> TokenStream {
    let mut year_matches: String = "match year {\n".to_string();

    let re = Regex::new(r"year(\d\d\d\d)$").unwrap();
    for entry in fs::read_dir("src").unwrap() {
        if let Some(cap) = re.captures(entry.unwrap().path().to_str().unwrap()) {
            let year = &cap[1].parse::<i64>().unwrap();
            let mut day_matches: String = "match day {\n".to_string();
            let dir = format!("src/year{}", year);
            let re = Regex::new(r"/day(\d\d).rs$").unwrap();
            for entry in fs::read_dir(dir).unwrap() {
                if let Some(cap) = re.captures(entry.unwrap().path().to_str().unwrap()) {
                    let day = &cap[1].parse::<i64>().unwrap();
                    day_matches.push_str(&format!("{1} => make_prob!(year{0}, day{1:02}),\n", year, day));
                }
            }
            day_matches.push_str("_ => panic!(\"bad day\"),\n}");
            year_matches.push_str(&format!("{} => {},\n", year, day_matches));
        }
    }
    year_matches.push_str("_ => panic!(\"bad year\"),\n}");
    format!("pub fn get_prob(year: i64, day: i64) -> (Box<dyn Fn(&str) -> String>, Box<dyn Fn(&str) -> String>) {{ {} }}", year_matches).parse().unwrap()
}

#[proc_macro]
pub fn make_mods(item: TokenStream) -> TokenStream {
    let mut mods: String = String::new();
    let dir = syn::parse_macro_input!(item as syn::LitStr);
    let re = Regex::new(r"/day(\d\d).rs$").unwrap();
    for entry in fs::read_dir(dir.value()).unwrap() {
        if let Some(cap) = re.captures(entry.unwrap().path().to_str().unwrap()) {
            mods.push_str(&format!("pub mod day{};", &cap[1]));
        }
    }
    mods.parse().unwrap()
}
