use crate::utils::*;
use itertools::chain;
use std::cmp::{max, min};
use std::mem::transmute;
use Dir::*;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod simd {
    use core::arch::x86_64::*;

    pub type T = __m256i;

    pub fn zero() -> __m256i {
        unsafe { _mm256_setzero_si256() }
    }

    // All u8x32
    pub fn step_west(row: &__m256i) -> __m256i {
        unsafe {
            // (row >> u8x32(1)) | (row.rotate_lanes_left(1) << u8x32(7))
            _mm256_or_si256(
                _mm256_srli_epi64(*row, 1),
                _mm256_slli_epi64(
                    _mm256_blend_epi32::<0b00_11_11_11>(
                        _mm256_setzero_si256(),
                        _mm256_permute4x64_epi64(*row, 0b00_11_10_01),
                    ),
                    63,
                ),
            )
        }
    }

    pub fn step_east(row: &__m256i) -> __m256i {
        unsafe {
            // (row << u8x32(1)) | (row.rotate_lanes_right(1) >> u8x32(7))
            _mm256_or_si256(
                _mm256_slli_epi64(*row, 1),
                _mm256_srli_epi64(
                    _mm256_blend_epi32::<0b11_11_11_00>(
                        _mm256_setzero_si256(),
                        _mm256_permute4x64_epi64(*row, 0b10_01_00_11),
                    ),
                    63,
                ),
            )
        }
    }

    #[expect(clippy::too_many_arguments)]
    pub fn or8(
        a: __m256i,
        b: __m256i,
        c: __m256i,
        d: __m256i,
        e: __m256i,
        f: __m256i,
        g: __m256i,
        h: __m256i,
    ) -> __m256i {
        unsafe {
            _mm256_or_si256(
                a,
                _mm256_or_si256(
                    b,
                    _mm256_or_si256(
                        c,
                        _mm256_or_si256(
                            d,
                            _mm256_or_si256(e, _mm256_or_si256(f, _mm256_or_si256(g, h))),
                        ),
                    ),
                ),
            )
        }
    }

    pub fn or4(a: __m256i, b: __m256i, c: __m256i, d: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(a, _mm256_or_si256(b, _mm256_or_si256(c, d))) }
    }

    pub fn or3(a: __m256i, b: __m256i, c: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(a, _mm256_or_si256(b, c)) }
    }

    pub fn or(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_or_si256(a, b) }
    }

    pub fn and(a: __m256i, b: __m256i) -> __m256i {
        unsafe { _mm256_and_si256(a, b) }
    }

    pub fn not(a: __m256i) -> __m256i {
        unsafe { _mm256_xor_si256(a, _mm256_set1_epi16(-1)) }
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod simd {
    use core::arch::aarch64::*;

    pub type T = uint8x16x2_t;

    pub fn zero() -> uint8x16x2_t {
        unsafe {
            let x = vld1q_dup_u8(&0);
            uint8x16x2_t(x, x)
        }
    }

    // All u8x32
    pub fn step_west(row: &uint8x16x2_t) -> uint8x16x2_t {
        unsafe {
            // (row >> u8x32(1)) | (row.rotate_lanes_left(1) << u8x32(7))
            uint8x16x2_t(
                vorrq_u8(vshrq_n_u8(row.0, 1), vshlq_n_u8(vextq_u8(row.0, row.1, 1), 7)),
                vorrq_u8(vshrq_n_u8(row.1, 1), vshlq_n_u8(vextq_u8(row.1, row.0, 1), 7)),
            )
        }
    }

    pub fn step_east(row: &uint8x16x2_t) -> uint8x16x2_t {
        unsafe {
            // (row << u8x32(1)) | (row.rotate_lanes_right(1) >> u8x32(7))
            uint8x16x2_t(
                vorrq_u8(vshlq_n_u8(row.0, 1), vshrq_n_u8(vextq_u8(row.1, row.0, 15), 7)),
                vorrq_u8(vshlq_n_u8(row.1, 1), vshrq_n_u8(vextq_u8(row.0, row.1, 15), 7)),
            )
        }
    }

    #[expect(clippy::too_many_arguments)]
    pub fn or8(
        a: uint8x16x2_t,
        b: uint8x16x2_t,
        c: uint8x16x2_t,
        d: uint8x16x2_t,
        e: uint8x16x2_t,
        f: uint8x16x2_t,
        g: uint8x16x2_t,
        h: uint8x16x2_t,
    ) -> uint8x16x2_t {
        unsafe {
            uint8x16x2_t(
                vorrq_u8(
                    a.0,
                    vorrq_u8(
                        b.0,
                        vorrq_u8(
                            c.0,
                            vorrq_u8(d.0, vorrq_u8(e.0, vorrq_u8(f.0, vorrq_u8(g.0, h.0)))),
                        ),
                    ),
                ),
                vorrq_u8(
                    a.1,
                    vorrq_u8(
                        b.1,
                        vorrq_u8(
                            c.1,
                            vorrq_u8(d.1, vorrq_u8(e.1, vorrq_u8(f.1, vorrq_u8(g.1, h.1)))),
                        ),
                    ),
                ),
            )
        }
    }

    pub fn or4(a: uint8x16x2_t, b: uint8x16x2_t, c: uint8x16x2_t, d: uint8x16x2_t) -> uint8x16x2_t {
        unsafe {
            uint8x16x2_t(
                vorrq_u8(a.0, vorrq_u8(b.0, vorrq_u8(c.0, d.0))),
                vorrq_u8(a.1, vorrq_u8(b.1, vorrq_u8(c.1, d.1))),
            )
        }
    }

    pub fn or3(a: uint8x16x2_t, b: uint8x16x2_t, c: uint8x16x2_t) -> uint8x16x2_t {
        unsafe {
            uint8x16x2_t(vorrq_u8(a.0, vorrq_u8(b.0, c.0)), vorrq_u8(a.1, vorrq_u8(b.1, c.1)))
        }
    }

    pub fn or(a: uint8x16x2_t, b: uint8x16x2_t) -> uint8x16x2_t {
        unsafe { uint8x16x2_t(vorrq_u8(a.0, b.0), vorrq_u8(a.1, b.1)) }
    }

    pub fn and(a: uint8x16x2_t, b: uint8x16x2_t) -> uint8x16x2_t {
        unsafe { uint8x16x2_t(vandq_u8(a.0, b.0), vandq_u8(a.1, b.1)) }
    }

    pub fn not(a: uint8x16x2_t) -> uint8x16x2_t {
        unsafe {
            let neg1 = vreinterpretq_u8_s8(vld1q_dup_s8(&-1));
            uint8x16x2_t(veorq_u8(a.0, neg1), veorq_u8(a.1, neg1))
        }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

fn propose(
    dirs: [Dir; 4],
    [nw, n, ne]: &[simd::T; 3],
    [w, c, e]: &[simd::T; 3],
    [sw, s, se]: &[simd::T; 3],
) -> [simd::T; 4] {
    let mut proposals = [*c; 4];
    let mut passed = simd::or8(*nw, *n, *ne, *w, *e, *sw, *s, *se);
    for d in dirs {
        let (row, dir) = match d {
            North => (&mut proposals[0], simd::not(simd::or3(*ne, *n, *nw))),
            South => (&mut proposals[1], simd::not(simd::or3(*se, *s, *sw))),
            West => (&mut proposals[2], simd::not(simd::or3(*nw, *w, *sw))),
            East => (&mut proposals[3], simd::not(simd::or3(*ne, *e, *se))),
        };
        *row = simd::and(*row, simd::and(dir, passed));
        passed = simd::and(passed, simd::not(dir));
    }
    proposals
}

fn check_collisions(
    [_, s, _, _]: &[simd::T; 4],
    [_, _, w, e]: &[simd::T; 4],
    [n, _, _, _]: &[simd::T; 4],
) -> [simd::T; 4] {
    [
        simd::and(*n, simd::not(*s)),
        simd::and(*s, simd::not(*n)),
        simd::and(simd::step_west(w), simd::not(simd::step_east(e))),
        simd::and(simd::step_east(e), simd::not(simd::step_west(w))),
    ]
}

#[derive(Clone)]
struct Grid {
    es: [simd::T; 160],
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut res = Self { es: [simd::zero(); 160] };
        for (line, r) in input.lines().zip(24..) {
            let mut row = [0_u64; 4];
            for (v, c) in line.chars().zip(72..) {
                row[c / 64] |= ((v == '#') as u64) << (c % 64);
            }
            res.es[r] = unsafe { transmute::<[u64; 4], simd::T>(row) };
        }
        res
    }

    // Can remove once map_windows is stabilized.
    #[expect(unstable_name_collisions)]
    fn step(&mut self, dirs: &mut [Dir; 4]) -> bool {
        let mut next = self.clone();
        let mut moved = false;
        let zeroes = [simd::zero(); 2];
        chain!(&zeroes, &self.es, &zeroes)
            .map(|row| [simd::step_east(row), *row, simd::step_west(row)])
            .map_windows(|[above, cur, below]| propose(*dirs, above, cur, below))
            .map_windows(|[above, cur, below]| check_collisions(above, cur, below))
            .enumerate()
            .for_each(|(i, [from_s, from_n, from_e, from_w])| {
                let dests = simd::or4(from_n, from_s, from_w, from_e);
                if unsafe { transmute::<simd::T, [u64; 4]>(dests) } == [0_u64; 4] {
                    return;
                }
                moved = true;
                next.es[i + 1] = simd::and(next.es[i + 1], simd::not(from_s));
                next.es[i - 1] = simd::and(next.es[i - 1], simd::not(from_n));
                next.es[i] = simd::and(
                    next.es[i],
                    simd::and(
                        simd::not(simd::step_west(&from_w)),
                        simd::not(simd::step_east(&from_e)),
                    ),
                );
                next.es[i] = simd::or(next.es[i], dests);
            });
        dirs.rotate_left(1);
        *self = next;
        moved
    }
}

pub fn part1(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    let mut dirs = [North, South, West, East];
    for _ in 0..10 {
        grid.step(&mut dirs);
    }
    let (mut min_x, mut min_y) = (u32::MAX, u32::MAX);
    let (mut max_x, mut max_y) = (u32::MIN, u32::MIN);
    let mut elf_cnt = 0;
    for (r, row) in grid.es.into_iter().enumerate() {
        let ns: [u64; 4] = unsafe { transmute(row) };
        if ns == [0; 4] {
            continue;
        }
        min_y = min(min_y, r as u32);
        max_y = max(max_y, r as u32 + 1);
        let mut iter = ns.iter().rev().enumerate().filter(|p| *p.1 != 0).peekable();
        let (i, n) = iter.peek().unwrap();
        min_x = min(min_x, 64 * *i as u32 + n.leading_zeros());
        let (i, n) = iter.last().unwrap();
        max_x = max(max_x, 64 * i as u32 + 64 - n.trailing_zeros());
        elf_cnt += ns.into_iter().map(|n| n.count_ones()).sum::<u32>();
    }
    (max_x - min_x) * (max_y - min_y) - elf_cnt
}

pub fn part2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let mut dirs = [North, South, West, East];
    let mut i = 1;
    while grid.step(&mut dirs) {
        i += 1;
    }
    i
}
