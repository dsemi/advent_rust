use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;
use std::collections::VecDeque;

const CHUNK_SIZE: usize = 8000;

fn find_indexes(seed: &str, num: usize) -> impl Iterator<Item = usize> {
    let mut hasher = Md5::new();
    hasher.input_str(seed);
    (0..)
        .step_by(CHUNK_SIZE)
        .flat_map(move |n| {
            (n..n + CHUNK_SIZE)
                .into_par_iter()
                .map(|i| {
                    let mut h = hasher.clone();
                    h.input_str(&i.to_string());
                    let mut out = h.result_str();
                    for _ in 0..num {
                        h.reset();
                        h.input_str(&out);
                        out = h.result_str();
                    }
                    (i, out)
                })
                .collect::<Vec<_>>()
        })
        .scan(
            VecDeque::new(),
            |pot: &mut VecDeque<(usize, u8)>, (i, hashed)| {
                while !pot.is_empty() {
                    if i - pot[0].0 > 1000 {
                        pot.pop_front();
                    } else {
                        break;
                    }
                }
                match hashed
                    .as_bytes()
                    .windows(3)
                    .filter_map(|w| (w[0] == w[1] && w[0] == w[2]).then(|| w[0]))
                    .next()
                {
                    None => Some(None),
                    Some(ch) => {
                        let (done, rest): (VecDeque<(usize, u8)>, VecDeque<(usize, u8)>) =
                            pot.iter().partition(|x| {
                                hashed
                                    .as_bytes()
                                    .windows(5)
                                    .any(|w| w.iter().all(|c| c == &x.1))
                            });
                        *pot = rest;
                        pot.push_back((i, ch));
                        Some(Some(done.into_iter().map(|x| x.0).collect::<Vec<_>>()))
                    }
                }
            },
        )
        .flatten()
        .flatten()
}

pub fn part1(input: &str) -> Option<usize> {
    find_indexes(input, 0).nth(63)
}

pub fn part2(input: &str) -> Option<usize> {
    find_indexes(input, 2016).nth(63)
}
