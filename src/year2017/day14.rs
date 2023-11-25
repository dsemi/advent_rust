use num::Integer;
use rayon::prelude::*;

fn reverse<T>(v: &mut [T], mut lo: usize, mut hi: usize) {
    let len = v.len();
    while lo < hi {
        v.swap(lo % len, hi % len);
        lo += 1;
        hi -= 1;
    }
}

fn hash(n: usize, lens: Vec<usize>) -> Vec<u8> {
    let mut result: Vec<u8> = (0..=255).collect();
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

fn knot_hash(key: &str, i: usize) -> u128 {
    let res = hash(
        64,
        format!("{key}-{i}")
            .bytes()
            .map(|x| x as usize)
            .chain(vec![17, 31, 73, 47, 23])
            .collect(),
    );
    res.chunks(res.len() / 16)
        .map(|x| x.iter().fold(0, |a, &b| a ^ b))
        .fold(0, |acc, x| (acc << 8) | x as u128)
}

fn hashes(key: &str) -> impl ParallelIterator<Item = u128> + '_ {
    (0..128).into_par_iter().map(move |i| knot_hash(key, i))
}

pub fn part1(input: &str) -> u32 {
    hashes(input).map(|h| h.count_ones()).sum()
}

fn dfs(grid: &mut [bool], i: usize) -> bool {
    if !grid[i] {
        return false;
    }
    grid[i] = false;
    let (y, x) = i.div_rem(&128);
    if x > 0 && grid[i - 1] {
        dfs(grid, i - 1);
    }
    if x < 127 && grid[i + 1] {
        dfs(grid, i + 1);
    }
    if y > 0 && grid[i - 128] {
        dfs(grid, i - 128);
    }
    if y < 127 && grid[i + 128] {
        dfs(grid, i + 128);
    }
    true
}

pub fn part2(input: &str) -> usize {
    let mut arr = hashes(input)
        .flat_map_iter(|row| (0..128).rev().map(move |i| row & (1 << i) != 0))
        .collect::<Vec<_>>();
    (0..arr.len()).filter(|&i| dfs(&mut arr, i)).count()
}
