use itertools::iterate;
use safe_arch::*;
use std::mem::transmute;

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

const FACTOR_A4: m256i = unsafe { m256i(transmute([984943658_u64; 4])) };
const FACTOR_B4: m256i = unsafe { m256i(transmute([1914720637_u64; 4])) };
const MOD: m256i = unsafe { m256i(transmute([0x7fffffff_u64; 4])) };
const LOWER_16: m256i = unsafe { m256i(transmute([0xffff_u64; 4])) };

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

fn generate4(mut v: m256i, factor: m256i) -> m256i {
    v = mul_u64_low_bits_m256i(v, factor);
    v = add_i64_m256i(shr_imm_u64_m256i::<31>(v), v & MOD);
    add_i64_m256i(shr_imm_u64_m256i::<31>(v), v) & MOD
}

pub fn part1(input: &str) -> u64 {
    let (a0, b0) = parse(input);
    let mut a: m256i = generate(a0, FACTOR_A).into();
    let mut b: m256i = generate(b0, FACTOR_B).into();
    let mut ans = zeroed_m256i();
    for _ in 0..10_000_000 {
        ans = sub_i64_m256i(ans, cmp_eq_mask_i64_m256i(a & LOWER_16, b & LOWER_16));
        a = generate4(a, FACTOR_A4);
        b = generate4(b, FACTOR_B4);
    }
    <[u64; 4]>::from(ans).into_iter().sum()
}

fn do_mask(v: m256i, cmp: m256i) -> i32 {
    move_mask_m256d(cast_to_m256d_from_m256i(cmp_eq_mask_i64_m256i(
        v & cmp,
        zeroed_m256i(),
    )))
}

pub fn part2(input: &str) -> u64 {
    let (a0, b0) = parse(input);
    let mut a: m256i = generate(a0, FACTOR_A).into();
    let mut b: m256i = generate(b0, FACTOR_B).into();
    let mut ans = 0;
    let mut mask_a = do_mask(a, set_splat_i64_m256i(3));
    let mut mask_b = do_mask(b, set_splat_i64_m256i(7));
    for _ in 0..5_000_000 {
        while mask_a == 0 {
            a = generate4(a, FACTOR_A4);
            mask_a = do_mask(a, set_splat_i64_m256i(3));
        }
        while mask_b == 0 {
            b = generate4(b, FACTOR_B4);
            mask_b = do_mask(b, set_splat_i64_m256i(7));
        }
        let idx_a = mask_a.trailing_zeros() as usize;
        let idx_b = mask_b.trailing_zeros() as usize;

        mask_a ^= 1 << idx_a;
        mask_b ^= 1 << idx_b;

        if <[u64; 4]>::from(a)[idx_a] as u16 == <[u64; 4]>::from(b)[idx_b] as u16 {
            ans += 1;
        }
    }
    ans
}
