use ahash::AHashMap;
use ahash::AHashSet;
use itertools::Itertools;
use num::Num;
use num_traits::cast::FromPrimitive;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::iter::Sum;
use std::ops::{Add, AddAssign, BitAnd, Mul, MulAssign, Neg, Sub, SubAssign};

pub fn bfs<T, F, I, I2>(start: T, neighbs: F) -> Bfs<T, F, impl Fn(&T) -> T, T>
where
    T: Clone,
    T: Eq,
    T: Hash,
    F: Fn(&T) -> I,
    I: IntoIterator<Item = T, IntoIter = I2>,
    I2: Iterator<Item = T>,
{
    bfs_on(|x| x.clone(), start, neighbs)
}

pub fn bfs_on<T, F, G, H, I, I2>(h: G, start: T, neighbs: F) -> Bfs<T, F, G, H>
where
    H: Eq,
    H: Hash,
    G: Fn(&T) -> H,
    F: Fn(&T) -> I,
    I: IntoIterator<Item = T, IntoIter = I2>,
    I2: Iterator<Item = T>,
{
    let x = h(&start);
    Bfs {
        frontier: vec![(0, start)].into_iter().collect(),
        visited: vec![x].into_iter().collect(),
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
    F: Fn(&T) -> I,
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

pub fn a_star<T, F, I, I2, F2, F3, F4>(
    neighbors: F,
    dist: F2,
    heur: F3,
    goal: F4,
    start: T,
) -> Option<Vec<T>>
where
    T: Copy,
    T: Eq,
    T: Hash,
    T: Ord,
    F: Fn(&T) -> I,
    I: IntoIterator<Item = T, IntoIter = I2>,
    I2: Iterator<Item = T>,
    F2: Fn(&T, &T) -> usize,
    F3: Fn(&T) -> usize,
    F4: Fn(&T) -> bool,
{
    let mut visited: AHashSet<T> = vec![start].into_iter().collect();
    let mut queue: BinaryHeap<(Reverse<usize>, T)> = BinaryHeap::new();
    queue.push((Reverse(0), start));
    let mut came_from: AHashMap<T, T> = AHashMap::new();
    let mut g_score: AHashMap<T, usize> = vec![(start, 0)].into_iter().collect();
    let mut f_score: AHashMap<T, usize> = vec![(start, heur(&start))].into_iter().collect();
    while let Some((_, st)) = queue.pop() {
        if goal(&st) {
            let mut result = vec![st];
            let mut curr = st;
            while let Some(v) = came_from.get(&curr) {
                result.push(*v);
                curr = *v;
            }
            result.reverse();
            return Some(result);
        }
        visited.remove(&st);
        for st2 in neighbors(&st) {
            let tent_g_score = g_score
                .get(&st)
                .unwrap_or(&usize::MAX)
                .checked_add(dist(&st, &st2))
                .unwrap();
            if tent_g_score < *g_score.get(&st2).unwrap_or(&usize::MAX) {
                came_from.insert(st2, st);
                g_score.insert(st2, tent_g_score);
                f_score.insert(st2, tent_g_score + heur(&st2));
                if visited.insert(st2) {
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

impl <T: Eq> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl <T: Eq> PartialOrd for State<T> {
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
    I2: Iterator<Item = (usize, T)>
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

impl <T, F, I, I2> Iterator for Dijkstra<T, F>
where
    T: Clone,
    T: Eq,
    T: Hash,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = (usize, T), IntoIter = I2>,
    I2: Iterator<Item = (usize, T)>
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
                    self.queue.push(Reverse(State {
                        dist,
                        elem: st2,
                    }));
                }
            }
        }
        Some((state.dist, state.elem))
    }
}

pub fn unit_dir(c: char) -> Coord<i64> {
    match c {
        '<' => Coord::new(-1, 0),
        '>' => Coord::new(1, 0),
        'v' => Coord::new(0, -1),
        '^' => Coord::new(0, 1),
        _ => panic!("Unknown direction"),
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord<T>
where
    T: Copy,
    T: Num,
    T: BitAnd<Output = T>,
    T: FromPrimitive,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn scale(&self, n: T) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
        }
    }

    pub fn pow(self, n: T) -> Self {
        if n == FromPrimitive::from_u8(0).unwrap() {
            Self {
                x: FromPrimitive::from_u8(1).unwrap(),
                y: FromPrimitive::from_u8(0).unwrap(),
            }
        } else if n & FromPrimitive::from_u8(1).unwrap() != FromPrimitive::from_u8(0).unwrap() {
            self * self.pow(n - FromPrimitive::from_u8(1).unwrap())
        } else {
            (self * self).pow(n / FromPrimitive::from_u8(2).unwrap())
        }
    }
}

impl<T: Add<Output = T>> Add for Coord<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b, T> Add<&'b Coord<T>> for &'a Coord<T>
where
    T: Copy,
    T: Add<Output = T>,
{
    type Output = Coord<T>;

    fn add(self, other: &'b Coord<T>) -> Coord<T> {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Coord<T>
where
    T: Copy,
    T: Num,
{
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Num> Sub for Coord<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b, T> Sub<&'b Coord<T>> for &'a Coord<T>
where
    T: Copy,
    T: Sub<Output = T>,
{
    type Output = Coord<T>;

    fn sub(self, other: &'b Coord<T>) -> Coord<T> {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Coord<T>
where
    T: Copy,
    T: Num,
{
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T> Mul for Coord<T>
where
    T: Copy,
    T: Num,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }
}

impl<T> MulAssign for Coord<T>
where
    T: Copy,
    T: Num,
{
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<T> Sum for Coord<T>
where
    T: Copy,
    T: Num,
    T: BitAnd<Output = T>,
    T: FromPrimitive,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(
            Coord::new(
                FromPrimitive::from_u8(0).unwrap(),
                FromPrimitive::from_u8(0).unwrap(),
            ),
            |a, b| a + b,
        )
    }
}

pub fn adjacents(coord: Coord<i64>) -> impl Iterator<Item = Coord<i64>> {
    (-1..2)
        .cartesian_product(-1..2)
        .filter_map(move |(x, y)| (x != 0 || y != 0).then(|| Coord::new(coord.x + x, coord.y + y)))
}

pub fn dist(a: &Coord<i32>, b: &Coord<i32>) -> i32 {
    let c = a - b;
    c.x.abs() + c.y.abs()
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coord3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Coord3<T>
where
    T: Copy,
    T: Num,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn scale(&self, n: T) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }

    pub fn div(&self, n: T) -> Self {
        Self {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

impl<T: Add<Output = T>> Add for Coord3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign for Coord3<T>
where
    T: Copy,
    T: Num,
{
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Num> Sub for Coord3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Coord3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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

pub fn prime_factors(n: u64) -> PrimeFactors<impl Iterator<Item = u64>> {
    let sqrt = (n as f64).sqrt() as u64;
    PrimeFactors {
        n,
        fs: std::iter::once(2).chain((3..=sqrt).step_by(2)),
    }
}
