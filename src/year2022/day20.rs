fn fix_refs<'a>(
    skip_size: usize,
    mut a: u16,
    mut b: u16,
    next: &mut [u16],
    far_prev: &mut [u16],
    far_next: &mut [u16],
) {
    for _ in 0..skip_size + 1 {
        far_next[a as usize] = b;
        far_prev[b as usize] = a;
        a = next[a as usize];
        b = next[b as usize];
    }
}

macro_rules! search {
    ($to_move:ident, $skip_size:ident, $cur:ident, $far_step:ident, $step:ident) => {
        for _ in 0..$to_move / $skip_size {
            $cur = $far_step[$cur as usize];
        }
        for _ in 0..$to_move % $skip_size {
            $cur = $step[$cur as usize];
        }
    };
}

fn mix(input: &str, scale: i64, times: usize) -> i64 {
    let ns: Vec<i64> = input
        .lines()
        .map(|x| x.parse::<i64>().unwrap() * scale)
        .collect();
    let skip_size = ((ns.len() / 2) as f64).sqrt() as usize / 2;
    let mut prev = (0..ns.len() as u16).collect::<Vec<_>>();
    let mut next = prev.clone();
    let mut far_prev = prev.clone();
    let mut far_next = prev.clone();
    prev.rotate_right(1);
    next.rotate_left(1);
    far_prev.rotate_right(skip_size);
    far_next.rotate_left(skip_size);
    let m = ns.len() - 1;
    for _ in 0..times {
        for (idx, n) in ns.iter().enumerate() {
            // Remove
            next[prev[idx] as usize] = next[idx];
            prev[next[idx] as usize] = prev[idx];
            fix_refs(
                skip_size,
                far_prev[idx],
                next[idx],
                &mut next,
                &mut far_prev,
                &mut far_next,
            );
            // Find new pos
            let mut to_move = n.rem_euclid(m as i64) as usize;
            let mut cur = next[idx];
            if to_move > m / 2 {
                to_move = m - to_move;
                search!(to_move, skip_size, cur, far_prev, prev);
            } else {
                search!(to_move, skip_size, cur, far_next, next);
            }
            // Insert
            next[prev[cur as usize] as usize] = idx as u16;
            prev[idx] = prev[cur as usize];
            prev[cur as usize] = idx as u16;
            next[idx] = cur;
            fix_refs(
                skip_size,
                far_prev[cur as usize],
                idx as u16,
                &mut next,
                &mut far_prev,
                &mut far_next,
            );
        }
    }
    let mut cur = ns.iter().position(|&x| x == 0).unwrap();
    let mut res = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            cur = next[cur] as usize;
        }
        res += ns[cur];
    }
    res
}

pub fn part1(input: &str) -> i64 {
    mix(input, 1, 1)
}

pub fn part2(input: &str) -> i64 {
    mix(input, 811589153, 10)
}
