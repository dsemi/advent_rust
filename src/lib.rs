use proc_macro::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::sync::Mutex;
use winnow::ascii::dec_int;
use winnow::combinator::preceded;
use winnow::prelude::*;

static PROBS: Mutex<BTreeMap<i64, BTreeSet<i64>>> = Mutex::new(BTreeMap::new());

#[proc_macro]
pub fn make_problems(_item: TokenStream) -> TokenStream {
    let mut year_matches = proc_macro2::TokenStream::new();

    let map = PROBS.lock().unwrap();
    for (year, days) in map.iter() {
        let mut day_matches = proc_macro2::TokenStream::new();
        for day in days {
            let year_ident = format!("year{year}")
                .parse::<proc_macro2::TokenStream>()
                .unwrap();
            let day_ident = format!("day{day:02}")
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
        pub fn get_prob<'a>(year: i64, day: i64) -> Option<fn() -> (Part<'a>, Part<'a>)> {
            match year {
                #year_matches
                _ => None,
            }
        }
    };
    result.into()
}

fn i64(input: &mut &str) -> PResult<i64> {
    dec_int(input)
}

#[proc_macro]
pub fn make_mods(item: TokenStream) -> TokenStream {
    // Use Span::source_file() when it becomes stable.
    let mut mods = proc_macro2::TokenStream::new();
    let d = syn::parse_macro_input!(item as syn::LitStr);
    let mut map = PROBS.lock().unwrap();
    for entry in fs::read_dir(d.value()).unwrap().map(|x| x.unwrap().path()) {
        let path = entry.to_str().unwrap();
        if let Ok((_, year, _, day, _)) = ("src/year", i64, "/day", i64, ".rs").parse(path) {
            let m: proc_macro2::TokenStream = format!("day{day:02}").parse().unwrap();
            let mstr = m.to_string();
            let day = preceded("day", i64).parse(mstr.as_str()).unwrap();
            map.entry(year).or_default().insert(day);
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
                format!("test_{year}_{day:02}").parse().unwrap();
            result.extend(quote! {
                #[test]
                fn #fn_name() -> Result<(), Box<dyn Error>> {
                    let input = get_file_input(#year, #day, false)?;
                    let (part1, part2) = get_prob(#year, #day).unwrap()();
                    let (ex1, ex2) = get_expected_solutions(#year, #day)?;
                    assert_eq!(ex1, part1(&input));
                    assert_eq!(ex2, part2(&input));
                    Ok(())
                }
            });
        }
    }
    result.into()
}

#[proc_macro]
pub fn lower(item: TokenStream) -> TokenStream {
    let mut result = proc_macro2::TokenStream::new();
    let i = syn::parse_macro_input!(item as syn::Ident)
        .to_string()
        .to_lowercase();
    result.extend(quote! { #i });
    result.into()
}
