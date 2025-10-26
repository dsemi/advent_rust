use crate::utils::parsers::*;
use crate::utils::*;
use bit_set::BitSet;

#[derive(Clone)]
struct Loc {
    max_z: usize,
    brick_idx: Option<usize>,
}

impl Default for Loc {
    fn default() -> Self {
        Loc { max_z: 1, brick_idx: None }
    }
}

struct Cubes {
    x: Interval<usize>,
    y: Interval<usize>,
    z: Interval<usize>,
}

fn cubes(i: &mut &str) -> Result<Cubes> {
    let ((x0, y0, z0), (x1, y1, z1)) = sep2(coord3(usize), '~').parse_next(i)?;
    Ok(Cubes {
        x: Interval::new(x0, x1 + 1),
        y: Interval::new(y0, y1 + 1),
        z: Interval::new(z0, z1 + 1),
    })
}

#[derive(Clone, Default)]
struct Brick {
    above: BitSet<usize>,
    below: BitSet<usize>,
}

fn fall(input: &str) -> Vec<Brick> {
    let mut cubes = lines(cubes).read(input);
    cubes.sort_unstable_by_key(|b| b.z.lo);
    let (C(x0, y0), C(mut x1, mut y1)) = cubes
        .iter()
        .map(|b| (C(b.x.lo, b.y.lo), C(b.x.hi, b.y.hi)))
        .reduce(|(a, b), (c, d)| (a.smol(c), b.swol(d)))
        .unwrap();
    x1 -= x0;
    y1 -= y0;
    cubes.iter_mut().for_each(|cubes| {
        cubes.x -= x0;
        cubes.y -= y0;
    });
    let mut bricks = Vec::new();
    let mut grid: Grid<Loc> = Grid::new(x1, y1);
    for (i, mut cubes) in cubes.into_iter().enumerate() {
        let idxs = cubes.x.range().flat_map(|x| cubes.y.range().map(move |y| (x, y)));
        let z = idxs.clone().map(|p| grid[p].max_z).max().unwrap();
        cubes.z -= cubes.z.lo - z;
        bricks.push(Brick::default());
        idxs.for_each(|p| {
            let loc = &mut grid[p];
            if let Some(j) = loc.brick_idx.replace(i)
                && loc.max_z == z
            {
                bricks[j].above.insert(i);
                bricks[i].below.insert(j);
            }
            loc.max_z = cubes.z.hi;
        });
    }
    bricks
}

pub fn part1(input: &str) -> usize {
    let bricks = fall(input);
    let mut unsafe_bricks: BitSet<usize> = BitSet::default();
    bricks
        .iter()
        .filter(|brick| brick.below.len() == 1)
        .for_each(|brick| unsafe_bricks.extend(&brick.below));
    bricks.len() - unsafe_bricks.len()
}

pub fn part2(input: &str) -> usize {
    let bricks = fall(input);
    let mut removed_by = vec![usize::MAX; bricks.len()];
    let mut falls = 0;
    let mut no_incoming = Vec::new();
    for i in 0..bricks.len() {
        no_incoming.push(i);
        removed_by[i] = i;
        while let Some(j) = no_incoming.pop() {
            for a in &bricks[j].above {
                if removed_by[a] != i && bricks[a].below.into_iter().all(|x| removed_by[x] == i) {
                    falls += 1;
                    removed_by[a] = i;
                    no_incoming.push(a);
                }
            }
        }
    }
    falls
}
