use crate::utils::C;

#[derive(Eq, PartialEq)]
enum Orientation {
    V,
    H,
}

struct Segment {
    o: Orientation,
    a: C<i32>,
    b: C<i32>,
    d: i32,
    r: bool,
}

struct Wire {
    parts: Vec<Segment>,
}

fn parse_wires(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|line| {
            let mut pos = C(0, 0);
            let mut steps = 0;
            Wire {
                parts: line
                    .split(',')
                    .map(|p| {
                        let (o, d) = match p.as_bytes()[0] {
                            b'U' => (Orientation::V, C(0, 1)),
                            b'D' => (Orientation::V, C(0, -1)),
                            b'L' => (Orientation::H, C(-1, 0)),
                            b'R' => (Orientation::H, C(1, 0)),
                            _ => panic!("Unknown direction: {}", p),
                        };
                        let n = p[1..].parse::<i32>().unwrap();
                        let prev = pos;
                        pos += d * n;
                        let (d, a, b, r) = if prev < pos {
                            (steps, prev, pos, false)
                        } else {
                            (steps + n, pos, prev, true)
                        };
                        steps += n;
                        Segment { o, a, b, d, r }
                    })
                    .collect(),
            }
        })
        .collect()
}

impl Wire {
    fn intersections<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = (i32, i32)> + 'a {
        self.parts.iter().flat_map(move |w1| {
            other.parts.iter().filter_map(move |w2| {
                if w1.o == w2.o {
                    return None;
                }
                let (hs, vs) = if w1.o == Orientation::H {
                    (w1, w2)
                } else {
                    (w2, w1)
                };
                (hs.a.0 <= vs.a.0 && vs.a.0 <= hs.b.0 && vs.a.1 <= hs.a.1 && hs.a.1 <= vs.b.1).then(
                    || {
                        (
                            vs.a.0.abs() + hs.a.1.abs(),
                            hs.d + (if hs.r { -1 } else { 1 }) * (hs.a.0 - vs.a.0).abs()
                                + vs.d
                                + (if vs.r { -1 } else { 1 }) * (vs.a.1 - hs.a.1).abs(),
                        )
                    },
                )
            })
        })
    }
}

pub fn part1(input: &str) -> Option<i32> {
    let wires = &parse_wires(input);
    (0..wires.len())
        .flat_map(|w1| {
            (w1 + 1..wires.len()).flat_map(move |w2| wires[w1].intersections(&wires[w2]))
        })
        .map(|c| c.0)
        .min()
}

pub fn part2(input: &str) -> Option<i32> {
    let wires = &parse_wires(input);
    (0..wires.len())
        .flat_map(|w1| {
            (w1 + 1..wires.len()).flat_map(move |w2| wires[w1].intersections(&wires[w2]))
        })
        .map(|c| c.1)
        .min()
}
