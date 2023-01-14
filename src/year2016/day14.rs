use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;
use safe_arch::*;

const CHUNK_SIZE: usize = 8000;

fn write(res: m128i, hex: &mut m256i) {
    // Scale up for 32 chars
    *hex = convert_to_i16_m256i_from_u8_m128i(res);
    // Swap half byte pairs to get proper ordering
    *hex =
        shr_imm_u16_m256i::<4>(*hex) | (shl_imm_u16_m256i::<8>(*hex) & set_splat_i16_m256i(0xf00));
    // Add ASCII code pointer for digit/letter
    *hex = add_i8_m256i(
        *hex,
        add_i8_m256i(
            set_splat_i8_m256i(48),
            set_splat_i8_m256i(39) & cmp_gt_mask_i8_m256i(*hex, set_splat_i8_m256i(9)),
        ),
    );
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
                    let mut h = hasher;
                    h.input_str(&i.to_string());
                    let mut res = [0; 16];
                    let mut out = zeroed_m256i();
                    h.result(&mut res);
                    write(res.into(), &mut out);
                    for _ in 0..num {
                        h.reset();
                        h.input(&<[u8; 32]>::from(out));
                        h.result(&mut res);
                        write(res.into(), &mut out);
                    }
                    (i, <[u8; 32]>::from(out))
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
