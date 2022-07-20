use ahash::AHashMap;

type Coord4 = (i64, i64, i64, i64);

fn parse_points(input: &str) -> Vec<Coord4> {
    input
        .lines()
        .map(|line| {
            let ns = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            (ns[0], ns[1], ns[2], ns[3])
        })
        .collect()
}

fn dist(a: Coord4, b: Coord4) -> i64 {
    let (w0, x0, y0, z0) = a;
    let (w1, x1, y1, z1) = b;
    (w0 - w1).abs() + (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs()
}

fn find(parent: &[usize], mut k: usize) -> usize {
    while k != parent[k] {
        k = parent[k]
    }
    k
}

fn union(parent: &mut [usize], rank: &mut [usize], x: usize, y: usize) {
    let x_root = find(parent, x);
    let y_root = find(parent, y);
    if x_root == y_root {
        return;
    }
    if rank[x_root] < rank[y_root] {
        parent[x_root] = y_root;
    } else if rank[x_root] > rank[y_root] {
        parent[y_root] = x_root;
    } else {
        parent[y_root] = x_root;
        rank[x_root] += 1;
    }
}

fn constellations(pts: Vec<Coord4>) -> usize {
    let mut parent = vec![0; pts.len()];
    for (i, p) in parent.iter_mut().enumerate() {
        *p = i;
    }
    let mut rank = vec![0; pts.len()];
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            if dist(pts[i], pts[j]) <= 3 {
                union(&mut parent, &mut rank, i, j);
            }
        }
    }
    let mut m = AHashMap::new();
    for p in 0..pts.len() {
        m.insert(find(&parent, p), true);
    }
    m.len()
}

pub fn part1(input: &str) -> usize {
    constellations(parse_points(input))
}

pub fn part2(_input: &str) -> String {
    " ".to_string()
}
