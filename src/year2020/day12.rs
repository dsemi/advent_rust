use crate::utils::Coord;

fn travel(start: Coord<i64>, move_way: bool, s: &str) -> i64 {
    let mut st = [Coord::new(0, 0), start];
    let idx = usize::from(move_way);
    for line in s.lines() {
        let cmd = line.chars().next().unwrap();
        let n: i64 = line[1..].parse().unwrap();
        match cmd {
            'N' => st[idx] += Coord::new(0, n),
            'S' => st[idx] -= Coord::new(0, n),
            'E' => st[idx] += Coord::new(n, 0),
            'W' => st[idx] -= Coord::new(n, 0),
            'L' | 'R' => {
                st[1] *= (if cmd == 'R' {
                    Coord::new(0, -1)
                } else {
                    Coord::new(0, 1)
                })
                .pow(n / 90)
            }
            'F' => st[0] += st[1].scale(n),
            _ => panic!("Invalid instruction: {}", cmd),
        }
    }
    st[0].x.abs() + st[0].y.abs()
}

pub fn part1(input: &str) -> i64 {
    travel(Coord::new(1, 0), false, input)
}

pub fn part2(input: &str) -> i64 {
    travel(Coord::new(10, 1), true, input)
}
