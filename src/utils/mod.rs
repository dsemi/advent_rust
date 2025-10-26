use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use nalgebra::Vector2;
use nalgebra::Vector3;
use num::cast::AsPrimitive;
use num::pow::Pow;
use num::traits::SaturatingAdd;
use num::{Bounded, FromPrimitive, Num, One, PrimInt, Signed, Zero};
use smallvec::SmallVec;
use std::cmp::Ordering::*;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BinaryHeap, VecDeque};
use std::convert::{From, identity};
use std::hash::Hash;
use std::iter::Sum;
use std::ops::Deref;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Rem, Shr,
    ShrAssign, Sub, SubAssign,
};
use streaming_iterator::StreamingIterator;

pub mod ocr;
pub mod parsers;

pub fn bfs<T, F, I, I2>(start: T, neighbs: F) -> Bfs<T, F, impl Fn(&T) -> T, T>
where
    T: Clone + Eq + Hash,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = T, IntoIter = I2>,
    I2: Iterator<Item = T>,
{
    bfs_on(|x| x.clone(), [start], neighbs)
}

pub fn bfs_m<T, J, J2, F, I, I2>(starts: J, neighbs: F) -> Bfs<T, F, impl Fn(&T) -> T, T>
where
    T: Clone + Eq + Hash,
    J: IntoIterator<Item = T, IntoIter = J2>,
    J2: Iterator<Item = T>,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = T, IntoIter = I2>,
    I2: Iterator<Item = T>,
{
    bfs_on(|x| x.clone(), starts, neighbs)
}

pub fn bfs_on<T, J, J2, F, G, H, I, I2>(h: G, starts: J, neighbs: F) -> Bfs<T, F, G, H>
where
    H: Eq + Hash,
    G: Fn(&T) -> H,
    J: IntoIterator<Item = T, IntoIter = J2>,
    J2: Iterator<Item = T>,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = T, IntoIter = I2>,
    I2: Iterator<Item = T>,
{
    let mut visited = HashSet::new();
    let mut frontier = VecDeque::new();
    for start in starts {
        visited.insert(h(&start));
        frontier.push_back((0, start));
    }
    Bfs { frontier, visited, hash: h, neighbs }
}

pub struct Bfs<T, F, G, H> {
    pub frontier: VecDeque<(usize, T)>,
    pub visited: HashSet<H>,
    pub hash: G,
    pub neighbs: F,
}

impl<T, F, G, H, I, I2> Iterator for Bfs<T, F, G, H>
where
    H: Eq,
    H: Hash,
    G: Fn(&T) -> H,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = T, IntoIter = I2>,
    I2: Iterator<Item = T>,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<(usize, T)> {
        let (d, item) = self.frontier.pop_front()?;
        for neighb in (self.neighbs)(&item) {
            if self.visited.insert((self.hash)(&neighb)) {
                self.frontier.push_back((d + 1, neighb));
            }
        }
        Some((d, item))
    }
}

#[derive(Eq, PartialEq)]
struct State<T> {
    dist: usize,
    elem: T,
}

impl<T: Eq> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl<T: Eq> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Dijkstra<T, F> {
    queue: BinaryHeap<Reverse<State<T>>>,
    dists: HashMap<T, usize>,
    neighbors: F,
}

pub fn dijkstra<T, F, I, I2>(start: T, neighbors: F) -> Dijkstra<T, F>
where
    T: Clone,
    T: Eq,
    T: Hash,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = (usize, T), IntoIter = I2>,
    I2: Iterator<Item = (usize, T)>,
{
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(State { dist: 0, elem: start }));
    Dijkstra { queue, dists: HashMap::new(), neighbors }
}

impl<T, F, I, I2> Iterator for Dijkstra<T, F>
where
    T: Clone,
    T: Eq,
    T: Hash,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = (usize, T), IntoIter = I2>,
    I2: Iterator<Item = (usize, T)>,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        let Reverse(state) = self.queue.pop()?;
        let shortest = self.dists.entry(state.elem.clone()).or_insert(state.dist);
        if state.dist <= *shortest {
            *shortest = state.dist;
            let neighbs = (self.neighbors)(&state.elem);
            for (d, st2) in neighbs {
                let dist = state.dist + d;
                let shortest = self.dists.entry(st2.clone()).or_insert(dist + 1);
                if dist < *shortest {
                    *shortest = dist;
                    self.queue.push(Reverse(State { dist, elem: st2 }));
                }
            }
        }
        Some((state.dist, state.elem))
    }
}

pub fn transpose<T: Copy>(inp: &[Vec<T>]) -> Vec<Vec<T>> {
    let cols = inp.iter().map(|x| x.len()).max().unwrap();
    let mut out = vec![vec![]; cols];
    for i in 0..cols {
        for row in inp.iter() {
            if i < row.len() {
                out[i].push(row[i]);
            }
        }
    }
    out
}

pub trait AbsDiff<T = Self> {
    fn abs_diff(self, other: T) -> T;
}

macro_rules! impl_abs_diff {
    ($($i:ident),*) => ($(
        impl AbsDiff<$i> for $i {
            #[inline]
            fn abs_diff(self, other: $i) -> $i {
                self.abs_diff(other) as $i
            }
        }
    )*)
}

impl_abs_diff!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

macro_rules! impl_num_op {
    (impl $imp:ident::$method:ident for $t:ty, ($($acc:tt is $ins:tt),*)) => {
        impl<$($ins: $imp<Output = $ins>),*> $imp<$t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: $t) -> Self::Output {
                Self($($ins::$method(self.$acc, other.$acc)),*)
            }
        }
    };
}

macro_rules! forward_ref_num_op {
    (impl $imp:ident::$method:ident for $t:ty, ($($ins:tt),*)) => {
        impl<'a, $($ins: Copy + $imp<Output = $ins>),*> $imp<$t> for &'a $t {
            type Output = <$t as $imp<$t>>::Output;

            #[inline]
            fn $method(self, other: $t) -> <$t as $imp<$t>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<$($ins: Copy + $imp<Output = $ins>),*> $imp<&$t> for $t {
            type Output = <$t as $imp<$t>>::Output;

            #[inline]
            fn $method(self, other: &$t) -> <$t as $imp<$t>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<$($ins: Copy + $imp<Output = $ins>),*> $imp<&$t> for &$t {
            type Output = <$t as $imp<$t>>::Output;

            #[inline]
            fn $method(self, other: &$t) -> <$t as $imp<$t>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! num_assign {
    (impl $imp:ident::$method:ident via $delegate:ident::$method_delegate:ident for $t:ty, ($($ins:tt$(: $bound:tt $(+ $rest:tt)*)?),*)) => {
        impl<$($ins: Copy + $delegate<Output = $ins> $(+ $bound $(+ $rest)*)?),*> $imp<$t> for $t {
            #[inline]
            fn $method(&mut self, other: $t) {
                *self = (*self).$method_delegate(other);
            }
        }
    };
}

macro_rules! broadcast {
    ($method:ident, $t:ty, $rt:ty, ($($acc:tt is $ins:tt$(: $bound:tt $(+ $rest:tt)*)?),*)) => {
        impl<$($ins$(: $bound $(+ $rest)*)?),*> $t {
            #[inline]
            pub fn $method(&self) -> $rt {
                Self($(self.$acc.$method()),*)
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct C<T, U = T>(pub T, pub U);

mod cparse {
    use winnow::Parser;
    use winnow::error::ParserError;
    use winnow::stream::{AsChar, Compare, Stream, StreamIsPartial};

    pub fn c<'a, I, O, E, F>(f: F) -> impl Parser<I, super::C<O>, E> + 'a
    where
        I: Stream + StreamIsPartial + Compare<&'a str> + Compare<char> + 'a,
        <I as Stream>::Token: AsChar + Clone,
        O: 'a,
        E: ParserError<I> + 'a,
        F: Parser<I, O, E> + 'a,
    {
        super::parsers::coord(f).output_into()
    }
}
pub use cparse::*;

impl<A, B> C<A, B> {
    pub fn as_<U: Copy + 'static, V: Copy + 'static>(self) -> C<U, V>
    where
        A: AsPrimitive<U>,
        B: AsPrimitive<V>,
    {
        C(self.0.as_(), self.1.as_())
    }
}

impl<T> C<T> {
    pub fn vec(self) -> Vector2<T> {
        Vector2::new(self.0, self.1)
    }
}

impl<T: Num + Copy> C<T> {
    pub fn sum(&self) -> T {
        self.0 + self.1
    }

    pub fn product(&self) -> T {
        self.0 * self.1
    }

    pub fn pow<N: Num + BitAnd<Output = N> + Shr<Output = N> + Copy>(self, n: N) -> Self {
        if n.is_zero() {
            Self(T::one(), T::zero())
        } else if (n & N::one()).is_zero() {
            (self * self).pow(n >> N::one())
        } else {
            self * self.pow(n - N::one())
        }
    }
}

broadcast!(abs, C<A, B>, C<A, B>, (0 is A: Signed, 1 is B: Signed));
broadcast!(signum, C<A, B>, C<A, B>, (0 is A: Signed, 1 is B: Signed));

impl<T: Num + AbsDiff<T> + Copy> C<T> {
    pub fn dist(&self, other: &Self) -> T {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl<A: Ord, B: Ord> C<A, B> {
    pub fn smol(self, Self(c, d): Self) -> Self {
        let Self(a, b) = self;
        Self(min(a, c), min(b, d))
    }

    pub fn swol(self, Self(c, d): Self) -> Self {
        let Self(a, b) = self;
        Self(max(a, c), max(b, d))
    }
}

impl_num_op!(impl Add::add for C<A, B>, (0 is A, 1 is B));
forward_ref_num_op!(impl Add::add for C<A, B>, (A, B));
impl_num_op!(impl Sub::sub for C<A, B>, (0 is A, 1 is B));
forward_ref_num_op!(impl Sub::sub for C<A, B>, (A, B));

num_assign!(impl AddAssign::add_assign via Add::add for C<A, B>, (A, B));
num_assign!(impl SubAssign::sub_assign via Sub::sub for C<A, B>, (A, B));

impl<T: Num + Copy> Mul for C<T> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0 - self.1 * other.1, self.0 * other.1 + self.1 * other.0)
    }
}

num_assign!(impl MulAssign::mul_assign via Mul::mul for C<A>, (A: Num));

impl<T, A, B> Mul<T> for C<A, B>
where
    T: Num + Copy,
    A: Mul<T, Output = A>,
    B: Mul<T, Output = B>,
{
    type Output = Self;

    #[inline]
    fn mul(self, n: T) -> Self {
        Self(self.0 * n, self.1 * n)
    }
}

impl<T: Num + Copy> Div<T> for C<T> {
    type Output = Self;

    #[inline]
    fn div(self, n: T) -> Self {
        Self(self.0 / n, self.1 / n)
    }
}

impl<T: Num + Copy> Rem<T> for C<T> {
    type Output = Self;

    #[inline]
    fn rem(self, n: T) -> Self {
        Self(self.0 % n, self.1 % n)
    }
}

impl<A: Neg<Output = A>, B: Neg<Output = B>> Neg for C<A, B> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl<T: Num + Copy> Sum for C<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(T::zero(), T::zero()), |a, b| a + b)
    }
}

impl<T> From<(T, T)> for C<T> {
    #[inline]
    fn from((a, b): (T, T)) -> Self {
        C(a, b)
    }
}

impl<T> From<C<T>> for (T, T) {
    #[inline]
    fn from(C(a, b): C<T>) -> Self {
        (a, b)
    }
}

pub fn adjacents<T>(C(x, y): C<T>) -> impl Iterator<Item = C<T>>
where
    T: Add<Output = T> + Sub<Output = T> + One + Copy,
{
    [
        C(x - T::one(), y - T::one()),
        C(x - T::one(), y),
        C(x - T::one(), y + T::one()),
        C(x, y - T::one()),
        C(x, y + T::one()),
        C(x + T::one(), y - T::one()),
        C(x + T::one(), y),
        C(x + T::one(), y + T::one()),
    ]
    .into_iter()
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct C3<A, B = A, C = A>(pub A, pub B, pub C);

mod c3parse {
    use winnow::Parser;
    use winnow::error::ParserError;
    use winnow::stream::{AsChar, Compare, Stream, StreamIsPartial};

    pub fn c3<'a, I, O, E, F>(f: F) -> impl Parser<I, super::C3<O>, E> + 'a
    where
        I: Stream + StreamIsPartial + Compare<&'a str> + Compare<char> + 'a,
        <I as Stream>::Token: AsChar + Clone,
        O: 'a,
        E: ParserError<I> + 'a,
        F: Parser<I, O, E> + 'a,
    {
        super::parsers::coord3(f).output_into()
    }
}
pub use c3parse::*;

impl<T> C3<T> {
    pub fn xy(self) -> C<T> {
        C(self.0, self.1)
    }

    pub fn as_<U: Copy + 'static>(self) -> C3<U>
    where
        T: AsPrimitive<U>,
    {
        C3(self.0.as_(), self.1.as_(), self.2.as_())
    }

    pub fn vec(self) -> Vector3<T> {
        Vector3::new(self.0, self.1, self.2)
    }
}

impl<T: Num + Copy> C3<T> {
    pub fn sum(&self) -> T {
        self.0 + self.1 + self.2
    }

    pub fn product(&self) -> T {
        self.0 * self.1 * self.2
    }
}

broadcast!(abs, C3<A, B, C>, C3<A, B, C>, (0 is A: Signed, 1 is B: Signed, 2 is C: Signed));
broadcast!(signum, C3<A, B, C>, C3<A, B, C>, (0 is A: Signed, 1 is B: Signed, 2 is C: Signed));

impl<T: Num + AbsDiff<T> + Copy> C3<T> {
    pub fn dist(&self, other: &Self) -> T {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2)
    }
}

impl<T: Ord + Copy> C3<T> {
    pub fn smol(self, o: Self) -> Self {
        Self(min(self.0, o.0), min(self.1, o.1), min(self.2, o.2))
    }

    pub fn swol(self, o: Self) -> Self {
        Self(max(self.0, o.0), max(self.1, o.1), max(self.2, o.2))
    }
}

impl_num_op!(impl Add::add for C3<A, B, C>, (0 is A, 1 is B, 2 is C));
forward_ref_num_op!(impl Add::add for C3<A, B, C>, (A, B, C));
impl_num_op!(impl Sub::sub for C3<A, B, C>, (0 is A, 1 is B, 2 is C));
forward_ref_num_op!(impl Sub::sub for C3<A, B, C>, (A, B, C));

num_assign!(impl AddAssign::add_assign via Add::add for C3<A, B, C>, (A, B, C));
num_assign!(impl SubAssign::sub_assign via Sub::sub for C3<A, B, C>, (A, B, C));

impl<T: Num + Copy> Mul<T> for C3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, n: T) -> Self {
        Self(self.0 * n, self.1 * n, self.2 * n)
    }
}

impl<T: Num + Copy> Div<T> for C3<T> {
    type Output = Self;

    #[inline]
    fn div(self, n: T) -> Self {
        Self(self.0 / n, self.1 / n, self.2 / n)
    }
}

impl<T: Neg<Output = T>> Neg for C3<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl<T: Num + Copy> Sum for C3<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(T::zero(), T::zero(), T::zero()), |a, b| a + b)
    }
}

impl<T> From<(T, T, T)> for C3<T> {
    fn from((a, b, c): (T, T, T)) -> Self {
        C3(a, b, c)
    }
}

impl<T> From<C3<T>> for (T, T, T) {
    fn from(C3(a, b, c): C3<T>) -> Self {
        (a, b, c)
    }
}

fn mul_inv(mut a: i64, b0: i64) -> i64 {
    let mut b = b0;
    let (mut x0, mut x1) = (0, 1);
    if b == 1 {
        return 1;
    }
    while a > 1 {
        let q = a.div_euclid(b);
        let r = a.rem_euclid(b);
        a = b;
        b = r;
        let old_x0 = x0;
        x0 = x1 - q * x0;
        x1 = old_x0;
    }
    if x1 < 0 {
        x1 += b0;
    }
    x1
}

pub fn chinese_remainder(an: Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    let prod = an.iter().map(|x| x.1).product();
    for (a_i, n_i) in &an {
        let p = prod / n_i;
        sum += a_i * mul_inv(p, *n_i) * p;
    }
    sum.rem_euclid(prod)
}

pub struct PrimeFactors<I: Iterator<Item = u64>> {
    n: u64,
    fs: I,
}

impl<I: Iterator<Item = u64>> Iterator for PrimeFactors<I> {
    type Item = (u64, u32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.n == 1 {
                return None;
            }
            if let Some(f) = self.fs.next() {
                let mut cnt = 0;
                while self.n.is_multiple_of(f) {
                    cnt += 1;
                    self.n /= f;
                }
                if cnt > 0 {
                    return Some((f, cnt));
                }
            } else {
                let n = self.n;
                self.n = 1;
                return Some((n, 1));
            }
        }
    }
}

impl<I: Iterator<Item = u64>> PrimeFactors<I> {
    pub fn sum_divisors(self) -> u64 {
        self.map(|(p, a)| (p.pow(a + 1) - 1) / (p - 1)).product()
    }
}

pub fn prime_factors(n: u64) -> PrimeFactors<impl Iterator<Item = u64>> {
    let sqrt = (n as f64).sqrt() as u64;
    PrimeFactors { n, fs: std::iter::once(2).chain((3..=sqrt).step_by(2)) }
}

pub struct Partitions {
    buf: Vec<i32>,
    stack: Vec<(usize, i32, i32)>,
    in_progress: bool,
}

impl Partitions {
    pub fn new(len: usize, tot: i32) -> Self {
        Self { buf: vec![0; len + 1], stack: vec![(len, 0, tot)], in_progress: true }
    }
}

impl StreamingIterator for Partitions {
    type Item = [i32];

    fn advance(&mut self) {
        while let Some((n, y, t)) = self.stack.pop() {
            self.buf[n] = y;
            if n == 1 {
                self.buf[0] = t;
                return;
            }
            for x in 0..=t {
                self.stack.push((n - 1, x, t - x))
            }
        }
        self.in_progress = false;
    }

    fn get(&self) -> Option<&Self::Item> {
        self.in_progress.then(|| &self.buf[..self.buf.len() - 1])
    }
}

pub struct Combinations<'a, T> {
    buf: Vec<&'a T>,
    stack: Vec<(&'a [T], &'a T, usize)>,
    in_progress: bool,
}

impl<'a, T> Combinations<'a, T> {
    pub fn new(input: &'a [T], len: usize) -> Self {
        Self {
            buf: vec![&input[0]; len + 1],
            stack: vec![(input, &input[0], 0)],
            in_progress: true,
        }
    }
}

pub trait CombinationsExt<'a, T> {
    fn combinations(&'a self, len: usize) -> Combinations<'a, T>;
}

impl<'a, T> CombinationsExt<'a, T> for [T] {
    fn combinations(&'a self, len: usize) -> Combinations<'a, T> {
        Combinations::new(self, len)
    }
}

impl<'a, T> StreamingIterator for Combinations<'a, T> {
    type Item = [&'a T];

    fn advance(&mut self) {
        while let Some((xs, v, n)) = self.stack.pop() {
            self.buf[n] = v;
            if n == self.buf.len() - 1 {
                return;
            }
            for i in (0..xs.len()).rev() {
                self.stack.push((&xs[i + 1..], &xs[i], n + 1))
            }
        }
        self.in_progress = false;
    }

    fn get(&self) -> Option<&Self::Item> {
        self.in_progress.then(|| &self.buf[1..])
    }
}

pub fn bits<T>(n: T) -> Bits<T> {
    Bits { n }
}

pub struct Bits<T> {
    n: T,
}

impl<T: PartialEq + PrimInt + BitAndAssign + Zero + One> Iterator for Bits<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == T::zero() {
            return None;
        }
        let b = self.n.trailing_zeros();
        self.n &= self.n - T::one();
        Some(b as usize)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Interval<T> {
    pub lo: T,
    pub hi: T,
}

impl<T> Interval<T> {
    pub fn new(lo: T, hi: T) -> Self {
        Self { lo, hi }
    }
}

impl<T: Copy> Interval<T> {
    pub fn range(&self) -> std::ops::Range<T> {
        self.lo..self.hi
    }
}

impl<T: Copy + Sub<Output = T>> Interval<T> {
    pub fn len(&self) -> T {
        self.hi - self.lo
    }
}

impl<T: Copy + Ord + Sub<Output = T>> Interval<T> {
    pub fn intersects(&self, o: &Self) -> bool {
        self.lo < o.hi && o.lo < self.hi
    }

    pub fn intersect(&self, o: &Self) -> Self {
        Self::new(max(self.lo, o.lo), min(self.hi, o.hi))
    }

    pub fn union(&self, o: &Self) -> Option<Self> {
        (self.lo <= o.hi && o.lo <= self.hi)
            .then(|| Self::new(min(self.lo, o.lo), max(self.hi, o.hi)))
    }

    pub fn contains(&self, v: T) -> bool {
        self.lo <= v && v < self.hi
    }

    pub fn valid(&self) -> bool {
        self.lo < self.hi
    }

    pub fn clamp_lo(&self, mut lo: T) -> Option<Self> {
        lo = self.lo.max(lo);
        (lo < self.hi).then(|| Interval::new(lo, self.hi))
    }

    pub fn clamp_hi(&self, mut hi: T) -> Option<Self> {
        hi = self.hi.min(hi);
        (hi > self.lo).then(|| Interval::new(self.lo, hi))
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Interval<T> {
    type Output = Self;

    fn add(self, diff: T) -> Self::Output {
        Interval::new(self.lo + diff, self.hi + diff)
    }
}

impl<T: AddAssign + Copy> AddAssign<T> for Interval<T> {
    fn add_assign(&mut self, diff: T) {
        self.lo += diff;
        self.hi += diff;
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Interval<T> {
    type Output = Self;

    fn sub(self, diff: T) -> Self::Output {
        Interval::new(self.lo - diff, self.hi - diff)
    }
}

impl<T: SubAssign + Copy> SubAssign<T> for Interval<T> {
    fn sub_assign(&mut self, diff: T) {
        self.lo -= diff;
        self.hi -= diff;
    }
}

impl<T: Ord + Copy> Sub<Interval<T>> for Interval<T> {
    type Output = SmallVec<[Self; 2]>;

    fn sub(self, other: Interval<T>) -> Self::Output {
        let mut result = Self::Output::new();
        if other.lo > self.lo {
            result.push(Interval::new(self.lo, min(self.hi, other.lo)));
        }
        if other.hi < self.hi {
            result.push(Interval::new(max(self.lo, other.hi), self.hi));
        }
        result
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect<T> {
    pub l: Interval<T>,
    pub w: Interval<T>,
}

impl<T> Rect<T> {
    pub fn new(l_lo: T, l_hi: T, w_lo: T, w_hi: T) -> Self {
        Self { l: Interval::new(l_lo, l_hi), w: Interval::new(w_lo, w_hi) }
    }
}

impl<T: Copy + Mul<Output = T> + Sub<Output = T>> Rect<T> {
    pub fn area(&self) -> T {
        self.l.len() * self.w.len()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cube<T> {
    pub l: Interval<T>,
    pub w: Interval<T>,
    pub h: Interval<T>,
}

impl<T> Cube<T> {
    pub fn new(l_lo: T, l_hi: T, w_lo: T, w_hi: T, h_lo: T, h_hi: T) -> Self {
        Self {
            l: Interval::new(l_lo, l_hi),
            w: Interval::new(w_lo, w_hi),
            h: Interval::new(h_lo, h_hi),
        }
    }
}

impl<T: Copy + Mul<Output = T> + Sub<Output = T>> Cube<T> {
    pub fn volume(&self) -> T {
        self.l.len() * self.w.len() * self.h.len()
    }
}

impl<T: Copy + Ord + Sub<Output = T>> Cube<T> {
    pub fn intersects(&self, o: &Self) -> bool {
        self.l.intersects(&o.l) && self.w.intersects(&o.w) && self.h.intersects(&o.h)
    }

    pub fn intersect(&self, o: &Self) -> Self {
        Self { l: self.l.intersect(&o.l), w: self.w.intersect(&o.w), h: self.h.intersect(&o.h) }
    }
}

pub struct UniqueIdx<T> {
    m: HashMap<T, usize>,
}

impl<T: Eq + Hash> UniqueIdx<T> {
    pub fn new() -> Self {
        UniqueIdx { m: HashMap::new() }
    }

    pub fn idx(&mut self, k: T) -> usize {
        let c = self.m.len();
        *self.m.entry(k).or_insert(c)
    }

    pub fn idx_with<F: FnMut()>(&mut self, k: T, mut f: F) -> usize {
        let c = self.m.len();
        *self.m.entry(k).or_insert_with(|| {
            f();
            c
        })
    }

    pub fn len(&self) -> usize {
        self.m.len()
    }
}

impl<T: Eq + Hash> FromIterator<T> for UniqueIdx<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ui = Self::new();
        iter.into_iter().for_each(|elem| {
            ui.idx(elem);
        });
        ui
    }
}

pub struct MapWindows<I: Iterator, F, T, const N: usize>
where
    F: FnMut([&I::Item; N]) -> T,
{
    iter: I,
    f: F,
    buf: VecDeque<I::Item>,
}

impl<I: Iterator, F, T, const N: usize> MapWindows<I, F, T, N>
where
    F: FnMut([&I::Item; N]) -> T,
{
    fn new(mut iter: I, f: F) -> Self {
        let buf = iter.by_ref().take(N - 1).collect();
        Self { iter, f, buf }
    }
}

impl<I: Iterator, F, T, const N: usize> Iterator for MapWindows<I, F, T, N>
where
    F: FnMut([&I::Item; N]) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|next| {
            self.buf.push_back(next);
            let res = (self.f)(std::array::from_fn(|i| &self.buf[i]));
            self.buf.pop_front();
            res
        })
    }
}

pub trait MapWindowsIterator: Iterator {
    fn map_windows<T, F, const N: usize>(self, f: F) -> MapWindows<Self, F, T, N>
    where
        Self: Sized,
        F: FnMut([&Self::Item; N]) -> T,
    {
        MapWindows::new(self, f)
    }
}

impl<I: Iterator> MapWindowsIterator for I {}

pub fn binary_search_by<T, F>(lo: T, hi: T, mut f: F) -> Result<T, T>
where
    T: Num + Copy + Ord,
    F: FnMut(&T) -> Ordering,
{
    let mut size = hi - lo;
    let mut left = lo;
    let mut right = hi;
    while left < right {
        let mid = left + size / (T::one() + T::one());
        match f(&mid) {
            Less => left = mid + T::one(),
            Greater => right = mid,
            Equal => return Ok(mid),
        }
        size = right - left;
    }
    Err(left)
}

pub fn partition_point<T, P>(lo: T, hi: T, mut pred: P) -> T
where
    T: Num + Copy + Ord,
    P: FnMut(&T) -> bool,
{
    binary_search_by(lo, hi, |x| if pred(x) { Less } else { Greater }).unwrap_or_else(|i| i)
}

pub fn floyd_warshall<T, U>(dists: &mut [U])
where
    T: Ord + SaturatingAdd,
    U: AsRef<[T]> + AsMut<[T]>,
{
    // Assumes dists is square.
    for k in 0..dists.len() {
        for i in 0..dists.len() {
            for j in 0..dists.len() {
                let dist = dists[i].as_ref()[k].saturating_add(&dists[k].as_ref()[j]);
                let e = &mut dists[i].as_mut()[j];
                if dist < *e {
                    *e = dist;
                }
            }
        }
    }
}

pub fn held_karp<T: Copy + Add<Output = T>>(
    adj: &[impl Index<usize, Output = T>],
    f: fn(T, T) -> T,
) -> Option<T> {
    let len = adj.len();
    let mut g = HashMap::new();
    for k in 1..len {
        g.insert((vec![k], k), adj[0][k]);
    }
    for s in 2..len {
        for set in (1..len).combinations(s) {
            for k in &set {
                g.insert(
                    (set.clone(), *k),
                    set.iter()
                        .filter(|&m| m != k)
                        .map(|&m| {
                            let mut set_not_k = set.clone();
                            set_not_k.retain(|i| i != k);
                            g[&(set_not_k, m)] + adj[m][*k]
                        })
                        .reduce(f)
                        .unwrap(),
                );
            }
        }
    }
    (1..len).map(move |k| g[&((1..len).collect(), k)] + adj[k][0]).reduce(f)
}

#[derive(Clone)]
struct UnionFindNode<T> {
    val: T,
    parent: usize,
    rank: usize,
}

#[derive(Clone)]
pub struct UnionFind<T> {
    nodes: Vec<UnionFindNode<T>>,
}

impl<T: Eq + Hash> UnionFind<T> {
    pub fn new() -> Self {
        UnionFind { nodes: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.nodes.push(UnionFindNode { val, parent: self.nodes.len(), rank: 0 })
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        match self.nodes[x_root].rank.cmp(&self.nodes[y_root].rank) {
            Less => self.nodes[x_root].parent = y_root,
            Greater => self.nodes[y_root].parent = x_root,
            Equal => {
                self.nodes[y_root].parent = x_root;
                self.nodes[x_root].rank += 1;
            }
        }
    }

    pub fn find(&mut self, k: usize) -> usize {
        if self.nodes[k].parent != k {
            self.nodes[k].parent = self.find(self.nodes[k].parent);
            return self.nodes[k].parent;
        }
        k
    }

    pub fn ncomponents(&mut self) -> usize {
        (0..self.nodes.len()).map(|p| self.find(p)).collect::<HashSet<_>>().len()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<T> Index<usize> for UnionFind<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.nodes[idx].val
    }
}

impl<T: Eq + Hash> FromIterator<T> for UnionFind<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut uf = Self::new();
        iter.into_iter().for_each(|elem| uf.push(elem));
        uf
    }
}

pub fn replace_with<T, F: FnOnce(&T) -> T>(r: &mut T, f: F) -> T {
    let r2 = f(r);
    std::mem::replace(r, r2)
}

pub trait Counter: Iterator {
    fn counts(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut m = if let Some(size) = self.size_hint().1 {
            HashMap::with_capacity(size)
        } else {
            HashMap::new()
        };
        self.for_each(|item| *m.entry(item).or_default() += 1);
        m
    }

    fn most_common(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let cnts = self.counts();
        cnts.into_iter().max_by_key(|x| x.1).map(|x| x.0)
    }

    fn least_common(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let cnts = self.counts();
        cnts.into_iter().min_by_key(|x| x.1).map(|x| x.0)
    }

    fn most_common_ordered(self) -> Vec<(Self::Item, usize)>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Ord,
    {
        let mut cnts: Vec<_> = self.counts().into_iter().collect();
        cnts.sort_unstable_by(|(k1, v1), (k2, v2)| v2.cmp(v1).then(k1.cmp(k2)));
        cnts
    }
}

impl<T: ?Sized> Counter for T where T: Iterator {}

pub fn mod_exp<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: Num + PartialOrd + ShrAssign<T> + Copy + Bounded,
{
    let one: T = One::one();
    let two: T = one + one;
    let zero: T = Zero::zero();
    let max: T = Bounded::max_value();

    assert!((modulus - one) < (max / (modulus - one)));

    let mut result = one;
    base = base % modulus;

    loop {
        if exp <= zero {
            break;
        }
        if exp % two == one {
            result = (result * base) % modulus;
        }
        exp >>= one;
        base = (base * base) % modulus;
    }

    result
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Mod<const M: i64>(pub i64);

impl<const M: i64> Mod<M> {
    pub fn mod_inv(self) -> Self {
        let (mut a, mut b, mut x0) = (self.0, M, 0);
        let mut result = 1;
        if b == 1 {
            return Mod(1);
        }
        while a > 1 {
            result -= a.div_euclid(b) * x0;
            a = a.rem_euclid(b);
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut x0, &mut result);
        }
        if result < 0 {
            result += M;
        }
        Mod(result)
    }
}

impl<const M: i64> Add for Mod<M> {
    type Output = Mod<M>;

    #[inline]
    fn add(self, other: Mod<M>) -> Self::Output {
        Mod((self.0 + other.0).rem_euclid(M))
    }
}

impl<const M: i64> AddAssign for Mod<M> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const M: i64> Sub for Mod<M> {
    type Output = Mod<M>;

    #[inline]
    fn sub(self, other: Mod<M>) -> Self::Output {
        Mod((self.0 - other.0).rem_euclid(M))
    }
}

impl<const M: i64> Mul for Mod<M> {
    type Output = Mod<M>;

    #[inline]
    fn mul(mut self, mut other: Mod<M>) -> Self::Output {
        let mut result = 0;
        while other.0 > 0 {
            if other.0.rem_euclid(2) == 1 {
                result = (result + self.0).rem_euclid(M);
            }
            self.0 = (2 * self.0).rem_euclid(M);
            other.0 = other.0.div_euclid(2);
        }
        Mod(result)
    }
}

impl<const M: i64> Mul<i64> for Mod<M> {
    type Output = Self;

    #[inline]
    fn mul(self, n: i64) -> Self {
        Mod((self.0 * n).rem_euclid(M))
    }
}

impl<const M: i64> Pow<i64> for Mod<M> {
    type Output = Mod<M>;

    #[inline]
    fn pow(self, rhs: i64) -> Self::Output {
        Mod(mod_exp(self.0, rhs, M))
    }
}

impl<const M: i64> Neg for Mod<M> {
    type Output = Mod<M>;

    #[inline]
    fn neg(self) -> Self::Output {
        Mod((-self.0).rem_euclid(M))
    }
}

impl<const M: i64> Div for Mod<M> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Mod((self.0 / rhs.0).rem_euclid(M))
    }
}

impl<const M: i64> Rem for Mod<M> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Mod((self.0 % rhs.0).rem_euclid(M))
    }
}

impl<const M: i64> Zero for Mod<M> {
    #[inline]
    fn zero() -> Self {
        Mod(0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl<const M: i64> One for Mod<M> {
    #[inline]
    fn one() -> Self {
        Mod(1_i64.rem_euclid(M))
    }
}

impl<const M: i64> Num for Mod<M> {
    type FromStrRadixErr = core::num::ParseIntError;

    #[inline]
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Mod(i64::from_str_radix(s, radix)?.rem_euclid(M)))
    }
}

impl<const M: i64> AsPrimitive<usize> for Mod<M> {
    #[inline]
    fn as_(self) -> usize {
        self.0.as_()
    }
}

/// Area of polygon given a list of points.
pub fn shoelace<T: Copy + Num + Signed + Sum>(pts: &[C<T>]) -> T {
    pts.windows(2).map(|w| (w[0].1 + w[1].1) * (w[1].0 - w[0].0)).sum::<T>().abs()
        / (T::one() + T::one())
}

/// Pick's theorem: A = i + b/2 - 1 where A is area of polygon, i is interior points, b is boundary points.
pub fn picks_interior<T: Num>(area: T, boundary: T) -> T {
    area + T::one() - boundary / (T::one() + T::one())
}

#[derive(Clone, Debug)]
pub struct Grid<T, I = usize> {
    pub rows: I,
    pub cols: I,
    pub elems: Vec<T>,
}

impl<T, I> IntoIterator for Grid<T, I> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}

impl<'a, T, I> IntoIterator for &'a Grid<T, I> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter()
    }
}

fn grid_from_iter<T, I, E, F, Idx>(iter: I, sep: T, f: F) -> Grid<E, Idx>
where
    T: Eq,
    I: IntoIterator<Item = T>,
    F: Fn(T) -> E,
    Idx: 'static + AsPrimitive<usize> + AddAssign + Copy + Div<Output = Idx> + Eq + One + Zero,
    usize: AsPrimitive<Idx>,
{
    let mut elems = Vec::new();
    let mut c = Idx::zero();
    let mut cols = Idx::zero();
    for b in iter {
        if b == sep {
            if cols.is_zero() {
                cols = c;
            }
            assert!(c.is_zero() || c == cols);
            c.set_zero()
        } else {
            elems.push(f(b));
            c += Idx::one();
        }
    }
    assert!(c.is_zero() || c == cols);
    Grid { rows: (elems.len() / cols.as_()).as_(), cols, elems }
}

impl<Idx> FromIterator<u8> for Grid<u8, Idx>
where
    Idx: 'static + AsPrimitive<usize> + AddAssign + Copy + Div<Output = Idx> + Eq + One + Zero,
    usize: AsPrimitive<Idx>,
{
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        grid_from_iter(iter, b'\n', identity)
    }
}

impl<Idx> FromIterator<char> for Grid<char, Idx>
where
    Idx: 'static + AsPrimitive<usize> + AddAssign + Copy + Div<Output = Idx> + Eq + One + Zero,
    usize: AsPrimitive<Idx>,
{
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        grid_from_iter(iter, '\n', identity)
    }
}

impl<T: Clone + Default, I: AsPrimitive<usize>> Grid<T, I> {
    #[inline]
    pub fn new(rows: I, cols: I) -> Grid<T, I> {
        Grid::new_with(rows, cols, Default::default())
    }
}

impl<T: Clone, I: AsPrimitive<usize>> Grid<T, I> {
    #[inline]
    pub fn new_with(rows: I, cols: I, elem: T) -> Self {
        Grid { rows, cols, elems: vec![elem; rows.as_() * cols.as_()] }
    }
}

impl<T: FromPrimitive, Idx> Grid<T, Idx>
where
    Idx: 'static + AsPrimitive<usize> + AddAssign + Copy + Div<Output = Idx> + Eq + One + Zero,
    usize: AsPrimitive<Idx>,
{
    pub fn ints<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        grid_from_iter(iter, b'\n', |b| T::from_u8(b - b'0').unwrap())
    }
}

impl<T, I> Grid<T, I>
where
    I: AsPrimitive<usize> + Copy + Ord + Zero,
    std::ops::Range<I>: Iterator<Item = I>,
{
    pub fn transform<S, F: FnMut(T) -> S>(self, f: F) -> Grid<S, I> {
        Grid { rows: self.rows, cols: self.cols, elems: self.elems.into_iter().map(f).collect() }
    }

    pub fn itransform<S, F: FnMut((C<I>, T)) -> S>(self, f: F) -> Grid<S, I> {
        Grid { rows: self.rows, cols: self.cols, elems: self.into_idx_iter().map(f).collect() }
    }

    #[inline]
    pub fn same_size<E: Clone + Default>(&self) -> Grid<E, I> {
        self.same_size_with(Default::default())
    }

    #[inline]
    pub fn same_size_with<E: Clone>(&self, elem: E) -> Grid<E, I> {
        Grid { rows: self.rows, cols: self.cols, elems: vec![elem; self.elems.len()] }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elems.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.elems.iter_mut()
    }

    #[inline]
    pub fn idxs(&self) -> impl Iterator<Item = C<I>> + use<T, I> {
        let rows = self.rows;
        let cols = self.cols;
        (I::zero()..rows).flat_map(move |r| (I::zero()..cols).map(move |c| C(r, c)))
    }

    pub fn idx_iter(&self) -> impl Iterator<Item = (C<I>, &T)> {
        self.idxs().zip(self.elems.iter())
    }

    pub fn idx_iter_mut(&mut self) -> impl Iterator<Item = (C<I>, &mut T)> {
        self.idxs().zip(self.elems.iter_mut())
    }

    pub fn into_idx_iter(self) -> impl Iterator<Item = (C<I>, T)> {
        self.idxs().zip(self)
    }

    pub fn position(&self, f: impl Fn(&T) -> bool) -> Option<C<I>> {
        self.idx_iter().find(move |(_, v)| f(v)).map(|(i, _)| i)
    }

    pub fn positions<'a>(&'a self, f: impl Fn(&T) -> bool + 'a) -> impl Iterator<Item = C<I>> + 'a {
        self.idx_iter().filter(move |(_, v)| f(v)).map(|(i, _)| i)
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.rows.as_()).map(|r| &self.elems[r * self.cols.as_()..(r + 1) * self.cols.as_()])
    }

    pub fn row(&self, r: I) -> impl Iterator<Item = &T> {
        let r = r.as_();
        self.elems[r * self.cols.as_()..(r + 1) * self.cols.as_()].iter()
    }

    pub fn col(&self, c: I) -> impl Iterator<Item = &T> {
        let c = c.as_();
        self.elems.iter().skip(c).step_by(self.cols.as_())
    }

    pub fn in_bounds(&self, C(r, c): C<I>) -> bool {
        r >= I::zero() && r < self.rows && c >= I::zero() && c < self.cols
    }

    pub fn get(&self, i: C<I>) -> Option<&T> {
        self.in_bounds(i).then(|| &self[i])
    }

    pub fn get_mut(&mut self, i: C<I>) -> Option<&mut T> {
        self.in_bounds(i).then(|| &mut self[i])
    }
}

impl<T, I, A, B> Index<C<A, B>> for Grid<T, I>
where
    I: AsPrimitive<usize>,
    A: AsPrimitive<usize>,
    B: AsPrimitive<usize>,
{
    type Output = T;

    fn index(&self, C(r, c): C<A, B>) -> &Self::Output {
        &self.elems[r.as_() * self.cols.as_() + c.as_()]
    }
}

impl<T, I, A, B> IndexMut<C<A, B>> for Grid<T, I>
where
    I: AsPrimitive<usize>,
    A: AsPrimitive<usize>,
    B: AsPrimitive<usize>,
{
    fn index_mut(&mut self, C(r, c): C<A, B>) -> &mut T {
        &mut self.elems[r.as_() * self.cols.as_() + c.as_()]
    }
}

impl<T, I: AsPrimitive<usize>> Index<(I, I)> for Grid<T, I> {
    type Output = T;

    fn index(&self, (r, c): (I, I)) -> &Self::Output {
        &self.elems[r.as_() * self.cols.as_() + c.as_()]
    }
}

impl<T, I: AsPrimitive<usize>> IndexMut<(I, I)> for Grid<T, I> {
    fn index_mut(&mut self, (r, c): (I, I)) -> &mut T {
        &mut self.elems[r.as_() * self.cols.as_() + c.as_()]
    }
}

impl<T, I: AsPrimitive<usize>> Index<I> for Grid<T, I> {
    type Output = T;

    fn index(&self, idx: I) -> &Self::Output {
        &self.elems[idx.as_()]
    }
}

impl<T, I: AsPrimitive<usize>> IndexMut<I> for Grid<T, I> {
    fn index_mut(&mut self, idx: I) -> &mut T {
        &mut self.elems[idx.as_()]
    }
}

#[derive(Default)]
pub struct DefaultVec<T>(Vec<T>);

impl<T: Default> DefaultVec<T> {
    #[inline]
    fn ensure_size(&mut self, idx: usize) {
        if idx >= self.0.len() {
            self.0.resize_with(idx + 1, T::default);
        }
    }

    #[inline]
    pub fn get(&mut self, idx: usize) -> &T {
        self.ensure_size(idx);
        self.0.index(idx)
    }

    #[inline]
    pub fn get_mut(&mut self, idx: usize) -> &mut T {
        self.ensure_size(idx);
        self.0.index_mut(idx)
    }
}

impl<T> Deref for DefaultVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
