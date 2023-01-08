use std::cell::Cell;

struct Node<'a> {
    val: i64,
    prev: Cell<Option<&'a Node<'a>>>,
    next: Cell<Option<&'a Node<'a>>>,
    far_prev: Cell<Option<&'a Node<'a>>>,
    far_next: Cell<Option<&'a Node<'a>>>,
}

fn fix_refs<'a>(skip_size: usize, mut a: &'a Node<'a>, mut b: &'a Node<'a>) {
    for _ in 0..skip_size + 1 {
        a.far_next.set(Some(b));
        b.far_prev.set(Some(a));
        a = a.next.get().unwrap();
        b = b.next.get().unwrap();
    }
}

macro_rules! search {
    ($to_move:ident, $skip_size:ident, $cur:ident, $far_step:ident, $step:ident) => {
        for _ in 0..$to_move / $skip_size {
            $cur = $cur.$far_step.get().unwrap();
        }
        for _ in 0..$to_move % $skip_size {
            $cur = $cur.$step.get().unwrap();
        }
    };
}

fn mix(input: &str, scale: i64, times: usize) -> i64 {
    let ns: Vec<Node> = input
        .lines()
        .map(|x| Node {
            val: x.parse::<i64>().unwrap() * scale,
            prev: Cell::new(None),
            next: Cell::new(None),
            far_prev: Cell::new(None),
            far_next: Cell::new(None),
        })
        .collect();
    let skip_size = ((ns.len() / 2) as f64).sqrt() as usize / 2;
    let m = ns.len() - 1;
    for i in 0..ns.len() {
        ns[(i + 1) % ns.len()].prev.set(Some(&ns[i]));
        ns[i].next.set(Some(&ns[(i + 1) % ns.len()]));
        ns[(i + skip_size) % ns.len()].far_prev.set(Some(&ns[i]));
        ns[i].far_next.set(Some(&ns[(i + skip_size) % ns.len()]));
    }
    for _ in 0..times {
        for n in ns.iter() {
            // Remove
            n.prev.get().unwrap().next.set(n.next.get());
            n.next.get().unwrap().prev.set(n.prev.get());
            fix_refs(skip_size, n.far_prev.get().unwrap(), n.next.get().unwrap());
            // Find new pos
            let mut to_move = n.val.rem_euclid(m as i64) as usize;
            let mut cur = n.next.get().unwrap();
            if to_move > m / 2 {
                to_move = m - to_move;
                search!(to_move, skip_size, cur, far_prev, prev);
            } else {
                search!(to_move, skip_size, cur, far_next, next);
            }
            // Insert
            cur.prev.get().unwrap().next.set(Some(n));
            n.prev.set(cur.prev.get());
            cur.prev.set(Some(n));
            n.next.set(Some(cur));
            fix_refs(skip_size, cur.far_prev.get().unwrap(), n);
        }
    }
    let mut cur = ns.iter().find(|x| x.val == 0).unwrap();
    let mut res = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            cur = cur.next.get().unwrap();
        }
        res += cur.val;
    }
    res
}

pub fn part1(input: &str) -> i64 {
    mix(input, 1, 1)
}

pub fn part2(input: &str) -> i64 {
    mix(input, 811589153, 10)
}
