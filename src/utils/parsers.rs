pub use winnow::ascii::*;
pub use winnow::combinator::*;
pub use winnow::error::*;
pub use winnow::prelude::*;
pub use winnow::stream::*;
pub use winnow::token::*;

// Use when trait_alias stabilizes.
// pub trait Parser<I, O> = winnow::prelude::Parser<I, O, ContextError>;

pub trait ParserExt<I, O> {
    fn read(&mut self, i: I) -> O;
}

impl<I, O, P> ParserExt<I, O> for P
where
    I: Stream + StreamIsPartial + Clone,
    P: Parser<I, O, ContextError>,
{
    fn read(&mut self, i: I) -> O {
        self.parse(i).unwrap()
    }
}

macro_rules! impl_signed {
    ($($i:ident),*) => ($(
        #[allow(dead_code)]
        pub fn $i<I>(input: &mut I) -> PResult<$i>
        where
            I: StreamIsPartial + Stream,
            <I as Stream>::Token: AsChar + Clone,
        {
            dec_int(input)
        }
    )*)
}
impl_signed!(i8, i16, i32, i64, i128);

#[allow(dead_code)]
pub fn isize<I>(input: &mut I) -> PResult<isize>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Token: AsChar + Clone,
{
    dec_int(input).map(|n: i64| n as isize)
}

macro_rules! impl_unsigned {
    ($($i:ident),*) => ($(
        #[allow(dead_code)]
        pub fn $i<I>(input: &mut I) -> PResult<$i>
        where
            I: StreamIsPartial + Stream,
            <I as Stream>::Token: AsChar + Clone,
        {
            dec_uint(input)
        }
    )*)
}
impl_unsigned!(u8, u16, u32, u64, u128);

#[allow(dead_code)]
pub fn usize<I>(input: &mut I) -> PResult<usize>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Token: AsChar + Clone,
{
    dec_uint(input).map(|n: u64| n as usize)
}

pub trait LitExt<O = Self> {
    fn read(i: &str) -> O;
}

macro_rules! impl_litext {
    ($($i:ident),*) => ($(
        impl LitExt for $i {
            fn read(i: &str) -> $i {
                $i.read(i)
            }
        }
    )*)
}

impl_litext!(i8, i16, i32, i64, i128, isize);
impl_litext!(u8, u16, u32, u64, u128, usize);

pub fn separated_triplet<I, O1, O2, O3, O4, O5, E, F1, F2, F3, G1, G2>(
    mut f1: F1,
    mut sep1: G1,
    mut f2: F2,
    mut sep2: G2,
    mut f3: F3,
) -> impl Parser<I, (O1, O3, O5), E>
where
    I: Stream,
    E: ParserError<I>,
    F1: Parser<I, O1, E>,
    G1: Parser<I, O2, E>,
    F2: Parser<I, O3, E>,
    G2: Parser<I, O4, E>,
    F3: Parser<I, O5, E>,
{
    move |i: &mut I| {
        let a = f1.parse_next(i)?;
        let _ = sep1.parse_next(i)?;
        let b = f2.parse_next(i)?;
        let _ = sep2.parse_next(i)?;
        let c = f3.parse_next(i)?;
        Ok((a, b, c))
    }
}

pub fn sep_tuple2<I, O, O2, E, F, G>(mut f: F, mut sep: G) -> impl Parser<I, (O, O), E>
where
    I: Stream,
    E: ParserError<I>,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
{
    move |i: &mut I| {
        let a = f.parse_next(i)?;
        let _ = sep.parse_next(i)?;
        let b = f.parse_next(i)?;
        Ok((a, b))
    }
}

pub fn sep_tuple3<I, O, O2, E, F, G>(mut f: F, mut sep: G) -> impl Parser<I, (O, O, O), E>
where
    I: Stream,
    E: ParserError<I>,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
{
    move |i: &mut I| {
        let a = f.parse_next(i)?;
        let _ = sep.parse_next(i)?;
        let b = f.parse_next(i)?;
        let _ = sep.parse_next(i)?;
        let c = f.parse_next(i)?;
        Ok((a, b, c))
    }
}

pub fn sep_tuple4<I, O, O2, E, F, G>(mut f: F, mut sep: G) -> impl Parser<I, (O, O, O, O), E>
where
    I: Stream,
    E: ParserError<I>,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
{
    move |i: &mut I| {
        let a = f.parse_next(i)?;
        let _ = sep.parse_next(i)?;
        let b = f.parse_next(i)?;
        let _ = sep.parse_next(i)?;
        let c = f.parse_next(i)?;
        let _ = sep.parse_next(i)?;
        let d = f.parse_next(i)?;
        Ok((a, b, c, d))
    }
}

pub fn strip<I, O, E, F>(f: F) -> impl Parser<I, O, E>
where
    I: Stream + StreamIsPartial,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
    F: Parser<I, O, E>,
{
    delimited(space0, f, space0)
}

pub fn coord<'a, I, O, E, F>(f: F) -> impl Parser<I, (O, O), E> + 'a
where
    I: Stream + StreamIsPartial + Compare<&'a str> + 'a,
    <I as Stream>::Token: AsChar + Clone,
    O: 'a,
    E: ParserError<I> + 'a,
    F: Parser<I, O, E> + 'a,
{
    strip(sep_tuple2(f, (",", space0)))
}

pub fn coord3<'a, I, O, E, F>(f: F) -> impl Parser<I, (O, O, O), E> + 'a
where
    I: Stream + StreamIsPartial + Compare<&'a str> + 'a,
    <I as Stream>::Token: AsChar + Clone,
    O: 'a,
    E: ParserError<I> + 'a,
    F: Parser<I, O, E> + 'a,
{
    strip(sep_tuple3(f, (",", space0)))
}

pub fn list<'a, I, O, E, F>(f: F) -> impl Parser<I, Vec<O>, E> + 'a
where
    I: Stream + StreamIsPartial + Compare<&'a str> + 'a,
    <I as Stream>::Token: AsChar + Clone,
    O: 'a,
    E: ParserError<I> + 'a,
    F: Parser<I, O, E> + 'a,
{
    separated(0.., f, (',', space0))
}

pub fn lines<'a, I, O, E, F>(f: F) -> impl Parser<I, Vec<O>, E> + 'a
where
    I: Stream + StreamIsPartial + Compare<&'a str> + 'a,
    O: 'a,
    E: ParserError<I> + 'a,
    F: Parser<I, O, E> + 'a,
{
    separated(0.., f, "\n")
}

pub fn lines_iter<'a, O, F>(i: &'a str, mut f: F) -> impl Iterator<Item = O> + 'a
where
    F: Parser<&'a str, O, ContextError> + 'a,
{
    i.lines().map(move |line| f.read(line))
}

macro_rules! cons1 {
    ($make:ident, $arg1:expr) => {
        cons1_inner(advent::lower!($make), $make, $arg1)
    };
}
pub(crate) use cons1;

pub fn cons1_inner<'a, A, Arg1, I, O, E>(
    str_make: &'a str,
    make: fn(A) -> O,
    arg1: Arg1,
) -> impl Parser<I, O, E> + 'a
where
    A: 'a,
    Arg1: Parser<I, A, E> + 'a,
    I: Stream + StreamIsPartial + Compare<&'a str> + 'a,
    <I as Stream>::Token: AsChar + Clone,
    O: 'a,
    E: ParserError<I> + 'a,
{
    (str_make, space1, arg1).map(move |(_, _, a)| make(a))
}

macro_rules! cons2 {
    ($make:ident, $arg1:expr, $arg2:expr) => {
        cons2_inner(advent::lower!($make), $make, $arg1, $arg2)
    };
}
pub(crate) use cons2;

pub fn cons2_inner<'a, A, B, Arg1, Arg2, I, O, E>(
    str_make: &'a str,
    make: fn(A, B) -> O,
    arg1: Arg1,
    arg2: Arg2,
) -> impl Parser<I, O, E> + 'a
where
    A: 'a,
    B: 'a,
    Arg1: Parser<I, A, E> + 'a,
    Arg2: Parser<I, B, E> + 'a,
    I: Stream + StreamIsPartial + Compare<&'a str> + 'a,
    <I as Stream>::Token: AsChar + Clone,
    O: 'a,
    E: ParserError<I> + 'a,
{
    (str_make, space1, arg1, space1, arg2).map(move |(_, _, a, _, b)| make(a, b))
}

macro_rules! parse_decl {
    ($($i:ident),*) => ($(
        fn $i(&self) -> $i;
    )*)
}

macro_rules! parse_impl {
    ($($i:ident),*) => ($(
        fn $i(&self) -> $i {
            $i.read(self)
        }
    )*)
}

pub trait ParseInts {
    parse_decl!(i8, i16, i32, i64, i128, isize);
    parse_decl!(u8, u16, u32, u64, u128, usize);
}

impl ParseInts for str {
    parse_impl!(i8, i16, i32, i64, i128, isize);
    parse_impl!(u8, u16, u32, u64, u128, usize);
}
