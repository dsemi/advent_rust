use crate::utils::bits;

fn parse(input: &str) -> (Vec<u128>, Vec<u128>, Vec<u128>, (usize, usize)) {
    let mut robot = (0, 0);
    let rows = input.lines().count();
    let (mut wall, mut box_l, mut box_r) = (vec![0; rows], vec![0; rows], vec![0; rows]);
    for (r, row) in input.lines().enumerate() {
        for (c, v) in row.chars().enumerate() {
            robot = if v == '@' { (r, c) } else { robot };
            match v {
                '#' => wall[r] |= 1 << c,
                'O' | '[' => box_l[r] |= 1 << c,
                ']' => box_r[r] |= 1 << c,
                _ => (),
            }
        }
    }
    (wall, box_l, box_r, robot)
}

fn go_v(wall: &[u128], l: &mut [u128], r: &mut [u128], orig: u128, from: usize, to: usize) -> bool {
    if orig & wall[to] != 0 {
        return false;
    }
    let mut next = orig & (l[to] | r[to]);
    next |= (next & l[to]) << 1 & r[to] | (next & r[to]) >> 1 & l[to];
    if next == 0 || go_v(wall, l, r, next, to, to + to - from) {
        l[to] |= l[from] & orig;
        r[to] |= r[from] & orig;
        l[from] &= !orig;
        r[from] &= !orig;
        return true;
    }
    false
}

fn go_h(wall: &u128, l: &mut u128, r: &mut u128, orig: u128, op: fn(u128) -> u128) -> bool {
    let next = op(orig);
    if next & *wall != 0 {
        return false;
    }
    if next & (*l | *r) == 0 || go_h(wall, l, r, next, op) {
        *l |= op(*l) & next;
        *r |= op(*r) & next;
        *l &= !orig;
        *r &= !orig;
        return true;
    }
    false
}

fn solve(warehouse: &str, path: &str) -> usize {
    let (wall, mut b_l, mut b_r, (mut r, mut c)) = parse(warehouse);
    for dir in path.chars().filter(|&b| b != '\n') {
        match dir {
            '^' if go_v(&wall, &mut b_l, &mut b_r, 1 << c, r, r - 1) => r -= 1,
            'v' if go_v(&wall, &mut b_l, &mut b_r, 1 << c, r, r + 1) => r += 1,
            '<' if go_h(&wall[r], &mut b_l[r], &mut b_r[r], 1 << c, |x| x >> 1) => c -= 1,
            '>' if go_h(&wall[r], &mut b_l[r], &mut b_r[r], 1 << c, |x| x << 1) => c += 1,
            _ => (),
        };
    }
    b_l.into_iter().enumerate().flat_map(|(r, row)| bits(row).map(move |b| 100 * r + b)).sum()
}

pub fn part1(input: &str) -> usize {
    let (warehouse, path) = input.split_once("\n\n").unwrap();
    solve(warehouse, path)
}

pub fn part2(input: &str) -> usize {
    let (warehouse, path) = input.split_once("\n\n").unwrap();
    let warehouse: String = warehouse
        .chars()
        .flat_map(|v| match v {
            '#' => vec!['#', '#'],
            'O' => vec!['[', ']'],
            '.' => vec!['.', '.'],
            '@' => vec!['@', '.'],
            _ => vec![v],
        })
        .collect();
    solve(&warehouse, path)
}
