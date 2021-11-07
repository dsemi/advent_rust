#![allow(clippy::many_single_char_names)]
#![deny(clippy::disallowed_type)]

extern crate proc_macro;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use scan_fmt::scan_fmt as scanf;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::sync::Mutex;

lazy_static! {
    static ref PROBS: Mutex<BTreeMap<i64, BTreeSet<i64>>> = Mutex::new(BTreeMap::new());
}

#[proc_macro]
pub fn make_problems(_item: TokenStream) -> TokenStream {
    let mut year_matches = proc_macro2::TokenStream::new();

    let map = PROBS.lock().unwrap();
    for (year, days) in map.iter() {
        let mut day_matches = proc_macro2::TokenStream::new();
        for day in days {
            let year_ident = format!("year{}", year)
                .parse::<proc_macro2::TokenStream>()
                .unwrap();
            let day_ident = format!("day{:02}", day)
                .parse::<proc_macro2::TokenStream>()
                .unwrap();
            day_matches.extend(quote! {
                #day => Some(make_prob!(#year_ident, #day_ident)),
            });
        }
        year_matches.extend(quote! {
            #year => match day {
                #day_matches
                _ => None,
            },
        });
    }
    let result = quote! {
        pub fn get_prob(year: i64, day: i64) -> Option<(Output, Output)> {
            match year {
                #year_matches
                _ => None,
            }
        }
    };
    result.into()
}

#[proc_macro]
pub fn make_mods(item: TokenStream) -> TokenStream {
    // Use Span::source_file() when it becomes stable.
    let mut mods = proc_macro2::TokenStream::new();
    let d = syn::parse_macro_input!(item as syn::LitStr);
    let mut map = PROBS.lock().unwrap();
    for entry in fs::read_dir(d.value()).unwrap().map(|x| x.unwrap().path()) {
        let path = entry.to_str().unwrap();
        if let Ok((year, day)) = scanf!(path, "src/year{}/day{}.rs", i64, i64) {
            let m: proc_macro2::TokenStream = format!("day{:02}", day).parse().unwrap();
            let day = scanf!(&m.to_string(), "day{}", i64).unwrap();
            map.entry(year).or_insert_with(BTreeSet::new).insert(day);
            mods.extend(quote! {
                pub mod #m;
            });
        }
    }
    mods.into()
}

#[proc_macro]
pub fn make_tests(_item: TokenStream) -> TokenStream {
    let mut result = proc_macro2::TokenStream::new();

    let map = PROBS.lock().unwrap();
    for (year, days) in map.iter() {
        for day in days {
            let fn_name: proc_macro2::TokenStream =
                format!("test_{0}_{1:02}", year, day).parse().unwrap();
            result.extend(quote! {
                #[test]
                fn #fn_name() {
                    let input = get_file_input(#year, #day, false);
                    let (part1, part2) = get_prob(#year, #day).unwrap();
                    if let Some((ex1, ex2)) = get_expected_solutions(#year, #day) {
                        assert_eq!(ex1, part1(&input));
                        assert_eq!(ex2, part2(&input));
                    }
                }
            });
        }
    }
    result.into()
}
