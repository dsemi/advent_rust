use ahash::AHashSet;
use itertools::Itertools;

fn reverse<T>(v: &mut [T], mut lo: usize, mut hi: usize) {
    let len = v.len();
    while lo < hi {
        v.swap(lo % len, hi % len);
        lo += 1;
        hi -= 1;
    }
}

fn hash(n: usize, lens: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = (0..256).collect();
    let mut pos = 0;
    let mut skip_size = 0;
    for _ in 0..n {
        for l in &lens {
            reverse(&mut result, pos, pos + l - 1);
            pos += l + skip_size;
            skip_size += 1;
        }
    }
    result
}

fn knot_hash(input: &str) -> Vec<usize> {
    let res = hash(
        64,
        input
            .chars()
            .map(|x| x as u8 as usize)
            .chain(vec![17, 31, 73, 47, 23])
            .collect(),
    );
    res.chunks(res.len() / 16)
        .map(|x| x.iter().fold(0, |a, b| a ^ *b))
        .collect()
}

fn hashes(key: &str) -> impl Iterator<Item = Vec<usize>> + '_ {
    (0..128).map(move |i| knot_hash(&format!("{key}-{i}")))
}

pub fn part1(input: &str) -> u32 {
    hashes(input)
        .map(|h| h.into_iter().fold(0, |a, b| a + b.count_ones()))
        .sum()
}

fn grid<I>(bss: I) -> AHashSet<(i32, i32)>
where
    I: Iterator<Item = Vec<usize>>,
{
    bss.enumerate()
        .flat_map(|(r, bs)| {
            bs.into_iter().enumerate().flat_map(move |(c, w)| {
                (0..8).filter_map(move |i| {
                    (w & (1 << i) != 0).then(|| (r as i32, (c * 8 + 7 - i) as i32))
                })
            })
        })
        .collect()
}

fn adjacents(c: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter_map(move |d| {
            let c2 = (c.0 + d.0, c.1 + d.1);
            (c2.0 >= 0 && c2.0 < 128 && c2.1 >= 0 && c2.1 < 128).then(|| c2)
        })
}

fn region_containing(arr: &AHashSet<(i32, i32)>, c: (i32, i32)) -> AHashSet<(i32, i32)> {
    let mut xs = vec![c];
    let mut result = AHashSet::new();
    while let Some(x) = xs.pop() {
        if !arr.contains(&x) || !result.insert(x) {
            continue;
        }
        xs.extend(adjacents(x));
    }
    result
}

pub fn part2(input: &str) -> usize {
    let arr = grid(hashes(input));
    let mut s = AHashSet::new();
    (0..128)
        .cartesian_product(0..128)
        .filter_map(|x| {
            (arr.contains(&x) && !s.contains(&x)).then(|| {
                s.extend(region_containing(&arr, x));
            })
        })
        .count()
}
