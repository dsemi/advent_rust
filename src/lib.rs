use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Mutex;
use syn::parse_macro_input;
use winnow::ascii::digit1;
use winnow::combinator::preceded;
use winnow::error::Result;
use winnow::prelude::*;

static PROBS: Mutex<BTreeMap<i64, BTreeSet<i64>>> = Mutex::new(BTreeMap::new());

fn i64(input: &mut &str) -> Result<i64> {
    digit1.parse_to().parse_next(input)
}

#[proc_macro_attribute]
pub fn register_mods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let module = item.clone();
    let module = parse_macro_input!(module as syn::ItemMod);
    let s = module.ident.to_string();
    let year = preceded("year", i64).parse(&s).unwrap();
    let mut map = PROBS.lock().unwrap();
    for dec in module.content.unwrap().1 {
        if let syn::Item::Mod(m) = dec {
            let s = m.ident.to_string();
            if let Ok(day) = preceded("day", i64).parse(&s) {
                map.entry(year).or_default().insert(day);
            }
        }
    }
    item
}

#[proc_macro]
pub fn make_problems(_item: TokenStream) -> TokenStream {
    let mut year_matches = proc_macro2::TokenStream::new();

    let map = PROBS.lock().unwrap();
    for (year, days) in map.iter() {
        let mut day_matches = proc_macro2::TokenStream::new();
        for day in days {
            let year_ident = format!("year{year}").parse::<proc_macro2::TokenStream>().unwrap();
            let day_ident = format!("day{day:02}").parse::<proc_macro2::TokenStream>().unwrap();
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
        pub fn get_prob(year: i64, day: i64) -> Option<(Part, Part)> {
            match year {
                #year_matches
                _ => None,
            }
        }
    };
    result.into()
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
                    let (part1, part2) = get_prob(#year, #day).unwrap();
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
    let i = syn::parse_macro_input!(item as syn::Ident).to_string().to_lowercase();
    result.extend(quote! { #i });
    result.into()
}

const PARSER_IDENT: &str = "parser";

fn parse_fields(
    fields: &syn::Fields,
    cons: &proc_macro2::TokenStream,
    name_matcher: proc_macro2::TokenStream,
    parse_name: bool,
) -> proc_macro2::TokenStream {
    match fields {
        syn::Fields::Unnamed(fields) => {
            let mut parsing_tup = Vec::new();
            let mut fn_match_arg_tup = Vec::new();
            let mut fn_apply_arg_tup = Vec::new();
            if parse_name {
                parsing_tup.push(name_matcher);
                fn_match_arg_tup.push(quote! { _ });
            }
            let mut arg = 'a';
            for field in &fields.unnamed {
                let mut fun = syn::Ident::new(
                    &field.ty.to_token_stream().to_string().to_lowercase(),
                    Span::call_site(),
                );
                for attr in &field.attrs {
                    if attr.path().is_ident(PARSER_IDENT) {
                        attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("impl") {
                                let ident: syn::Ident = meta.value()?.parse()?;
                                fun = ident;
                            }
                            Ok(())
                        })
                        .unwrap();
                    }
                }
                if !parsing_tup.is_empty() {
                    parsing_tup.push(quote! { space1 });
                }
                parsing_tup.push(quote! { #fun });
                if !fn_match_arg_tup.is_empty() {
                    fn_match_arg_tup.push(quote! { _ });
                }
                let id = syn::Ident::new(&arg.to_string(), Span::call_site());
                fn_match_arg_tup.push(quote! { #id });
                fn_apply_arg_tup.push(quote! { #id });
                arg = char::from_u32(arg as u32 + 1).unwrap();
            }
            quote! {
                (#(#parsing_tup),*).map(move |(#(#fn_match_arg_tup),*)|
                                        #cons(#(#fn_apply_arg_tup),*))
            }
        }
        syn::Fields::Unit => quote! { #name_matcher.value(#cons) },
        syn::Fields::Named(_) => unimplemented!(),
    }
}

#[proc_macro_derive(Parser, attributes(parser))]
pub fn parser(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let mut parse_name = true;
    for attr in &ast.attrs {
        if attr.path().is_ident(PARSER_IDENT) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("dont_parse_name") {
                    parse_name = false;
                }
                Ok(())
            })
            .unwrap();
        }
    }
    let data_ident = &ast.ident;
    let inner = match &ast.data {
        syn::Data::Enum(enum_data) => {
            let parser_alts = enum_data
                .variants
                .iter()
                .filter(|val| {
                    !val.attrs.iter().any(|attr| {
                        let mut val = false;
                        if attr.path().is_ident(PARSER_IDENT) {
                            attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("skip") {
                                    val = true;
                                }
                                Ok(())
                            })
                            .unwrap();
                        }
                        val
                    })
                })
                .map(|val| {
                    let variant_ident = &val.ident;
                    let cons = quote! { #data_ident::#variant_ident };
                    let name_matcher = quote! { advent::lower!(#variant_ident) };
                    parse_fields(&val.fields, &cons, name_matcher, parse_name)
                })
                .collect::<Vec<_>>();
            quote! { alt((#(#parser_alts),*)) }
        }
        syn::Data::Struct(struct_data) => {
            let cons = quote! { #data_ident };
            let name_matcher = quote! { advent::lower!(#data_ident) };
            parse_fields(&struct_data.fields, &cons, name_matcher, parse_name)
        }
        syn::Data::Union(_) => unimplemented!(),
    };
    let lower_ident = data_ident.to_string().to_lowercase();
    let fn_name = syn::Ident::new(&lower_ident, Span::call_site());
    quote! {
        fn #fn_name(i: &mut &str) -> Result<#data_ident> {
            #inner.parse_next(i)
        }
    }
    .into()
}
