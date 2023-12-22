use crate::utils::parsers::*;
use crate::utils::*;
use ahash::AHashSet;

#[derive(Clone, Debug)]
struct Cube {
    x: Interval<u32>,
    y: Interval<u32>,
    z: Interval<u32>,
    supporting: Vec<usize>,
    supported: Vec<usize>,
}

impl Cube {
    fn intersects(&self, o: &Self) -> bool {
        self.x.intersects(&o.x) && self.y.intersects(&o.y) && self.z.intersects(&o.z)
    }
}

fn brick(i: &mut &str) -> PResult<Cube> {
    let ((x0, y0, z0), (x1, y1, z1)) = sep2(coord3(u32), '~').parse_next(i)?;
    Ok(Cube {
        x: Interval::new(x0, x1 + 1),
        y: Interval::new(y0, y1 + 1),
        z: Interval::new(z0, z1 + 1),
        supporting: Vec::new(),
        supported: Vec::new(),
    })
}

fn fall(bricks: &mut Vec<Cube>) {
    bricks.sort_unstable_by_key(|b| b.z.lo);
    for i in 0..bricks.len() {
        let mut brick = bricks[i].clone();
        while brick.z.lo > 1 {
            brick.z -= 1;
            let mut intersected = false;
            for j in 0..i {
                if brick.intersects(&bricks[j]) {
                    intersected = true;
                    brick.supported.push(j);
                    bricks[j].supporting.push(i);
                }
            }
            if intersected {
                brick.z += 1;
                break;
            }
        }
        bricks[i] = brick;
    }
}

pub fn part1(input: &str) -> usize {
    let mut bricks = lines(brick).read(input);
    fall(&mut bricks);
    let all: AHashSet<_> = (0..bricks.len()).collect();
    let essential: AHashSet<_> = bricks
        .into_iter()
        .filter_map(|brick| (brick.supported.len() == 1).then(|| brick.supported[0]))
        .collect();
    (&all - &essential).len()
}

fn neighbors(bricks: &mut Vec<Cube>, idx: usize) -> Vec<usize> {
    let idxs: Vec<_> = bricks[idx].supporting.iter().copied().collect();
    let mut ns = Vec::new();
    for &i in &idxs {
        let j = bricks[i].supported.iter().position(|&v| v == idx).unwrap();
        bricks[i].supported.remove(j);
        if bricks[i].supported.len() == 0 {
            ns.push(i);
        }
    }
    ns
}

pub fn part2(input: &str) -> usize {
    let mut bricks = lines(brick).read(input);
    fall(&mut bricks);
    let mut fall = 0;
    for i in 0..bricks.len() {
        let mut bricks = bricks.clone();
        fall += bfs(i, |n| neighbors(&mut bricks, *n)).count() - 1;
    }
    fall
}
