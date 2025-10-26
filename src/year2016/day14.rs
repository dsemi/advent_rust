#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use core::arch::aarch64::*;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
use core::arch::x86_64::*;
use hybrid_array::Array;
use md5::{Digest, Md5};
use rayon::prelude::*;

const CHUNK_SIZE: usize = 8000;

union Sum {
    hex: [u8; 32],
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    avx2: __m256i,
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    neon16x8x2: uint16x8x2_t,
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    neon8x16x2: uint8x16x2_t,
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
fn write(res: [u8; 16], out: &mut Sum) {
    unsafe {
        // Scale up for 32 chars
        out.avx2 = _mm256_cvtepu8_epi16(std::mem::transmute::<[u8; 16], __m128i>(res));
        // Swap half byte pairs to get proper ordering
        out.avx2 = _mm256_or_si256(
            _mm256_srli_epi16(out.avx2, 4),
            _mm256_and_si256(_mm256_slli_epi16(out.avx2, 8), _mm256_set1_epi16(0xf00)),
        );
        // Add ASCII code pointer for digit (48) / letter (10 + 48 + 39)
        out.avx2 = _mm256_add_epi8(
            out.avx2,
            _mm256_add_epi8(
                _mm256_set1_epi8(48),
                _mm256_and_si256(
                    _mm256_set1_epi8(39),
                    _mm256_cmpgt_epi8(out.avx2, _mm256_set1_epi8(9)),
                ),
            ),
        );
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn write(res: [u8; 16], out: &mut Sum) {
    unsafe {
        // Scale up for 32 chars
        out.neon16x8x2.0 = vmovl_u8(vld1_u8(&res[0]));
        out.neon16x8x2.1 = vmovl_u8(vld1_u8(&res[8]));
        // Swap half byte pairs to get proper ordering
        out.neon16x8x2.0 = vorrq_u16(
            vshrq_n_u16(out.neon16x8x2.0, 4),
            vandq_u16(vshlq_n_u16(out.neon16x8x2.0, 8), vld1q_dup_u16(&0xf00)),
        );
        out.neon16x8x2.1 = vorrq_u16(
            vshrq_n_u16(out.neon16x8x2.1, 4),
            vandq_u16(vshlq_n_u16(out.neon16x8x2.1, 8), vld1q_dup_u16(&0xf00)),
        );
        // Add ASCII code pointer for digit (48) / letter (10 + 48 + 39)
        out.neon8x16x2.0 = vaddq_u8(
            out.neon8x16x2.0,
            vaddq_u8(
                vld1q_dup_u8(&48),
                vandq_u8(vld1q_dup_u8(&39), vcgtq_u8(out.neon8x16x2.0, vld1q_dup_u8(&9))),
            ),
        );
        out.neon8x16x2.1 = vaddq_u8(
            out.neon8x16x2.1,
            vaddq_u8(
                vld1q_dup_u8(&48),
                vandq_u8(vld1q_dup_u8(&39), vcgtq_u8(out.neon8x16x2.1, vld1q_dup_u8(&9))),
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
    hasher.update(seed);
    (0..)
        .step_by(CHUNK_SIZE)
        .flat_map(move |n| {
            (n..n + CHUNK_SIZE)
                .into_par_iter()
                .map(|i| {
                    let mut h = hasher.clone();
                    h.update(i.to_string());
                    let mut res = Array::default();
                    let mut out = Sum { hex: [0; 32] };
                    h.finalize_into_reset(&mut res);
                    write(<[u8; 16]>::from(res), &mut out);
                    for _ in 0..num {
                        h.update(unsafe { out.hex });
                        h.finalize_into_reset(&mut res);
                        write(<[u8; 16]>::from(res), &mut out);
                    }
                    (i, unsafe { out.hex })
                })
                .collect::<Vec<_>>()
        })
        .scan(vec![Vec::new(); 16], |pot, (i, hashed)| {
            let fives: Vec<usize> = hashed
                .windows(5)
                .filter(|w| w[0] == w[1] && w[0] == w[2] && w[0] == w[3] && w[0] == w[4])
                .flat_map(|w| {
                    pot[idx(w[0])].drain(..).filter(|&v| i - v <= 1000).collect::<Vec<_>>()
                })
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
