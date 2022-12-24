use crate::utils::Coord;
use ahash::AHashSet;

struct Valley {
    w: i32,
    h: i32,
    blizz: Vec<(Coord<i32>, Coord<i32>)>,
    walls: AHashSet<Coord<i32>>,
}

impl Valley {
    fn new(input: &str) -> (Coord<i32>, Coord<i32>, Self) {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let h = grid.len() as i32 - 2;
        let w = grid[0].len() as i32 - 2;
        let start = Coord::new(0, 1);
        let goal = Coord::new(grid.len() as i32 - 1, grid[0].len() as i32 - 2);
        let mut blizz = Vec::new();
        let mut walls = AHashSet::new();
        walls.insert(start + Coord::new(-1, 0));
        walls.insert(goal + Coord::new(1, 0));
        for (r, row) in grid.into_iter().enumerate() {
            for (c, v) in row.into_iter().enumerate() {
                match v {
                    '^' => blizz.push((Coord::new(r as i32, c as i32), Coord::new(-1, 0))),
                    'v' => blizz.push((Coord::new(r as i32, c as i32), Coord::new(1, 0))),
                    '<' => blizz.push((Coord::new(r as i32, c as i32), Coord::new(0, -1))),
                    '>' => blizz.push((Coord::new(r as i32, c as i32), Coord::new(0, 1))),
                    '#' => {
                        walls.insert(Coord::new(r as i32, c as i32));
                    }
                    _ => (),
                }
            }
        }
        (start, goal, Valley { w, h, blizz, walls })
    }

    fn shortest_path(&mut self, start: Coord<i32>, goal: Coord<i32>) -> usize {
        let mut t = 0;
        let mut edges: AHashSet<Coord<i32>> = [start].into_iter().collect();
        while !edges.contains(&goal) {
            t += 1;
            let mut next_blizz = Vec::new();
            let mut blizz_set = AHashSet::new();
            for (pos, d) in self.blizz.iter() {
                let pos2 = Coord::new(
                    (pos.x + d.x - 1).rem_euclid(self.h) + 1,
                    (pos.y + d.y - 1).rem_euclid(self.w) + 1,
                );
                next_blizz.push((pos2, *d));
                blizz_set.insert(pos2);
            }
            self.blizz = next_blizz;
            let mut next_edges = AHashSet::new();
            for p in edges {
                if !self.walls.contains(&p) && !blizz_set.contains(&p) {
                    next_edges.insert(p);
                }
                for d in [
                    Coord::new(0, -1),
                    Coord::new(0, 1),
                    Coord::new(1, 0),
                    Coord::new(-1, 0),
                ] {
                    let p2 = p + d;
                    if !self.walls.contains(&p2) && !blizz_set.contains(&p2) {
                        next_edges.insert(p2);
                    }
                }
            }
            edges = next_edges;
        }
        t
    }
}

pub fn part1(input: &str) -> usize {
    let (start, goal, mut valley) = Valley::new(input);
    valley.shortest_path(start, goal)
}

pub fn part2(input: &str) -> usize {
    let (start, goal, mut valley) = Valley::new(input);
    valley.shortest_path(start, goal)
        + valley.shortest_path(goal, start)
        + valley.shortest_path(start, goal)
}
