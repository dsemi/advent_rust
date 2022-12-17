use ahash::AHashMap;
use std::cmp::min;

const ROCKS: &[&[u8]] = &[
    &[0b0011110],
    &[0b0001000, 0b0011100, 0b0001000],
    &[0b0011100, 0b0000100, 0b0000100],
    &[0b0010000, 0b0010000, 0b0010000, 0b0010000],
    &[0b0011000, 0b0011000],
];

const N: usize = 50;

fn shift(grid: &[u8], rock: &mut [u8], h: usize, op: fn(u8, u8) -> u8, b: u8) {
    if rock.iter().any(|r| r & b != 0) {
        return;
    }
    for j in h..min(grid.len(), h + rock.len()) {
        if op(rock[j - h], 1) & grid[j] != 0 {
            return;
        }
    }
    for r in rock.iter_mut() {
        *r = op(*r, 1);
    }
}

fn left(grid: &[u8], rock: &mut [u8], h: usize) {
    shift(grid, rock, h, |a, b| a << b, 1 << 6)
}

fn right(grid: &[u8], rock: &mut [u8], h: usize) {
    shift(grid, rock, h, |a, b| a >> b, 1)
}

fn place(grid: &mut Vec<u8>, rock: &[u8], h: usize) -> bool {
    for j in h..min(grid.len(), h + rock.len()) {
        if rock[j - h] & grid[j] != 0 {
            for x in 0..rock.len() {
                grid[x + h + 1] |= rock[x];
            }
            while *grid.last().unwrap() == 0 {
                grid.pop();
            }
            return true;
        }
    }
    false
}

fn solve(input: &str, lim: usize) -> usize {
    let pattern = input.as_bytes();
    let mut grid = vec![0b1111111];
    let mut k = 0;
    let mut i = 0;
    let mut seen = AHashMap::new();
    let mut extra = 0;
    while i < lim {
        let mut rock = ROCKS[i % 5].to_owned();
        grid.extend([0, 0, 0, 0]);
        let mut h = grid.len() - 1;
        while !place(&mut grid, &rock, h) {
            match pattern[k % input.len()] {
                b'<' => left(&grid, &mut rock, h),
                b'>' => right(&grid, &mut rock, h),
                _ => unreachable!(),
            }
            k += 1;
            h -= 1;
        }
        if i > N {
            let state = (k % input.len(), i % 5, grid[grid.len() - N..].to_owned());
            if let Some((rock_n, height)) = seen.get(&state) {
                let dy = grid.len() - 1 - height;
                let di = i - rock_n;
                let amt = (lim - i) / di;
                extra += amt * dy;
                i += amt * di
            }
            seen.insert(state, (i, grid.len() - 1));
        }
        i += 1;
    }
    grid.len() - 1 + extra
}

pub fn part1(input: &str) -> usize {
    solve(input, 2022)
}

pub fn part2(input: &str) -> usize {
    solve(input, 1000000000000)
}
