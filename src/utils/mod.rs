use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use num::{Num, PrimInt, Signed};
use num_traits::ops::saturating::SaturatingAdd;
use num_traits::{One, Zero};
use std::cmp::Ordering::*;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use std::hash::Hash;
use std::iter::Sum;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Shr, Sub,
    SubAssign,
};
use streaming_iterator::StreamingIterator;

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
    T: Ord,
    F: Fn(&T) -> I,
    I: IntoIterator<Item = (usize, T), IntoIter = I2>,
    I2: Iterator<Item = (usize, T)>,
    F2: Fn(&T) -> usize,
    F3: Fn(&T) -> bool,
{
    let mut visited: AHashSet<T> = vec![start.clone()].into_iter().collect();
    let mut queue: BinaryHeap<(Reverse<usize>, T)> = BinaryHeap::new();
    queue.push((Reverse(0), start.clone()));
    let mut came_from: AHashMap<T, T> = AHashMap::new();
    let mut g_score: AHashMap<T, usize> = vec![(start.clone(), 0)].into_iter().collect();
    let mut f_score: AHashMap<T, usize> = vec![(start.clone(), heur(&start))].into_iter().collect();
    while let Some((_, st)) = queue.pop() {
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
                    queue.push((Reverse(f_score[&st2]), st2));
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
    (impl $imp:ident, $method:ident for $t:ty, $u:ty, $g:tt) => {
        impl<'a, $g: Num + Copy> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<$g: Num + Copy> $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<$g: Num + Copy> $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct C<T>(pub T, pub T);

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

impl<T: Num + Signed + Copy> C<T> {
    pub fn dist(&self, other: &Self) -> T {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
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
forward_ref_binop! { impl Add, add for C<T>, C<T>, T }

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
forward_ref_binop! { impl Sub, sub for C<T>, C<T>, T }

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
forward_ref_binop! { impl Mul, mul for C<T>, C<T>, T }

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

macro_rules! impl_idx {
    ($($it:ty),*) => ($(
        impl<T> Index<C<$it>> for Vec<Vec<T>> {
            type Output = T;

            fn index(&self, C(r, c): C<$it>) -> &Self::Output {
                &self[r as usize][c as usize]
            }
        }

        impl<T> Index<C<$it>> for [Vec<T>] {
            type Output = T;

            fn index(&self, C(r, c): C<$it>) -> &Self::Output {
                &self[r as usize][c as usize]
            }
        }

        impl<T> IndexMut<C<$it>> for Vec<Vec<T>> {
            fn index_mut(&mut self, C(r, c): C<$it>) -> &mut T {
                &mut self[r as usize][c as usize]
            }
        }

        impl<T> IndexMut<C<$it>> for [Vec<T>] {
            fn index_mut(&mut self, C(r, c): C<$it>) -> &mut T {
                &mut self[r as usize][c as usize]
            }
        }
    )*)
}

impl_idx!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

pub trait GridIdx<I, T> {
    fn get_cell(&self, idx: C<I>) -> Option<&T>;
}

macro_rules! impl_get {
    ($($it:ty),*) => ($(
        impl<T> GridIdx<$it, T> for Vec<Vec<T>> {
            fn get_cell(&self, C(r, c): C<$it>) -> Option<&T> {
                self.get(r as usize)?.get(c as usize)
            }
        }

        impl<T> GridIdx<$it, T> for [Vec<T>] {
            fn get_cell(&self, C(r, c): C<$it>) -> Option<&T> {
                self.get(r as usize)?.get(c as usize)
            }
        }
    )*)
}

impl_get!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct C3<T>(pub T, pub T, pub T);

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

impl<T: Num + Signed + Copy> C3<T> {
    pub fn dist(&self, other: &Self) -> T {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
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
forward_ref_binop! { impl Add, add for C3<T>, C3<T>, T }

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
forward_ref_binop! { impl Sub, sub for C3<T>, C3<T>, T }

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
pub struct Interval {
    pub lo: i64,
    pub hi: i64,
}

impl Interval {
    pub fn new(lo: i64, hi: i64) -> Self {
        Self { lo, hi }
    }

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

    pub fn len(&self) -> i64 {
        self.hi - self.lo
    }

    pub fn contains(&self, v: i64) -> bool {
        self.lo <= v && v < self.hi
    }

    pub fn valid(&self) -> bool {
        self.lo < self.hi
    }
}

impl Add<i64> for Interval {
    type Output = Interval;

    fn add(self, diff: i64) -> Self::Output {
        Interval::new(self.lo + diff, self.hi + diff)
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

struct UnionFindNode<T> {
    val: T,
    parent: usize,
    rank: usize,
}

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
