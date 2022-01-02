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

fn constellations(mut pts: Vec<Coord4>) -> Vec<Vec<Coord4>> {
    let mut result = Vec::new();
    while let Some(neighb) = pts.pop() {
        let mut neighbs = vec![neighb];
        let mut changed = true;
        while std::mem::replace(&mut changed, false) {
            pts.retain(|p| {
                if neighbs.iter().any(|x| dist(*x, *p) <= 3) {
                    changed = true;
                    neighbs.push(*p);
                    return false;
                }
                true
            });
        }
        result.push(neighbs);
    }
    result
}

pub fn part1(input: &str) -> usize {
    constellations(parse_points(input)).len()
}

pub fn part2(_input: &str) -> String {
    " ".to_string()
}
