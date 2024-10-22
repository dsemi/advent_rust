use crate::utils::*;
use ahash::{AHashMap, AHashSet};
use itertools::iterate;
use std::collections::VecDeque;
use Outcome::*;

#[derive(Eq, PartialEq)]
enum Outcome {
    Finished,
    ElfDied,
    EndedEarly,
}

fn parse_graph(input: &str) -> Grid<(char, i32), i32> {
    input
        .chars()
        .collect::<Grid<_, i32>>()
        .transform(|v| (v, if "EG".contains(v) { 200 } else { 0 }))
}

fn neighbors(coord: &C<i32>) -> impl Iterator<Item = C<i32>> + '_ {
    // reading order
    vec![(-1, 0), (0, -1), (0, 1), (1, 0)]
        .into_iter()
        .map(|(x, y)| C(x, y) + *coord)
}

fn find_next_move(grid: &Grid<(char, i32), i32>, enemy: char, coord: C<i32>) -> Option<C<i32>> {
    let mut path = AHashMap::new();
    let mut visited = AHashSet::new();
    visited.insert(coord);
    let mut frontier = VecDeque::new();
    frontier.push_back(coord);
    let mut result = None;
    while let Some(mut pos) = frontier.pop_front() {
        let neighbs = neighbors(&pos).collect::<Vec<_>>();
        if neighbs.iter().any(|&n| grid[n].0 == enemy) {
            while let Some(&next) = path.get(&pos) {
                result = Some(pos);
                pos = next;
            }
            break;
        }
        for n in neighbs {
            if grid[n].0 == '.' && visited.insert(n) {
                path.insert(n, pos);
                frontier.push_back(n);
            }
        }
    }
    result
}

fn run_round(grid: &mut Grid<(char, i32), i32>, elf_power: i32, allow_elf_death: bool) -> Outcome {
    let mut elves = 0;
    let mut goblins = 0;
    let units = grid
        .idx_iter()
        .filter(|(_, v)| "EG".contains(v.0))
        .map(|(C(r, c), v)| {
            if v.0 == 'E' {
                elves += 1;
            } else {
                goblins += 1;
            }
            C(r, c)
        })
        .collect::<Vec<_>>();
    for mut pos in units {
        if elves == 0 || goblins == 0 {
            return EndedEarly;
        }
        let v = grid[pos];
        if !"EG".contains(v.0) {
            continue;
        }
        let enemy = if v.0 == 'E' { 'G' } else { 'E' };
        if let Some(p) = find_next_move(grid, enemy, pos) {
            grid[pos] = ('.', 0);
            grid[p] = v;
            pos = p;
        }
        if let Some(t_pos) = neighbors(&pos)
            .filter(|&n| grid[n].0 == enemy)
            .min_by_key(|&n| grid[n].1)
        {
            let pwr = if v.0 == 'E' { elf_power } else { 3 };
            let (t, hp) = grid[t_pos];
            if hp <= pwr {
                if !allow_elf_death && t == 'E' {
                    return ElfDied;
                } else {
                    if t == 'E' {
                        elves -= 1;
                    } else {
                        goblins -= 1;
                    }
                    grid[t_pos] = ('.', 0);
                }
            } else {
                grid[t_pos] = (t, hp - pwr);
            }
        }
    }
    Finished
}

fn score(grid: &Grid<(char, i32), i32>, c: i32) -> Option<i32> {
    let mut elves = false;
    let mut goblins = false;
    let mut total = 0;
    for &(t, v) in grid {
        if t == 'E' {
            elves = true;
        } else if t == 'G' {
            goblins = true;
        }
        if elves && goblins {
            return None;
        }
        total += v;
    }
    Some(c * total)
}

fn run(mut grid: Grid<(char, i32), i32>, elf_pwr: i32, allow_elf_death: bool) -> Option<i32> {
    for c in 0.. {
        let res = run_round(&mut grid, elf_pwr, allow_elf_death);
        if res == ElfDied {
            break;
        }
        if let Some(sc) = score(&grid, if res == Finished { c + 1 } else { c }) {
            return Some(sc);
        }
    }
    None
}

pub fn part1(input: &str) -> Option<i32> {
    let grid = parse_graph(input);
    run(grid, 3, true)
}

pub fn part2(input: &str) -> Option<i32> {
    let grid_start = parse_graph(input);
    let n = iterate(4, |&x| x * 2)
        .find(|&x| run(grid_start.clone(), x, false).is_some())
        .unwrap();
    let v = (n / 2..=n).collect::<Vec<_>>();
    let i = v.partition_point(|&x| run(grid_start.clone(), x, false).is_none());
    run(grid_start, v[i], false)
}
