use itertools::iterate;
use std::mem::transmute;

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

fn parse(input: &str) -> (u64, u64) {
    let pts: Vec<u64> = input
        .lines()
        .map(|x| x.split_whitespace().last().unwrap().parse().unwrap())
        .collect();
    (pts[0], pts[1])
}

fn generate(x: u64, factor: u64) -> [u64; 4] {
    iterate(x, |x| {
        let mut x = *x;
        x *= factor;
        x = (x >> 31) + (x & 0x7fffffff);
        ((x >> 31) + x) & 0x7fffffff
    })
    .take(4)
    .collect::<Vec<_>>()
    .try_into()
    .unwrap()
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod simd {
    use core::arch::x86_64::*;
    use std::mem::transmute;

    pub type T = __m256i;

    pub const FACTOR_A4: __m256i = unsafe { transmute([984943658_u64; 4]) };
    pub const FACTOR_B4: __m256i = unsafe { transmute([1914720637_u64; 4]) };
    pub const MOD: __m256i = unsafe { transmute([0x7fffffff_u64; 4]) };
    const LOWER_16: __m256i = unsafe { transmute([0xffff_u64; 4]) };

    pub fn generate4(v: &mut __m256i, factor: __m256i) -> __m256i {
        todo!("Verify this works");
        unsafe {
            // u64x4(v) * factor - Only multiplies lower 32
            *v = _mm256_mul_epu32(*v, factor);
            // (v >> 31) + (v & MOD)
            *v = _mm256_add_epi64(_mm256_srli_epi64(*v, 31), _mm256_and_si256(*v, MOD));
            // ((v >> 31) + v) & MOD
            *v = _mm256_and_si256(_mm256_add_epi64(_mm256_srli_epi64(*v, 31), *v), MOD)
        }
    }

    pub fn count_eq(ans: &mut __m256i, a: __m256i, b: __m256i) {
        // i64x4(ans) - ((a & LOWER_16) == (b & LOWER_16))
        unsafe {
            *ans = _mm256_sub_epi64(
                *ans,
                _mm256_cmpeq_epi64(_mm256_and_si256(a, LOWER_16), _mm256_and_si256(b, LOWER_16)),
            );
        }
    }

    pub fn do_mask(v: __m256i, cmp: __m256i) -> i32 {
        unsafe {
            // get_sign_bits_of_4_lanes(v & cmp == 0)
            _mm256_movemask_pd(_mm256_castsi256_pd(_mm256_cmpeq_epi64(
                _mm256_and_si256(v, cmp),
                splat(0),
            )))
        }
    }

    pub fn splat(n: i64) -> __m256i {
        unsafe { _mm256_set1_epi64x(n) }
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod simd {
    use core::arch::aarch64::*;
    use std::mem::transmute;

    pub type T = int64x2x2_t;

    pub const FACTOR_A4: int64x2_t = unsafe { transmute([984943658_u64; 2]) };
    pub const FACTOR_B4: int64x2_t = unsafe { transmute([1914720637_u64; 2]) };
    pub const MOD: int64x2_t = unsafe { transmute([0x7fffffff_u64; 2]) };
    const LOWER_16: int64x2_t = unsafe { transmute([0xffff_u64; 2]) };

    pub fn generate4(v: &mut int64x2x2_t, factor: int64x2_t) {
        unsafe {
            // u64x4(v) * factor - Only multiplies lower 32
            v.0 = vmull_s32(vmovn_s64(v.0), vmovn_s64(factor));
            v.1 = vmull_s32(vmovn_s64(v.1), vmovn_s64(factor));
            // (v >> 31) + (v & MOD)
            v.0 = vaddq_s64(vshrq_n_s64(v.0, 31), vandq_s64(v.0, MOD));
            v.1 = vaddq_s64(vshrq_n_s64(v.1, 31), vandq_s64(v.1, MOD));
            // ((v >> 31) + v) & MOD
            v.0 = vandq_s64(vaddq_s64(vshrq_n_s64(v.0, 31), v.0), MOD);
            v.1 = vandq_s64(vaddq_s64(vshrq_n_s64(v.1, 31), v.1), MOD);
        }
    }

    pub fn count_eq(ans: &mut int64x2x2_t, a: int64x2x2_t, b: int64x2x2_t) {
        unsafe {
            // i64x4(ans) - ((a & LOWER_16) == (b & LOWER_16))
            ans.0 = vsubq_s64(
                ans.0,
                vreinterpretq_s64_u64(vceqq_s64(
                    vandq_s64(a.0, LOWER_16),
                    vandq_s64(b.0, LOWER_16),
                )),
            );
            ans.1 = vsubq_s64(
                ans.1,
                vreinterpretq_s64_u64(vceqq_s64(
                    vandq_s64(a.1, LOWER_16),
                    vandq_s64(b.1, LOWER_16),
                )),
            );
        }
    }

    pub fn do_mask(v: int64x2x2_t, cmp: int64x2_t) -> i32 {
        unsafe {
            // get_sign_bits_of_4_lanes(v & cmp == 0)
            // There doesn't appear to be a nice way to count sign bits with neon.
            let mut n = 0;
            let cmps = transmute::<uint64x2x2_t, [u64; 4]>(uint64x2x2_t(
                vceqzq_s64(vandq_s64(v.0, cmp)),
                vceqzq_s64(vandq_s64(v.1, cmp)),
            ));
            for (i, c) in cmps.into_iter().enumerate() {
                n |= ((c & 1) << i) as i32;
            }
            n
        }
    }

    pub fn splat(n: i64) -> int64x2_t {
        unsafe { vld1q_dup_s64(&n) }
    }
}

pub fn part1(input: &str) -> i64 {
    let (a0, b0) = parse(input);
    let mut a: simd::T = unsafe { transmute(generate(a0, FACTOR_A)) };
    let mut b: simd::T = unsafe { transmute(generate(b0, FACTOR_B)) };
    let mut ans: simd::T = unsafe { transmute([0_u64; 4]) };
    for _ in 0..10_000_000 {
        simd::count_eq(&mut ans, a, b);
        simd::generate4(&mut a, simd::FACTOR_A4);
        simd::generate4(&mut b, simd::FACTOR_B4);
    }
    unsafe { transmute::<simd::T, [i64; 4]>(ans).into_iter().sum() }
}

pub fn part2(input: &str) -> i64 {
    let (a0, b0) = parse(input);
    let mut a: simd::T = unsafe { transmute(generate(a0, FACTOR_A)) };
    let mut b: simd::T = unsafe { transmute(generate(b0, FACTOR_B)) };
    let mut ans = 0;
    let mut mask_a = simd::do_mask(a, simd::splat(3));
    let mut mask_b = simd::do_mask(b, simd::splat(7));
    for _ in 0..5_000_000 {
        while mask_a == 0 {
            simd::generate4(&mut a, simd::FACTOR_A4);
            mask_a = simd::do_mask(a, simd::splat(3));
        }
        while mask_b == 0 {
            simd::generate4(&mut b, simd::FACTOR_B4);
            mask_b = simd::do_mask(b, simd::splat(7));
        }
        let idx_a = mask_a.trailing_zeros() as usize;
        let idx_b = mask_b.trailing_zeros() as usize;

        mask_a ^= 1 << idx_a;
        mask_b ^= 1 << idx_b;

        unsafe {
            if transmute::<simd::T, [u64; 4]>(a)[idx_a] as u16
                == transmute::<simd::T, [u64; 4]>(b)[idx_b] as u16
            {
                ans += 1;
            }
        }
    }
    ans
}
