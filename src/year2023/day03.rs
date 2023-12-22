use crate::utils::*;
use ahash::AHashSet;
use itertools::Itertools;

struct Engine {
    nums: Vec<u32>,
    mask: Grid<usize>,
    parts: Vec<(u8, C<usize>)>,
}

fn parse(input: &str) -> Engine {
    let grid: Grid<_> = input.bytes().collect();
    let mut nums = vec![0];
    let mut mask = grid.same_size();
    let mut parts = Vec::new();
    for (r, row) in grid.rows().enumerate() {
        let mut it = row.iter().enumerate().peekable();
        while it.peek().is_some() {
            if let Some(n) = it
                .peeking_take_while(|(_, v)| v.is_ascii_digit())
                .inspect(|&(c, _)| mask[(r, c)] = nums.len())
                .map(|(_, v)| (v - b'0') as u32)
                .reduce(|a, b| 10 * a + b)
            {
                nums.push(n)
            }

            it.peeking_take_while(|(_, v)| !v.is_ascii_digit())
                .filter(|&(_, v)| *v != b'.')
                .for_each(|(c, v)| parts.push((*v, C(r, c))));
        }
    }
    Engine { nums, mask, parts }
}

pub fn part1(input: &str) -> u32 {
    let Engine { nums, mask, parts } = parse(input);
    let idxs: AHashSet<usize> = parts
        .into_iter()
        .flat_map(|(_, pos)| adjacents(pos).map(|p| mask[p]))
        .collect();
    idxs.into_iter().map(|i| nums[i]).sum()
}

pub fn part2(input: &str) -> u32 {
    let Engine { nums, mask, parts } = parse(input);
    parts
        .into_iter()
        .filter(|&(p, _)| p == b'*')
        .filter_map(|(_, pos)| {
            let adjs: AHashSet<_> = adjacents(pos)
                .map(|p| mask[p])
                .filter(|&i| i != 0)
                .collect();
            (adjs.len() == 2).then(|| adjs.into_iter().map(|i| nums[i]).product::<u32>())
        })
        .sum()
}
