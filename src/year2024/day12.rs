use crate::utils::*;
use hashbrown::HashSet;

fn expand(seen: &mut HashSet<C<i32>>, grid: &Grid<u8, i32>, C(r, c): C<i32>, v: u8) -> usize {
    if !seen.insert(C(r, c)) {
        return 0;
    }
    let mut perimeter = 4;
    for adj in [C(r + 1, c), C(r - 1, c), C(r, c + 1), C(r, c - 1)] {
        if *grid.get(adj).unwrap_or(&b'.') == v {
            perimeter += expand(seen, grid, adj, v) - 1;
        }
    }
    perimeter
}

pub fn part1(input: &str) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let mut visited = HashSet::new();
    let mut total = 0;
    for (i, &v) in grid.idx_iter() {
        if visited.insert(i) {
            let mut seen = HashSet::new();
            let p = expand(&mut seen, &grid, i, v);
            total += seen.len() * p;
            visited.extend(seen);
        }
    }
    total
}

fn sides(region: &HashSet<C<i32>>) -> usize {
    let mut sides = 0;
    for d in [C(1, 0), C(-1, 0), C(0, 1), C(0, -1)] {
        let boundary: HashSet<_> =
            region.iter().cloned().filter(|p| !region.contains(&(p + d))).collect();
        let b_vec: Vec<_> = boundary.iter().cloned().collect();
        let mut ui: UniqueIdx<_> = b_vec.iter().cloned().collect();
        let mut uf: UnionFind<_> = b_vec.iter().cloned().collect();
        for p in b_vec {
            let a = p + d * C(0, 1);
            if boundary.contains(&a) {
                uf.union(ui.idx(p), ui.idx(a));
            }
            let a = p + d * C(0, -1);
            if boundary.contains(&a) {
                uf.union(ui.idx(p), ui.idx(a));
            }
        }
        sides += uf.ncomponents();
    }
    sides
}

pub fn part2(input: &str) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let mut visited = HashSet::new();
    let mut total = 0;
    for (i, &v) in grid.idx_iter() {
        if visited.insert(i) {
            let mut seen = HashSet::new();
            expand(&mut seen, &grid, i, v);
            total += seen.len() * sides(&seen);
            visited.extend(seen);
        }
    }
    total
}
