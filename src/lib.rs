use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::sync::Mutex;
use winnow::ascii::digit1;
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
    digit1.parse_to().parse_next(input)
}

#[proc_macro]
pub fn make_mods(item: TokenStream) -> TokenStream {
    // Use Span::source_file() when it becomes stable.
    let mut mods = proc_macro2::TokenStream::new();
    let d = syn::parse_macro_input!(item as syn::LitStr);
    let mut map = PROBS.lock().unwrap();
    for entry in fs::read_dir(d.value()).unwrap().map(|x| x.unwrap().path()) {
        let path = entry.to_str().unwrap();
        match ("src/year", i64, "/day", i64, ".rs").parse(path) {
            Ok((_, year, _, day, _)) => {
                let m: proc_macro2::TokenStream = format!("day{day:02}").parse().unwrap();
                let mstr = m.to_string();
                let day = preceded("day", i64).parse(mstr.as_str()).unwrap();
                map.entry(year).or_default().insert(day);
                mods.extend(quote! {
                    pub mod #m;
                });
            }
            Err(_) => {
                // Add prints here if debugging is necessary.
            }
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

#[proc_macro_derive(Parser)]
pub fn parser(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let data_ident = &ast.ident;
    let inner = match &ast.data {
        syn::Data::Enum(enum_data) => {
            let mut parser_alts = quote! {};
            for val in &enum_data.variants {
                let variant_ident = &val.ident;
                let name_matcher = quote! { advent::lower!(#variant_ident) };

                let parser_alt = match &val.fields {
                    syn::Fields::Unnamed(fields) => {
                        let mut parsing_tup = quote! { #name_matcher };
                        let mut fn_match_arg_tup = quote! { _ };
                        let mut fn_apply_arg_tup = quote! {};
                        let mut arg = 'a';
                        for field in &fields.unnamed {
                            let fun = syn::Ident::new(
                                &field.ty.to_token_stream().to_string().to_lowercase(),
                                Span::call_site(),
                            );
                            let id = syn::Ident::new(&arg.to_string(), Span::call_site());
                            parsing_tup = quote! { #parsing_tup, space1, #fun };
                            fn_match_arg_tup = quote! { #fn_match_arg_tup, _, #id };
                            fn_apply_arg_tup = quote! { #fn_apply_arg_tup #id, };
                            arg = char::from_u32(arg as u32 + 1).unwrap();
                        }
                        quote! {
                            (#parsing_tup).map(move |(#fn_match_arg_tup)|
                                               #data_ident::#variant_ident(#fn_apply_arg_tup))
                        }
                    }
                    syn::Fields::Unit => {
                        quote! { #name_matcher.value(#data_ident::#variant_ident) }
                    }
                    _ => unimplemented!(),
                };
                parser_alts = quote! { #parser_alts #parser_alt, };
            }
            quote! { alt((#parser_alts)) }
        }
        syn::Data::Struct(struct_data) => {
            let name_matcher = quote! { advent::lower!(#data_ident) };
            match &struct_data.fields {
                syn::Fields::Unnamed(fields) => {
                    let mut parsing_tup = quote! {};
                    let mut fn_match_arg_tup = quote! {};
                    let mut fn_apply_arg_tup = quote! {};
                    let mut arg = 'a';
                    let mut first = true;
                    for field in &fields.unnamed {
                        let fun = syn::Ident::new(
                            &field.ty.to_token_stream().to_string().to_lowercase(),
                            Span::call_site(),
                        );
                        let id = syn::Ident::new(&arg.to_string(), Span::call_site());
                        if first {
                            parsing_tup = quote! { #fun };
                            fn_match_arg_tup = quote! { #id };
                            fn_apply_arg_tup = quote! { #id };
                        } else {
                            parsing_tup = quote! { #parsing_tup, space1, #fun };
                            fn_match_arg_tup = quote! { #fn_match_arg_tup, _, #id };
                            fn_apply_arg_tup = quote! { #fn_apply_arg_tup, #id };
                        }
                        arg = char::from_u32(arg as u32 + 1).unwrap();
                        first = false;
                    }
                    quote! {
                        (#parsing_tup).map(move |(#fn_match_arg_tup)|
                                           #data_ident(#fn_apply_arg_tup))
                    }
                }
                syn::Fields::Unit => {
                    quote! { #name_matcher.value(#data_ident) }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    };
    let fn_name = syn::Ident::new(&data_ident.to_string().to_lowercase(), Span::call_site());
    quote! {
        fn #fn_name(i: &mut &str) -> PResult<#data_ident> {
            #inner.parse_next(i)
        }
    }
    .into()
}
