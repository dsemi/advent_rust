use crate::utils::UniqueIdx;
use smallvec::{smallvec as sv, SmallVec};

fn blink(stone: u64, mut f: impl FnMut(u64) -> usize) -> SmallVec<[usize; 2]> {
    if stone == 0 {
        return sv![f(1)];
    }
    let digits = stone.ilog10() + 1;
    if digits & 1 == 0 {
        let divisor = 10_u64.pow(digits / 2);
        return sv![f(stone / divisor), f(stone % divisor)];
    }
    sv![f(stone * 2024)]
}

fn count(stones: &[u64], blinks: usize) -> usize {
    let mut counts = Vec::new();
    let mut ui = UniqueIdx::new();
    let mut new_stones = Vec::new();
    let mut transforms = Vec::new();
    stones.iter().for_each(|&stone| {
        let idx = ui.idx_with(stone, || {
            new_stones.push(stone);
            counts.push(0)
        });
        counts[idx] += 1;
    });
    for _ in 0..blinks {
        let stones = std::mem::take(&mut new_stones);
        stones.into_iter().for_each(|stone| {
            transforms.push(blink(stone, |n| ui.idx_with(n, || new_stones.push(n))))
        });
        let mut next = vec![0; ui.len()];
        transforms.iter().zip(counts).for_each(|(ts, cnt)| ts.iter().for_each(|&t| next[t] += cnt));
        counts = next;
    }
    counts.into_iter().sum()
}

pub fn part1(input: Vec<u64>) -> usize {
    count(&input, 25)
}

pub fn part2(input: Vec<u64>) -> usize {
    count(&input, 75)
}
