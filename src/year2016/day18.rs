use itertools::iterate;

fn num_safe(n: u32, input: &str) -> u32 {
    let row_len = input.len() as u32;
    let row_mask = (1 << row_len) - 1;
    let row_one = input.bytes().fold(0u128, |acc, b| (acc << 1) | (b == b'^') as u128);
    let traps: u32 = iterate(row_one, |row| (row >> 1) ^ (row << 1) & row_mask)
        .take(n as usize)
        .map(|row| row.count_ones())
        .sum();
    n * row_len - traps
}

pub fn part1(input: &str) -> u32 {
    num_safe(40, input)
}

pub fn part2(input: &str) -> u32 {
    num_safe(400000, input)
}
