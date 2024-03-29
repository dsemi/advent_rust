fn mirrors(ns: &[u32], i: usize, smudges: u32) -> bool {
    (0..i)
        .rev()
        .zip(i..ns.len())
        .map(|(a, b)| (ns[a] ^ ns[b]).count_ones())
        .sum::<u32>()
        == smudges
}

fn summarize(grid: &str, smudges: u32) -> Option<usize> {
    let mut rows = Vec::new();
    let mut cols = Vec::new();
    for line in grid.lines() {
        cols.resize(line.len(), 0);
        let mut row = 0;
        for (c, v) in line.bytes().enumerate() {
            cols[c] = (cols[c] << 1) | ((v == b'#') as u32);
            row = (row << 1) | ((v == b'#') as u32);
        }
        rows.push(row);
    }
    for c in 1..cols.len() {
        if mirrors(&cols, c, smudges) {
            return Some(c);
        }
    }
    for r in 1..rows.len() {
        if mirrors(&rows, r, smudges) {
            return Some(100 * r);
        }
    }
    None
}

fn solve(input: &str, smudges: u32) -> usize {
    input
        .split("\n\n")
        .map(|grid| summarize(grid, smudges).unwrap())
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, 0)
}

pub fn part2(input: &str) -> usize {
    solve(input, 1)
}
