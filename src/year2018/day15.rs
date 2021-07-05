use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::utils::*;
use crate::year2018::day15::Outcome::*;

#[derive(Eq, PartialEq)]
enum Outcome {
    Finished,
    ElfDied,
    EndedEarly,
}

fn parse_graph(input: &str) -> Vec<Vec<(char, i32)>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|v| (v, if "EG".contains(v) { 200 } else { 0 }))
                .collect()
        })
        .collect()
}

fn neighbors(coord: &Coord<i32>) -> Vec<Coord<i32>> {
    // reading order
    vec![(-1, 0), (0, -1), (0, 1), (1, 0)]
        .into_iter()
        .map(|(x, y)| Coord::new(x, y) + *coord)
        .collect()
}

fn find_next_move(
    grid: &Vec<Vec<(char, i32)>>,
    enemy: char,
    coord: Coord<i32>,
) -> Option<Coord<i32>> {
    let mut path = HashMap::new();
    let mut visited = HashSet::new();
    visited.insert(coord);
    let mut frontier = VecDeque::new();
    frontier.push_back(coord);
    let mut result = None;
    while let Some(mut pos) = frontier.pop_front() {
        let neighbs = neighbors(&pos);
        if neighbs
            .iter()
            .any(|n| grid[n.x as usize][n.y as usize].0 == enemy)
        {
            while path.contains_key(&pos) {
                result = Some(pos);
                pos = path[&pos];
            }
            break;
        }
        for n in neighbs {
            if grid[n.x as usize][n.y as usize].0 == '.' && !visited.contains(&n) {
                visited.insert(n);
                path.insert(n, pos);
                frontier.push_back(n);
            }
        }
    }
    result
}

fn run_round(grid: &mut Vec<Vec<(char, i32)>>, elf_power: i32, allow_elf_death: bool) -> Outcome {
    let mut elves = 0;
    let mut goblins = 0;
    let units = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(c, v)| {
                    "EG".contains(v.0).then(|| {
                        if v.0 == 'E' {
                            elves += 1;
                        } else {
                            goblins += 1;
                        }
                        Coord::new(r as i32, c as i32)
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for mut pos in units {
        if elves == 0 || goblins == 0 {
            return EndedEarly;
        }
        let v = grid[pos.x as usize][pos.y as usize];
        if !"EG".contains(v.0) {
            continue;
        }
        let enemy = if v.0 == 'E' { 'G' } else { 'E' };
        if let Some(p) = find_next_move(&grid, enemy, pos) {
            grid[pos.x as usize][pos.y as usize] = ('.', 0);
            grid[p.x as usize][p.y as usize] = v;
            pos = p;
        }
        let targets = neighbors(&pos)
            .into_iter()
            .filter(|&n| grid[n.x as usize][n.y as usize].0 == enemy)
            .collect::<Vec<_>>();
        if !targets.is_empty() {
            let pwr = if v.0 == 'E' { elf_power } else { 3 };
            let t_pos = targets
                .into_iter()
                .reduce(|a, b| {
                    if grid[b.x as usize][b.y as usize] < grid[a.x as usize][a.y as usize] {
                        b
                    } else {
                        a
                    }
                })
                .unwrap();
            let (t, hp) = grid[t_pos.x as usize][t_pos.y as usize];
            if hp <= pwr {
                if !allow_elf_death && t == 'E' {
                    return ElfDied;
                } else {
                    if t == 'E' {
                        elves -= 1;
                    } else {
                        goblins -= 1;
                    }
                    grid[t_pos.x as usize][t_pos.y as usize] = ('.', 0);
                }
            } else {
                grid[t_pos.x as usize][t_pos.y as usize] = (t, hp - pwr);
            }
        }
    }
    Finished
}

fn score(grid: &Vec<Vec<(char, i32)>>, c: i32) -> Option<i32> {
    let mut elves = false;
    let mut goblins = false;
    let mut total = 0;
    for row in grid {
        for (t, v) in row {
            if t == &'E' {
                elves = true;
            } else if t == &'G' {
                goblins = true;
            }
            if elves && goblins {
                return None;
            }
            total += v;
        }
    }
    Some(c * total)
}

pub fn part1(input: &str) -> Option<i32> {
    let mut grid = parse_graph(input);
    (0..)
        .filter_map(|c| {
            let res = run_round(&mut grid, 3, true);
            score(&grid, if res == Finished { c + 1 } else { c })
        })
        .next()
}

pub fn part2(input: &str) -> i32 {
    let grid_start = parse_graph(input);
    for elf_pwr in 3.. {
        let mut grid = grid_start.clone();
        for c in 0.. {
            let res = run_round(&mut grid, elf_pwr, false);
            if res == ElfDied {
                break;
            }
            if let Some(sc) = score(&grid, if res == Finished { c + 1 } else { c }) {
                return sc;
            }
        }
    }
    panic!("No solution")
}
