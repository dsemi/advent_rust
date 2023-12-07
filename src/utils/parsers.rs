pub use nom::branch::alt;
pub use nom::bytes::complete::{take_till, take_while1};
pub use nom::character::complete::{
    alpha1, alphanumeric1, digit1, hex_digit1, multispace0, none_of, one_of, space0, space1,
};
pub use nom::character::{is_hex_digit, is_space};
pub use nom::combinator::{
    all_consuming, iterator, map, map_res, opt, recognize, rest, value, verify,
};
pub use nom::error::{Error, ParseError};
pub use nom::multi::{fold_many0, fold_many1, many1, separated_list0, separated_list1};
pub use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
pub use nom::{Finish, IResult, Parser};

use core::ops::RangeFrom;
use nom::{AsChar, Compare, InputIter, InputLength, InputTake, Slice};

pub trait ParserExt<I, O, E> {
    fn read(&mut self, i: I) -> O;
}

impl<I, O, E: std::fmt::Debug, P: Parser<I, O, E>> ParserExt<I, O, E> for P {
    fn read(&mut self, i: I) -> O {
        self.parse(i).unwrap().1
    }
}

pub fn anychar<T>(input: T) -> IResult<T, char>
where
    T: InputIter + InputLength + Slice<RangeFrom<usize>>,
    <T as InputIter>::Item: AsChar,
{
    nom::character::complete::anychar(input)
}

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

pub fn isize(i: &str) -> IResult<&str, isize> {
    map(i64, |n| n as isize)(i)
}

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

pub trait LitExt<O = Self> {
    fn read(i: &str) -> O;
}

macro_rules! impl_litext {
    ($t:ty, $f:expr) => {
        impl LitExt for $t {
            fn read(i: &str) -> $t {
                $f.read(i)
            }
        }
    };
}

impl_litext!(i8, i8);
impl_litext!(i16, i16);
impl_litext!(i32, i32);
impl_litext!(i64, i64);
impl_litext!(isize, isize);
impl_litext!(u8, u8);
impl_litext!(u16, u16);
impl_litext!(u32, u32);
impl_litext!(u64, u64);
impl_litext!(usize, usize);

pub fn coord<'a, O, E, F>(mut f: F) -> impl FnMut(&'a str) -> IResult<&'a str, (O, O), E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    move |i: &str| {
        let (i, _) = space0.parse(i)?;
        let (i, a) = f.parse(i)?;
        let (i, _) = pair(nom::bytes::complete::tag(","), space0).parse(i)?;
        let (i, b) = f.parse(i)?;
        let (i, _) = space0.parse(i)?;
        Ok((i, (a, b)))
    }
}

pub fn coord3<'a, O, E, F>(mut f: F) -> impl FnMut(&'a str) -> IResult<&'a str, (O, O, O), E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    move |i: &str| {
        let (i, _) = space0.parse(i)?;
        let (i, a) = f.parse(i)?;
        let (i, _) = pair(nom::bytes::complete::tag(","), space0).parse(i)?;
        let (i, b) = f.parse(i)?;
        let (i, _) = pair(nom::bytes::complete::tag(","), space0).parse(i)?;
        let (i, c) = f.parse(i)?;
        let (i, _) = space0.parse(i)?;
        Ok((i, (a, b, c)))
    }
}

pub fn sep_tuple2<'a, O, O2, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, (O, O), E>
where
    F: Parser<&'a str, O, E>,
    G: Parser<&'a str, O2, E>,
    E: ParseError<&'a str>,
{
    move |i: &str| {
        let (i, a) = f.parse(i)?;
        let (i, _) = sep.parse(i)?;
        let (i, b) = f.parse(i)?;
        Ok((i, (a, b)))
    }
}

pub fn sep_tuple3<'a, O, O2, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, (O, O, O), E>
where
    F: Parser<&'a str, O, E>,
    G: Parser<&'a str, O2, E>,
    E: ParseError<&'a str>,
{
    move |i: &str| {
        let (i, a) = f.parse(i)?;
        let (i, _) = sep.parse(i)?;
        let (i, b) = f.parse(i)?;
        let (i, _) = sep.parse(i)?;
        let (i, c) = f.parse(i)?;
        Ok((i, (a, b, c)))
    }
}

pub fn sep_tuple4<'a, O, O2, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, (O, O, O, O), E>
where
    F: Parser<&'a str, O, E>,
    G: Parser<&'a str, O2, E>,
    E: ParseError<&'a str>,
{
    move |i: &str| {
        let (i, a) = f.parse(i)?;
        let (i, _) = sep.parse(i)?;
        let (i, b) = f.parse(i)?;
        let (i, _) = sep.parse(i)?;
        let (i, c) = f.parse(i)?;
        let (i, _) = sep.parse(i)?;
        let (i, d) = f.parse(i)?;
        Ok((i, (a, b, c, d)))
    }
}

pub fn sep_list<'a, O, O2, E, F, G>(
    sep: G,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    G: Parser<&'a str, O2, E>,
    E: ParseError<&'a str>,
{
    delimited(space0, separated_list1(sep, f), space0)
}

pub fn list<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    separated_list0(pair(nom::bytes::complete::tag(","), space0), f)
}

pub fn lines_iter<'a, O, E, F>(i: &'a str, mut f: F) -> impl Iterator<Item = O> + 'a
where
    F: Parser<&'a str, O, E> + 'a,
    E: ParseError<&'a str> + std::fmt::Debug,
{
    i.lines().map(move |line| f.read(line))
}

pub fn lines<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    separated_list0(nom::bytes::complete::tag("\n"), f)
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

macro_rules! parse_decl {
    ($t:ty, $i:ident, $f:expr) => {
        fn $i(&self) -> $i;
    };
}

macro_rules! parse_impl {
    ($t:ty, $i:ident, $f:expr) => {
        fn $i(&self) -> $i {
            $f.read(self)
        }
    };
}

pub trait ParseInts {
    parse_decl!(i8, i8, i8);
    parse_decl!(i16, i16, i16);
    parse_decl!(i32, i32, i32);
    parse_decl!(i64, i64, i64);
    parse_decl!(isize, isize, isize);
    parse_decl!(u8, u8, u8);
    parse_decl!(u16, u16, u16);
    parse_decl!(u32, u32, u32);
    parse_decl!(u64, u64, u64);
    parse_decl!(usize, usize, usize);
}

impl ParseInts for str {
    parse_impl!(i8, i8, i8);
    parse_impl!(i16, i16, i16);
    parse_impl!(i32, i32, i32);
    parse_impl!(i64, i64, i64);
    parse_impl!(isize, isize, isize);
    parse_impl!(u8, u8, u8);
    parse_impl!(u16, u16, u16);
    parse_impl!(u32, u32, u32);
    parse_impl!(u64, u64, u64);
    parse_impl!(usize, usize, usize);
}
