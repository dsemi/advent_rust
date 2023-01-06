use std::cell::Cell;

struct Node<'a> {
    val: i64,
    prev: Cell<Option<&'a Node<'a>>>,
    next: Cell<Option<&'a Node<'a>>>,
    far_prev: Cell<Option<&'a Node<'a>>>,
    far_next: Cell<Option<&'a Node<'a>>>,
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
    let skip_size = ((ns.len() / 2) as f64).sqrt() as i64 / 2;
    let m = ns.len() as i64 - 1;
    for i in 0..ns.len() {
        ns[(i + 1) % ns.len()].prev.set(Some(&ns[i]));
        ns[i].next.set(Some(&ns[(i + 1) % ns.len()]));
        ns[(i + skip_size as usize) % ns.len()]
            .far_prev
            .set(Some(&ns[i]));
        ns[i]
            .far_next
            .set(Some(&ns[(i + skip_size as usize) % ns.len()]));
    }
    for _ in 0..times {
        for n in ns.iter() {
            // Remove
            let mut a = n.far_prev.get().unwrap();
            let mut b = n.next.get().unwrap();
            n.prev.get().unwrap().next.set(n.next.get());
            n.next.get().unwrap().prev.set(n.prev.get());
            for _ in 0..skip_size + 1 {
                a.far_next.set(Some(b));
                b.far_prev.set(Some(a));
                a = a.next.get().unwrap();
                b = b.next.get().unwrap();
            }
            // Find new pos
            let mut to_move = n.val.rem_euclid(m);
            b = n.next.get().unwrap();
            if to_move > m / 2 {
                to_move = m - to_move;
                while to_move >= skip_size {
                    to_move -= skip_size;
                    b = b.far_prev.get().unwrap();
                }
                for _ in 0..to_move {
                    b = b.prev.get().unwrap();
                }
            } else {
                while to_move >= skip_size {
                    to_move -= skip_size;
                    b = b.far_next.get().unwrap();
                }
                for _ in 0..to_move {
                    b = b.next.get().unwrap();
                }
            }
            // Insert
            b.prev.get().unwrap().next.set(Some(n));
            n.prev.set(b.prev.get());
            b.prev.set(Some(n));
            n.next.set(Some(b));
            a = b.far_prev.get().unwrap();
            b = n;
            for _ in 0..skip_size + 1 {
                a.far_next.set(Some(b));
                b.far_prev.set(Some(a));
                a = a.next.get().unwrap();
                b = b.next.get().unwrap();
            }
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
