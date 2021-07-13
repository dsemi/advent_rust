use crate::utils::Coord;

#[derive(Eq, PartialEq)]
enum Orientation {
    V,
    H,
}

struct Segment {
    o: Orientation,
    a: Coord<i32>,
    b: Coord<i32>,
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
            let mut pos = Coord::new(0, 0);
            let mut steps = 0;
            Wire {
                parts: line
                    .split(',')
                    .map(|p| {
                        let (o, d) = match &p[..1] {
                            "U" => (Orientation::V, Coord::new(0, 1)),
                            "D" => (Orientation::V, Coord::new(0, -1)),
                            "L" => (Orientation::H, Coord::new(-1, 0)),
                            "R" => (Orientation::H, Coord::new(1, 0)),
                            _ => panic!("Unknown direction: {}", p),
                        };
                        let n = p[1..].parse().unwrap();
                        let prev = pos;
                        pos += d.scale(n);
                        let (d, a, b, r) = if prev < pos {
                            (steps, prev, pos, false)
                        } else {
                            (steps + n, pos, prev, true)
                        };
                        steps += n;
                        Segment {
                            o: o,
                            a: a,
                            b: b,
                            d: d,
                            r: r,
                        }
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
                (hs.a.x <= vs.a.x && vs.a.x <= hs.b.x && vs.a.y <= hs.a.y && hs.a.y <= vs.b.y).then(
                    || {
                        (
                            vs.a.x.abs() + hs.a.y.abs(),
                            hs.d + (if hs.r { -1 } else { 1 }) * (hs.a.x - vs.a.x).abs()
                                + vs.d
                                + (if vs.r { -1 } else { 1 }) * (vs.a.y - hs.a.y).abs(),
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
