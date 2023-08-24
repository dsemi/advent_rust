use ahash::AHashMap;

fn acre(c: char) -> u128 {
    match c {
        '.' => 0,
        '|' => 1,
        '#' => 2,
        _ => panic!("Unrecognized char: {}", c),
    }
}

fn parse_landscape(input: &str) -> Vec<u128> {
    input
        .lines()
        .map(|line| line.chars().map(acre).reduce(|a, b| a << 2 | b).unwrap())
        .collect()
}

fn step(grid: &mut [u128]) {
    let mut prev = 0;
    for i in 0..grid.len() {
        let mut curr = grid[i];
        let next = *grid.get(i + 1).unwrap_or(&0);
        #[rustfmt::skip]
        let mut adjs = [prev << 2, prev, prev >> 2,
                        curr << 2,       curr >> 2,
                        next << 2, next, next >> 2];
        let mut curr2 = 0;
        for _ in 0..grid.len() {
            curr2 <<= 2;
            let mut adj_trees = 0;
            let mut adj_lumberyards = 0;
            for v in adjs.iter_mut() {
                adj_trees += (*v & 1) as u8;
                *v >>= 1;
                adj_lumberyards += (*v & 1) as u8;
                *v >>= 1;
            }
            let mut c = curr & 3;
            curr >>= 2;
            if c == 0 && adj_trees >= 3 {
                c = 1;
            } else if c == 1 && adj_lumberyards >= 3 {
                c = 2;
            } else if c == 2 && (adj_trees < 1 || adj_lumberyards < 1) {
                c = 0;
            }
            curr2 |= c;
        }
        prev = std::mem::replace(&mut grid[i], curr2);
    }
}

fn resource_value(grid: &[u128]) -> usize {
    let mut ws = 0;
    let mut ls = 0;
    for row in grid {
        let mut c = *row;
        while c > 0 {
            ws += (c & 1) as usize;
            c >>= 1;
            ls += (c & 1) as usize;
            c >>= 1;
        }
    }
    ws * ls
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse_landscape(input);
    for _ in 0..10 {
        step(&mut grid);
    }
    resource_value(&grid)
}

pub fn part2(input: &str) -> usize {
    const N: usize = 1_000_000_000;
    let mut t: AHashMap<Vec<u128>, usize> = AHashMap::new();
    let mut grid = parse_landscape(input);
    let mut rs = Vec::new();
    for c in 0..=N {
        rs.push(resource_value(&grid));
        if let Some(base) = t.get(&grid) {
            return rs[base + (N - base) % (c - base)];
        }
        t.insert(grid.clone(), c);
        step(&mut grid);
    }
    panic!("No solution found")
}
