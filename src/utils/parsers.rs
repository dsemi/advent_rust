pub use nom::branch::alt;
pub use nom::bytes::complete::{tag, take_till};
pub use nom::character::complete::{alpha1, anychar, i32, i64, one_of, space0, space1, u32, u64};
pub use nom::combinator::{map, opt, value};
use nom::error::ParseError;
pub use nom::multi::{separated_list0, separated_list1};
pub use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
pub use nom::IResult;
use nom::Parser;

pub fn usize(i: &str) -> IResult<&str, usize> {
    map(u64, |n| n as usize)(i)
}

pub fn coord<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, (O, O), E>
where
    F: Parser<&'a str, O, E> + Copy,
    E: ParseError<&'a str>,
{
    separated_pair(f, pair(tag(","), space0), f)
}

pub fn list<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    separated_list0(pair(tag(","), space0), f)
}

macro_rules! cons1 {
    ($make:ident, $arg1:expr) => {
        cons1_inner(advent::lower!($make), $make, $arg1)
    };
}
pub(crate) use cons1;

pub fn cons1_inner<'a, E, A, Arg1, Er>(
    str_make: &'a str,
    make: fn(A) -> E,
    arg1: Arg1,
) -> impl FnMut(&'a str) -> IResult<&'a str, E, Er>
where
    A: 'a,
    E: 'a,
    Arg1: Parser<&'a str, A, Er> + 'a,
    Er: ParseError<&'a str> + 'a,
{
    map(tuple((tag(str_make), space1, arg1)), move |(_, _, a)| {
        make(a)
    })
}

macro_rules! cons2 {
    ($make:ident, $arg1:expr, $arg2:expr) => {
        cons2_inner(advent::lower!($make), $make, $arg1, $arg2)
    };
}
pub(crate) use cons2;

pub fn cons2_inner<'a, E, A, B, Arg1, Arg2, Er>(
    str_make: &'a str,
    make: fn(A, B) -> E,
    arg1: Arg1,
    arg2: Arg2,
) -> impl FnMut(&'a str) -> IResult<&'a str, E, Er>
where
    A: 'a,
    B: 'a,
    E: 'a,
    Arg1: Parser<&'a str, A, Er> + 'a,
    Arg2: Parser<&'a str, B, Er> + 'a,
    Er: ParseError<&'a str> + 'a,
{
    map(
        tuple((tag(str_make), space1, arg1, space1, arg2)),
        move |(_, _, a, _, b)| make(a, b),
    )
}
