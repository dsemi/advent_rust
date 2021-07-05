use std::collections::HashSet;
use std::collections::VecDeque;

type Planet = Vec<Vec<bool>>;

fn parse_grid(input: &str) -> Planet {
    input
        .lines()
        .map(|row| row.chars().map(|v| v == '#').collect())
        .collect()
}

fn neighbor_counts(p: &Planet) -> Vec<Vec<u64>> {
    let mut result = vec![vec![0; p[0].len()]; p.len()];
    for r in 0..p.len() {
        for c in 0..p[r].len() {
            for (dr, dc) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let r2 = r as i32 + dr;
                let c2 = c as i32 + dc;
                if r2 >= 0 && r2 < p.len() as i32 && c2 >= 0 && c2 < p[r2 as usize].len() as i32 {
                    result[r][c] += p[r2 as usize][c2 as usize] as u64;
                }
            }
        }
    }
    result
}

fn next_bug(v: bool, adj_bugs: u64) -> bool {
    if v {
        adj_bugs == 1
    } else {
        adj_bugs == 1 || adj_bugs == 2
    }
}

fn biodiversity(p: &Planet) -> u64 {
    let mut i = 1;
    let mut result = 0;
    for row in p {
        for v in row {
            result += i * *v as u64;
            i *= 2;
        }
    }
    result
}

pub fn part1(input: &str) -> u64 {
    let mut planet = parse_grid(input);
    let mut s = HashSet::new();
    let mut result = biodiversity(&planet);
    while s.insert(result) {
        let counts = neighbor_counts(&planet);
        for (r, row) in planet.iter_mut().enumerate() {
            for (c, v) in row.iter_mut().enumerate() {
                *v = next_bug(*v, counts[r][c]);
            }
        }
        result = biodiversity(&planet);
    }
    result
}

fn step(planets: &mut VecDeque<Planet>, empty: &Planet) {
    planets.push_front(empty.clone());
    planets.push_back(empty.clone());
    let mut counts = planets
        .iter()
        .map(|p| neighbor_counts(p))
        .collect::<Vec<_>>();
    for i in 1..counts.len() - 1 {
        for (r, row) in counts[i].iter_mut().enumerate() {
            for (c, v) in row.iter_mut().enumerate() {
                if r == 2 && c == 2 {
                    *v = 0;
                } else if r == 1 && c == 2 {
                    for x in 0..5 {
                        *v += planets[i + 1][0][x] as u64;
                    }
                } else if r == 3 && c == 2 {
                    for x in 0..5 {
                        *v += planets[i + 1][4][x] as u64;
                    }
                } else if r == 2 && c == 1 {
                    for y in 0..5 {
                        *v += planets[i + 1][y][0] as u64;
                    }
                } else if r == 2 && c == 3 {
                    for y in 0..5 {
                        *v += planets[i + 1][y][4] as u64;
                    }
                } else {
                    if r == 0 {
                        *v += planets[i - 1][1][2] as u64;
                    } else if r == 4 {
                        *v += planets[i - 1][3][2] as u64;
                    }
                    if c == 0 {
                        *v += planets[i - 1][2][1] as u64;
                    } else if c == 4 {
                        *v += planets[i - 1][2][3] as u64;
                    }
                }
            }
        }
    }

    for (i, planet) in planets.iter_mut().enumerate() {
        for (r, row) in planet.iter_mut().enumerate() {
            for (c, v) in row.iter_mut().enumerate() {
                *v = next_bug(*v, counts[i][r][c]);
            }
        }
    }
}

pub fn part2(input: &str) -> usize {
    let empty = vec![vec![false; 5]; 5];
    let mut planets = vec![empty.clone(), parse_grid(input), empty.clone()]
        .into_iter()
        .collect();
    for _ in 0..200 {
        step(&mut planets, &empty);
    }
    planets
        .into_iter()
        .map(|p| {
            p.into_iter()
                .map(|row| row.into_iter().filter(|&v| v).count())
                .sum::<usize>()
        })
        .sum()
}
