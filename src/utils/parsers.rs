pub use nom::branch::alt;
pub use nom::bytes::complete::take_till;
pub use nom::character::complete::{
    alpha1, anychar, digit1, hex_digit1, multispace0, one_of, space0, space1,
};
pub use nom::character::{is_hex_digit, is_space};
pub use nom::combinator::{all_consuming, map, opt, recognize, rest, value, verify};
pub use nom::error::{Error, ParseError};
pub use nom::multi::{fold_many0, fold_many1, separated_list0, separated_list1};
pub use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
pub use nom::{Finish, IResult, Parser};

use nom::{AsChar, Compare, InputIter, InputLength, InputTake, Slice};

pub fn tag<T, Input>(tag: T) -> impl Fn(Input) -> IResult<Input, Input>
where
    Input: InputTake + Compare<T>,
    T: InputLength + Clone,
{
    nom::bytes::complete::tag(tag)
}

macro_rules! impl_signed {
    ($orig:expr, $new:ident, $t:ty) => {
        #[allow(dead_code)]
        pub fn $new<T>(input: T) -> IResult<T, $t>
        where
            T: InputIter
                + Slice<std::ops::RangeFrom<usize>>
                + InputLength
                + InputTake
                + Clone
                + for<'a> Compare<&'a [u8]>,
            <T as InputIter>::Item: AsChar,
        {
            $orig(input)
        }
    };
}

impl_signed!(nom::character::complete::i8, i8, i8);
impl_signed!(nom::character::complete::i16, i16, i16);
impl_signed!(nom::character::complete::i32, i32, i32);
impl_signed!(nom::character::complete::i64, i64, i64);

macro_rules! impl_unsigned {
    ($orig:expr, $new:ident, $t:ty) => {
        #[allow(dead_code)]
        pub fn $new<T>(input: T) -> IResult<T, $t>
        where
            T: InputIter + Slice<std::ops::RangeFrom<usize>> + InputLength,
            <T as InputIter>::Item: AsChar,
        {
            $orig(input)
        }
    };
}

impl_unsigned!(nom::character::complete::u8, u8, u8);
impl_unsigned!(nom::character::complete::u16, u16, u16);
impl_unsigned!(nom::character::complete::u32, u32, u32);
impl_unsigned!(nom::character::complete::u64, u64, u64);

pub fn usize(i: &str) -> IResult<&str, usize> {
    map(u64, |n| n as usize)(i)
}

pub fn coord<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, (O, O), E>
where
    F: Parser<&'a str, O, E> + Copy,
    E: ParseError<&'a str>,
{
    separated_pair(f, pair(nom::bytes::complete::tag(","), space0), f)
}

pub fn list<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    separated_list0(pair(nom::bytes::complete::tag(","), space0), f)
}

pub fn lines<'a, O, E, F>(i: &'a str, f: F) -> Vec<O>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str> + std::fmt::Debug,
{
    all_consuming(separated_list1(nom::bytes::complete::tag("\n"), f))(i)
        .finish()
        .map(|x| x.1)
        .unwrap()
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
    map(
        tuple((nom::bytes::complete::tag(str_make), space1, arg1)),
        move |(_, _, a)| make(a),
    )
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
        tuple((
            nom::bytes::complete::tag(str_make),
            space1,
            arg1,
            space1,
            arg2,
        )),
        move |(_, _, a, _, b)| make(a, b),
    )
}
