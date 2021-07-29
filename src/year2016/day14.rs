use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;

const CHUNK_SIZE: usize = 8000;

fn idx(byte: u8) -> usize {
    match byte {
        b'0'..=b'9' => (byte - b'0') as usize,
        b'a'..=b'f' => (byte - b'a' + 10) as usize,
        _ => panic!("Unknown byte: {}", byte),
    }
}

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
        .scan(vec![Vec::new(); 16], |pot, (i, hashed)| {
            let bytes = hashed.as_bytes();
            let fives: Vec<usize> = bytes
                .windows(5)
                .filter_map(|w| {
                    (w[0] == w[1] && w[0] == w[2] && w[0] == w[3] && w[0] == w[4]).then(|| {
                        pot[idx(w[0])]
                            .drain(..)
                            .filter(|&v| i - v <= 1000)
                            .collect::<Vec<_>>()
                    })
                })
                .flatten()
                .collect();
            for w in bytes.windows(3) {
                if w[0] == w[1] && w[0] == w[2] {
                    pot[idx(w[0])].push(i);
                    break;
                }
            }
            Some(fives)
        })
        .flatten()
}

pub fn part1(input: &str) -> Option<usize> {
    find_indexes(input, 0).nth(63)
}

pub fn part2(input: &str) -> Option<usize> {
    find_indexes(input, 2016).nth(63)
}
