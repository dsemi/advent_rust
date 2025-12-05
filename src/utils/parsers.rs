pub use winnow::ascii::*;
pub use winnow::combinator::*;
pub use winnow::error::*;
pub use winnow::prelude::*;
pub use winnow::stream::*;
pub use winnow::token::*;

pub trait Parser<I, O> = winnow::prelude::Parser<I, O, ContextError>;

pub trait ParserExt<I, O> {
    fn read(&mut self, i: I) -> O;
}

impl<I, O, P> ParserExt<I, O> for P
where
    I: Stream + StreamIsPartial + Clone,
    P: Parser<I, O>,
{
    fn read(&mut self, i: I) -> O {
        self.parse(i).unwrap()
    }
}

macro_rules! impl_signed {
    ($($i:ident),*) => ($(
        #[allow(dead_code)]
        pub fn $i<I>(input: &mut I) -> Result<$i>
        where
            I: StreamIsPartial + Stream,
            <I as Stream>::Slice: AsBStr + ParseSlice<$i>,
            <I as Stream>::Token: AsChar + Clone,
        {
            (opt(one_of(['-', '+'])), digit1).take().parse_to().parse_next(input)
        }
    )*)
}
impl_signed!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_unsigned {
    ($($i:ident),*) => ($(
        #[allow(dead_code)]
        pub fn $i<I>(input: &mut I) -> Result<$i>
        where
            I: StreamIsPartial + Stream,
            <I as Stream>::Slice: AsBStr + ParseSlice<$i>,
            <I as Stream>::Token: AsChar,
        {
            digit1.parse_to().parse_next(input)
        }
    )*)
}
impl_unsigned!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_float {
    ($($i:ident),*) => ($(
        #[allow(dead_code)]
        pub fn $i<I>(input: &mut I) -> Result<$i>
        where
            I: StreamIsPartial + Stream + Compare<Caseless<&'static str>> + Compare<char> + AsBStr,
            <I as Stream>::Slice: ParseSlice<$i>,
            <I as Stream>::Token: AsChar + Clone,
            <I as Stream>::IterOffsets: Clone,
        {
            float(input)
        }
    )*)
}
impl_float!(f32, f64);

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
impl_litext!(f32, f64);

pub fn separated_triplet<I, O1, O2, O3, O4, O5, E, F1, F2, F3, G1, G2>(
    mut f1: F1,
    mut sep1: G1,
    mut f2: F2,
    mut sep2: G2,
    mut f3: F3,
) -> impl winnow::prelude::Parser<I, (O1, O3, O5), E>
where
    I: Stream,
    E: ParserError<I>,
    F1: winnow::prelude::Parser<I, O1, E>,
    G1: winnow::prelude::Parser<I, O2, E>,
    F2: winnow::prelude::Parser<I, O3, E>,
    G2: winnow::prelude::Parser<I, O4, E>,
    F3: winnow::prelude::Parser<I, O5, E>,
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

pub fn sep2<I, O, O2, E, F, G>(mut f: F, mut sep: G) -> impl winnow::prelude::Parser<I, (O, O), E>
where
    I: Stream,
    E: ParserError<I>,
    F: winnow::prelude::Parser<I, O, E>,
    G: winnow::prelude::Parser<I, O2, E>,
{
    move |i: &mut I| {
        let a = f.parse_next(i)?;
        let _ = sep.parse_next(i)?;
        let b = f.parse_next(i)?;
        Ok((a, b))
    }
}

pub fn sep3<I, O, O2, E, F, G>(
    mut f: F,
    mut sep: G,
) -> impl winnow::prelude::Parser<I, (O, O, O), E>
where
    I: Stream,
    E: ParserError<I>,
    F: winnow::prelude::Parser<I, O, E>,
    G: winnow::prelude::Parser<I, O2, E>,
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

pub fn sep4<I, O, O2, E, F, G>(
    mut f: F,
    mut sep: G,
) -> impl winnow::prelude::Parser<I, (O, O, O, O), E>
where
    I: Stream,
    E: ParserError<I>,
    F: winnow::prelude::Parser<I, O, E>,
    G: winnow::prelude::Parser<I, O2, E>,
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

pub fn strip<I, O, E, F>(f: F) -> impl winnow::prelude::Parser<I, O, E>
where
    I: Stream + StreamIsPartial,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
    F: winnow::prelude::Parser<I, O, E>,
{
    delimited(space0, f, space0)
}

pub fn coord<'a, I, O, E, F>(f: F) -> impl winnow::prelude::Parser<I, (O, O), E> + 'a
where
    I: Stream + StreamIsPartial + Compare<&'a str> + Compare<char> + 'a,
    <I as Stream>::Token: AsChar + Clone,
    O: 'a,
    E: ParserError<I> + 'a,
    F: winnow::prelude::Parser<I, O, E> + 'a,
{
    sep2(strip(f), ',')
}

pub fn coord3<'a, I, O, E, F>(f: F) -> impl winnow::prelude::Parser<I, (O, O, O), E> + 'a
where
    I: Stream + StreamIsPartial + Compare<&'a str> + Compare<char> + 'a,
    <I as Stream>::Token: AsChar + Clone,
    O: 'a,
    E: ParserError<I> + 'a,
    F: winnow::prelude::Parser<I, O, E> + 'a,
{
    sep3(strip(f), ',')
}

pub fn spaced<'a, I, O, E, F>(f: F) -> impl winnow::prelude::Parser<I, Vec<O>, E> + 'a
where
    I: Stream + StreamIsPartial + Compare<&'a str> + 'a,
    <I as Stream>::Token: AsChar + Clone,
    O: 'a,
    E: ParserError<I> + 'a,
    F: winnow::prelude::Parser<I, O, E> + 'a,
{
    strip(separated(0.., f, space1))
}

pub fn list<'a, I, O, E, F>(f: F) -> impl winnow::prelude::Parser<I, Vec<O>, E> + 'a
where
    I: Stream + StreamIsPartial + Compare<&'a str> + Compare<char> + 'a,
    <I as Stream>::Token: AsChar + Clone,
    O: 'a,
    E: ParserError<I> + 'a,
    F: winnow::prelude::Parser<I, O, E> + 'a,
{
    separated(0.., f, (',', space0))
}

pub fn lines<'a, I, O, E, F>(f: F) -> impl winnow::prelude::Parser<I, Vec<O>, E> + 'a
where
    I: Stream + StreamIsPartial + Compare<&'a str> + 'a,
    O: 'a,
    E: ParserError<I> + 'a,
    F: winnow::prelude::Parser<I, O, E> + 'a,
{
    separated(0.., f, "\n")
}

// https://github.com/winnow-rs/winnow/issues/349
// pub fn sep_iter<'a, I, O, O2, E, F, G>(i: I, f: F, sep: G) -> impl Iterator<Item = O>
// where
// I: Stream,
// E: ParserError<I>,
// F: winnow::prelude::Parser<I, O, E>,
// G: winnow::prelude::Parser<I, O2, E>,
// {
// todo!()
// }

pub fn lines_iter<'a, O, F>(i: &'a str, mut f: F) -> impl Iterator<Item = O> + 'a
where
    F: Parser<&'a str, O> + 'a,
{
    i.lines().map(move |line| f.read(line))
}

macro_rules! read_trait {
    ($($i:ident),*) => (
        #[allow(dead_code)]
        pub trait ReadFromStr {
            $(fn $i(&self) -> $i;)*
        }

        impl ReadFromStr for str {
            $(fn $i(&self) -> $i {
                $i.read(self)
            })*
        }
    )
}

read_trait!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
