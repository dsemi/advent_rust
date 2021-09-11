use safe_arch::*;

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

#[allow(non_camel_case_types)]
union m256i_64 {
    arr: [u64; 4],
    v: m256i,
}

const FACTOR_A4: m256i_64 = m256i_64 {
    arr: [984943658; 4],
};
const FACTOR_B4: m256i_64 = m256i_64 {
    arr: [1914720637; 4],
};
const MOD: m256i_64 = m256i_64 { arr: [0x7fffffff; 4] };
const LOWER_16: m256i_64 = m256i_64 { arr: [0xffff; 4] };
const ZERO: m256i_64 = m256i_64 { arr: [0; 4] };
const THREE: m256i_64 = m256i_64 { arr: [3; 4] };
const SEVEN: m256i_64 = m256i_64 { arr: [7; 4] };

fn parse(input: &str) -> (u64, u64) {
    let pts: Vec<u64> = input
        .lines()
        .map(|x| x.split_whitespace().last().unwrap().parse().unwrap())
        .collect();
    (pts[0], pts[1])
}

fn generate(mut x: u64, factor: u64) -> u64 {
    x *= factor;
    x = (x >> 31) + (x & 0x7fffffff);
    x = ((x >> 31) + x) & 0x7fffffff;
    x
}

unsafe fn generate4(mut v: m256i, factor: m256i) -> m256i {
    v = mul_u64_low_bits_m256i(v, factor);
    let c = m128i::from(31_u128);
    v = add_i64_m256i(shr_all_u64_m256i(v, c), v & MOD.v);
    v = add_i64_m256i(shr_all_u64_m256i(v, c), v) & MOD.v;
    v
}

pub fn part1(input: &str) -> u64 {
    let (a0, b0) = parse(input);
    let mut a = m256i_64 { arr: [a0, 0, 0, 0] };
    let mut b = m256i_64 { arr: [b0, 0, 0, 0] };
    unsafe {
        for i in 1..4 {
            a.arr[i] = generate(a.arr[i - 1], FACTOR_A);
            b.arr[i] = generate(b.arr[i - 1], FACTOR_B);
        }
        let mut ans = m256i_64 { arr: [0; 4] };
        for _ in 0..10_000_000 {
            ans.v = sub_i64_m256i(
                ans.v,
                cmp_eq_mask_i64_m256i(a.v & LOWER_16.v, b.v & LOWER_16.v),
            );
            a.v = generate4(a.v, FACTOR_A4.v);
            b.v = generate4(b.v, FACTOR_B4.v);
        }
        ans.arr[0] + ans.arr[1] + ans.arr[2] + ans.arr[3]
    }
}

unsafe fn do_mask(v: m256i, cmp: m256i) -> i32 {
    move_mask_m256d(cast_to_m256d_from_m256i(cmp_eq_mask_i64_m256i(
        v & cmp,
        ZERO.v,
    )))
}

pub fn part2(input: &str) -> u64 {
    let (a0, b0) = parse(input);
    let mut a = m256i_64 { arr: [a0, 0, 0, 0] };
    let mut b = m256i_64 { arr: [b0, 0, 0, 0] };
    unsafe {
        for i in 1..4 {
            a.arr[i] = generate(a.arr[i - 1], FACTOR_A);
            b.arr[i] = generate(b.arr[i - 1], FACTOR_B);
        }

        let mut ans = 0;
        let mut mask_a = do_mask(a.v, THREE.v);
        let mut mask_b = do_mask(b.v, SEVEN.v);
        for _ in 0..5_000_000 {
            while mask_a == 0 {
                a.v = generate4(a.v, FACTOR_A4.v);
                mask_a = do_mask(a.v, THREE.v);
            }
            while mask_b == 0 {
                b.v = generate4(b.v, FACTOR_B4.v);
                mask_b = do_mask(b.v, SEVEN.v);
            }
            let idx_a = mask_a.trailing_zeros();
            let idx_b = mask_b.trailing_zeros();

            mask_a ^= 1 << idx_a;
            mask_b ^= 1 << idx_b;

            if a.arr[idx_a as usize] & 0xffff == b.arr[idx_b as usize] & 0xffff {
                ans += 1;
            }
        }
        ans
    }
}
