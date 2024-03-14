use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use nalgebra::Vector2;
use nalgebra::Vector3;
use num::{Num, PrimInt, Signed};
use num_traits::ops::saturating::SaturatingAdd;
use num_traits::{AsPrimitive, Bounded, FromPrimitive, One, Pow, Zero};
use smallvec::SmallVec;
use std::cmp::Ordering::*;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use std::convert::{identity, From};
use std::hash::Hash;
use std::iter::Sum;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Shr,
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
    let mut visited = AHashSet::new();
    let mut frontier = VecDeque::new();
    for start in starts {
        visited.insert(h(&start));
        frontier.push_back((0, start));
    }
    Bfs {
        frontier,
        visited,
        hash: h,
        neighbs,
    }
}

pub struct Bfs<T, F, G, H> {
    pub frontier: VecDeque<(usize, T)>,
    pub visited: AHashSet<H>,
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
struct AStarNode<T> {
    weight: usize,
    elem: T,
}

impl<T: Eq> PartialOrd for AStarNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for AStarNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

pub fn a_star<T, F, I, I2, F2, F3>(
    neighbors: F,
    heur: F2,
    goal: F3,
    start: T,
) -> Option<Vec<(usize, T)>>
where
    T: Clone,
    T: Eq,
    T: Hash,
    F: Fn(&T) -> I,
    I: IntoIterator<Item = (usize, T), IntoIter = I2>,
    I2: Iterator<Item = (usize, T)>,
    F2: Fn(&T) -> usize,
    F3: Fn(&T) -> bool,
{
    let mut visited: AHashSet<T> = vec![start.clone()].into_iter().collect();
    let mut queue: BinaryHeap<AStarNode<T>> = BinaryHeap::new();
    queue.push(AStarNode {
        weight: 0,
        elem: start.clone(),
    });
    let mut came_from: AHashMap<T, T> = AHashMap::new();
    let mut g_score: AHashMap<T, usize> = vec![(start.clone(), 0)].into_iter().collect();
    let mut f_score: AHashMap<T, usize> = vec![(start.clone(), heur(&start))].into_iter().collect();
    while let Some(AStarNode { elem: st, .. }) = queue.pop() {
        if goal(&st) {
            let mut result = vec![(g_score[&st], st.clone())];
            let mut curr = &st;
            while let Some(v) = came_from.get(curr) {
                result.push((g_score[v], v.clone()));
                curr = v;
            }
            result.reverse();
            return Some(result);
        }
        visited.remove(&st);
        for (dist, st2) in neighbors(&st) {
            let tent_g_score = g_score
                .get(&st)
                .unwrap_or(&usize::MAX)
                .checked_add(dist)
                .unwrap();
            if tent_g_score < *g_score.get(&st2).unwrap_or(&usize::MAX) {
                came_from.insert(st2.clone(), st.clone());
                g_score.insert(st2.clone(), tent_g_score);
                f_score.insert(st2.clone(), tent_g_score + heur(&st2));
                if visited.insert(st2.clone()) {
                    queue.push(AStarNode {
                        weight: f_score[&st2],
                        elem: st2,
                    });
                }
            }
        }
    }
    None
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
    dists: AHashMap<T, usize>,
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
    queue.push(Reverse(State {
        dist: 0,
        elem: start,
    }));
    Dijkstra {
        queue,
        dists: AHashMap::new(),
        neighbors,
    }
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

macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $g:tt) => {
        impl<'a, $g: Num + Copy> $imp<$t> for &'a $t {
            type Output = <$t as $imp<$t>>::Output;

            #[inline]
            fn $method(self, other: $t) -> <$t as $imp<$t>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<$g: Num + Copy> $imp<&$t> for $t {
            type Output = <$t as $imp<$t>>::Output;

            #[inline]
            fn $method(self, other: &$t) -> <$t as $imp<$t>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<$g: Num + Copy> $imp<&$t> for &$t {
            type Output = <$t as $imp<$t>>::Output;

            #[inline]
            fn $method(self, other: &$t) -> <$t as $imp<$t>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

pub trait AbsDiff<T> {
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct C<T>(pub T, pub T);

mod cparse {
    use winnow::error::ParserError;
    use winnow::stream::{AsChar, Compare, Stream, StreamIsPartial};
    use winnow::Parser;

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

impl<T> C<T> {
    pub fn as_<U: Copy + 'static>(self) -> C<U>
    where
        T: AsPrimitive<U>,
    {
        C(self.0.as_(), self.1.as_())
    }

    pub fn vec(self) -> Vector2<T> {
        Vector2::new(self.0, self.1)
    }
}

impl<T: Num + Copy> C<T> {
    pub fn sum(&self) -> T {
        self.0 + self.1
    }
}

impl<T: Num + Copy> C<T> {
    pub fn product(&self) -> T {
        self.0 * self.1
    }
}

impl<T: Num + Signed> C<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs(), self.1.abs())
    }
}

impl<T: Num + AbsDiff<T> + Copy> C<T> {
    pub fn dist(&self, other: &Self) -> T {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl<T: Signed + Copy> C<T> {
    pub fn signum(&self) -> Self {
        Self(self.0.signum(), self.1.signum())
    }
}

impl<T: Ord + Copy> C<T> {
    pub fn smol(self, o: Self) -> Self {
        Self(min(self.0, o.0), min(self.1, o.1))
    }

    pub fn swol(self, o: Self) -> Self {
        Self(max(self.0, o.0), max(self.1, o.1))
    }
}

impl<T: Num> Add for C<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
forward_ref_binop! { impl Add, add for C<T>, T }

impl<T: Num + Copy> AddAssign for C<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Num> Sub for C<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
forward_ref_binop! { impl Sub, sub for C<T>, T }

impl<T: Num + Copy> SubAssign for C<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T: Num + Copy> Mul for C<T> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(
            self.0 * other.0 - self.1 * other.1,
            self.0 * other.1 + self.1 * other.0,
        )
    }
}
forward_ref_binop! { impl Mul, mul for C<T>, T }

impl<T: Num + Copy> MulAssign for C<T> {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<T: Num + Copy> Mul<T> for C<T> {
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

impl<T: Neg<Output = T>> Neg for C<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl<T: Num + Copy> C<T> {
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

impl<T: Num + Copy> Sum for C<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(T::zero(), T::zero()), |a, b| a + b)
    }
}

impl<T> From<(T, T)> for C<T> {
    fn from((a, b): (T, T)) -> Self {
        C(a, b)
    }
}

impl<T> From<C<T>> for (T, T) {
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
pub struct C3<T>(pub T, pub T, pub T);

mod c3parse {
    use winnow::error::ParserError;
    use winnow::stream::{AsChar, Compare, Stream, StreamIsPartial};
    use winnow::Parser;

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
}

impl<T: Num + Copy> C3<T> {
    pub fn product(&self) -> T {
        self.0 * self.1 * self.2
    }
}

impl<T: Num + Signed> C3<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs(), self.1.abs(), self.2.abs())
    }
}

impl<T: Num + AbsDiff<T> + Copy> C3<T> {
    pub fn dist(&self, other: &Self) -> T {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2)
    }
}

impl<T: Signed + Copy> C3<T> {
    pub fn signum(&self) -> Self {
        Self(self.0.signum(), self.1.signum(), self.2.signum())
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

impl<T: Num> Add for C3<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
forward_ref_binop! { impl Add, add for C3<T>, T }

impl<T: Num + Copy> AddAssign for C3<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Num> Sub for C3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
forward_ref_binop! { impl Sub, sub for C3<T>, T }

impl<T: Num + Copy> SubAssign for C3<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

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
                while self.n % f == 0 {
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
    PrimeFactors {
        n,
        fs: std::iter::once(2).chain((3..=sqrt).step_by(2)),
    }
}

pub struct Cache<'a, A, R> {
    cache: AHashMap<A, R>,
    func: &'a dyn Fn(&mut Self, A) -> R,
}

impl<'a, A, R> Cache<'a, A, R>
where
    A: Copy + Eq + Hash,
    R: Copy,
{
    pub fn from(func: &'a dyn Fn(&mut Self, A) -> R) -> Self {
        Self {
            cache: AHashMap::new(),
            func,
        }
    }

    pub fn get(&mut self, arg: A) -> R {
        if let Some(v) = self.cache.get(&arg) {
            return *v;
        }
        let v = (self.func)(self, arg);
        self.cache.insert(arg, v);
        v
    }
}

pub struct Partitions {
    buf: Vec<i32>,
    stack: Vec<(usize, i32, i32)>,
    in_progress: bool,
}

impl Partitions {
    pub fn new(len: usize, tot: i32) -> Self {
        Self {
            buf: vec![0; len + 1],
            stack: vec![(len, 0, tot)],
            in_progress: true,
        }
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

    pub fn len(&self) -> T {
        self.hi - self.lo
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

pub struct UniqueIdx<T> {
    m: AHashMap<T, usize>,
}

impl<T: Eq + Hash> UniqueIdx<T> {
    pub fn new() -> Self {
        UniqueIdx { m: AHashMap::new() }
    }

    pub fn idx(&mut self, k: T) -> usize {
        let c = self.m.len();
        *self.m.entry(k).or_insert(c)
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

pub fn binary_search_by<F>(lo: i64, hi: i64, mut f: F) -> Result<i64, i64>
where
    F: FnMut(&i64) -> Ordering,
{
    let mut size = hi - lo;
    let mut left = lo;
    let mut right = hi;
    while left < right {
        let mid = left + size / 2;
        match f(&mid) {
            Less => left = mid + 1,
            Greater => right = mid,
            Equal => return Ok(mid),
        }
        size = right - left;
    }
    Err(left)
}

pub fn partition_point<P>(lo: i64, hi: i64, mut pred: P) -> i64
where
    P: FnMut(&i64) -> bool,
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
    let mut g = AHashMap::new();
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
    (1..len)
        .map(move |k| g[&((1..len).collect(), k)] + adj[k][0])
        .reduce(f)
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

impl<T> UnionFind<T> {
    pub fn new() -> Self {
        UnionFind { nodes: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.nodes.push(UnionFindNode {
            val,
            parent: self.nodes.len(),
            rank: 0,
        })
    }

    pub fn find(&self, mut k: usize) -> usize {
        while k != self.nodes[k].parent {
            k = self.nodes[k].parent;
        }
        k
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

impl<T> FromIterator<T> for UnionFind<T> {
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
    fn counts(self) -> AHashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut m = AHashMap::new();
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

pub fn tails(s: &str) -> impl Iterator<Item = &'_ str> {
    std::iter::successors(Some(s), |s| (s.len() > 1).then(|| &s[1..]))
}

pub fn inits(s: &str) -> impl Iterator<Item = &'_ str> {
    std::iter::successors(Some(s), |s| (s.len() > 1).then(|| &s[..s.len() - 1]))
}

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

#[derive(Clone, Copy, Debug)]
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

    fn add(self, other: Mod<M>) -> Self::Output {
        Mod((self.0 + other.0).rem_euclid(M))
    }
}

impl<const M: i64> Mul for Mod<M> {
    type Output = Mod<M>;

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

impl<const M: i64> Pow<i64> for Mod<M> {
    type Output = Mod<M>;

    fn pow(self, rhs: i64) -> Self::Output {
        Mod(mod_exp(self.0, rhs, M))
    }
}

impl<const M: i64> Neg for Mod<M> {
    type Output = Mod<M>;

    fn neg(self) -> Self::Output {
        Mod((-self.0).rem_euclid(M))
    }
}

/// Area of polygon given a list of points.
pub fn shoelace<T: Copy + Num + Signed + Sum>(pts: &[C<T>]) -> T {
    pts.windows(2)
        .map(|w| (w[0].1 + w[1].1) * (w[1].0 - w[0].0))
        .sum::<T>()
        .abs()
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
    Grid {
        rows: (elems.len() / cols.as_()).as_(),
        cols,
        elems,
    }
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
        Grid {
            rows,
            cols,
            elems: vec![elem; rows.as_() * cols.as_()],
        }
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
        Grid {
            rows: self.rows,
            cols: self.cols,
            elems: self.elems.into_iter().map(f).collect(),
        }
    }

    pub fn itransform<S, F: FnMut((C<I>, T)) -> S>(self, f: F) -> Grid<S, I> {
        Grid {
            rows: self.rows,
            cols: self.cols,
            elems: self.into_idx_iter().map(f).collect(),
        }
    }

    #[inline]
    pub fn same_size<E: Clone + Default>(&self) -> Grid<E, I> {
        self.same_size_with(Default::default())
    }

    #[inline]
    pub fn same_size_with<E: Clone>(&self, elem: E) -> Grid<E, I> {
        Grid {
            rows: self.rows,
            cols: self.cols,
            elems: vec![elem; self.elems.len()],
        }
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
    pub fn idxs(&self) -> impl Iterator<Item = C<I>> {
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

impl<T, I: AsPrimitive<usize>> Index<C<I>> for Grid<T, I> {
    type Output = T;

    fn index(&self, C(r, c): C<I>) -> &Self::Output {
        &self.elems[r.as_() * self.cols.as_() + c.as_()]
    }
}

impl<T, I: AsPrimitive<usize>> IndexMut<C<I>> for Grid<T, I> {
    fn index_mut(&mut self, C(r, c): C<I>) -> &mut T {
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
