use md5::{Digest, Md5};
use rayon::prelude::*;

const CHUNK_SIZE: usize = 64_000;

fn find_num(f: fn(&[u8]) -> bool, key: &str) -> Option<usize> {
    let keyb = key.as_bytes();
    (0..).step_by(CHUNK_SIZE).find_map(|n| {
        (n..n + CHUNK_SIZE).into_par_iter().find_first(|i: &usize| {
            let mut hasher = Md5::new();
            hasher.update(keyb);
            hasher.update(i.to_string());
            f(hasher.finalize().as_slice())
        })
    })
}

pub fn part1(input: &str) -> Option<usize> {
    find_num(|v| v[0] == 0 && v[1] == 0 && v[2] <= 0xF, input)
}

pub fn part2(input: &str) -> Option<usize> {
    find_num(|v| v[0] == 0 && v[1] == 0 && v[2] == 0, input)
}
