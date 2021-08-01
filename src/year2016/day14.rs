use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;
use safe_arch::*;

const CHUNK_SIZE: usize = 8000;

union HashRes {
    n: m128i,
    md: [u8; 16],
}

union HexStr {
    n: m256i,
    hex: [u8; 32],
}

impl HashRes {
    unsafe fn write(&self, res: &mut HexStr) {
        // Scale up for 32 chars
        res.n = convert_to_i16_m256i_from_u8_m128i(self.n);
        // Swap half byte pairs to get proper ordering
        res.n = shr_all_i16_m256i(res.n, m128i::from(4_i128))
            | (shl_all_u16_m256i(res.n, m128i::from(8_u128)) & set_splat_i16_m256i(0xf00));
        // Add ASCII code pointer for digit/letter
        res.n = add_i8_m256i(
            res.n,
            add_i8_m256i(
                m256i::from([48_i8; 32]),
                m256i::from([39_i8; 32]) & cmp_gt_mask_i8_m256i(res.n, m256i::from([9_i8; 32])),
            ),
        );
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
                    let mut out = HexStr { hex: [0; 32] };
                    unsafe {
                        h.result(&mut res.md);
                        res.write(&mut out);
                        for _ in 0..num {
                            h.reset();
                            h.input(&out.hex);
                            h.result(&mut res.md);
                            res.write(&mut out);
                        }
                        (i, out.hex)
                    }
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
