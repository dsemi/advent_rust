use crate::utils::parsers::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::iter::zip;
use std::sync::Mutex;
use wide::*;

const MASK: i32 = (1 << 24) - 1;

macro_rules! step {
    ($secret:ident, $mask:ident) => {{
        $secret ^= $secret << 6 & $mask;
        $secret ^= $secret >> 5;
        $secret ^= $secret << 11 & $mask;
    }};
}

pub fn part1(input: &str) -> i64 {
    let mask = i32x8::splat(MASK);
    lines(i32)
        .read(input)
        .par_chunks(8)
        .map(|chunk| {
            let mut secret = i32x8::new(chunk.try_into().unwrap());
            (0..2000).for_each(|_| step!(secret, mask));
            secret.reduce_add() as i64
        })
        .sum()
}

// Range of -9..=9 possible for each, 19.pow(4) possible values
fn idx(a: i32, b: i32, c: i32, d: i32) -> usize {
    (6859 * (a + 9) + 361 * (b + 9) + 19 * (c + 9) + d + 9) as usize
}

pub fn part2(input: &str) -> i32 {
    let mask = i32x8::splat(MASK);
    let result = Mutex::new(vec![0; 19_usize.pow(4)]);
    let ns = lines(i32).read(input);
    ns.par_chunks(192).for_each(|chunk| {
        let mut dp = vec![0; 19_usize.pow(4)];
        let mut seen = vec![0_u16; 19_usize.pow(4)];
        let mut rems = [[0; 2001]; 8];
        for (inner, id) in chunk.chunks(8).zip((1..).step_by(8)) {
            let mut secret = i32x8::new(inner.try_into().unwrap());
            for i in 0..2000 {
                zip(&mut rems, secret.as_array_ref()).for_each(|(r, s)| r[i] = s % 10);
                step!(secret, mask);
            }
            zip(&mut rems, secret.as_array_ref()).for_each(|(r, s)| r[2000] = s % 10);

            for (offset, rem) in rems.iter().enumerate() {
                let id = id + offset as u16;
                for (e, d, c, b, a) in rem.iter().tuple_windows() {
                    let idx = idx(d - e, c - d, b - c, a - b);
                    if seen[idx] != id {
                        dp[idx] += a;
                        seen[idx] = id;
                    }
                }
            }
        }
        let mut global = result.lock().unwrap();
        global.iter_mut().zip(dp).for_each(|(a, b)| *a += b);
    });
    result.into_inner().unwrap().into_iter().max().unwrap()
}
