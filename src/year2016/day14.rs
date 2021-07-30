use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;

const CHUNK_SIZE: usize = 8000;

struct HashRes {
    md: [u8; 16],
}

impl HashRes {
    fn write(&self, res: &mut [u8]) {
        let mut d = u128::from_be_bytes(self.md);
        for i in 0..32 {
            let x = (d & 0xF) as u8;
            res[31-i] = if x > 9 {
                x - 10 + b'a'
            } else {
                x + b'0'
            };
            d >>= 4;
        }
    }
}

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
                    let mut res = HashRes { md: [0; 16] };
                    let mut out = [0; 32];
                    h.result(&mut res.md);
                    res.write(&mut out);
                    for _ in 0..num {
                        h.reset();
                        h.input(&out);
                        h.result(&mut res.md);
                        res.write(&mut out);
                    }
                    (i, out)
                })
                .collect::<Vec<_>>()
        })
        .scan(vec![Vec::new(); 16], |pot, (i, hashed)| {
            let fives: Vec<usize> = hashed
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
            for w in hashed.windows(3) {
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
