use fancy_regex::{Match, Regex};
use std::collections::HashMap;

use crate::utils::*;

fn make_portal(m: Match) -> (usize, String) {
    if m.as_str().starts_with('.') {
        (m.start(), m.as_str()[1..].to_string())
    } else {
        (m.end() - 1, m.as_str()[..m.as_str().len() - 1].to_string())
    }
}

fn parse_maze(
    input: &str,
) -> (
    Vec<Vec<bool>>,
    Coord<i32>,
    Coord<i32>,
    HashMap<Coord<i32>, (Coord<i32>, i32)>,
) {
    let rows = input.lines().collect::<Vec<_>>();
    let mut grid = vec![vec![false; rows[0].len()]; rows.len()];
    for (r, row) in rows.iter().enumerate() {
        for (c, v) in row.chars().enumerate() {
            grid[r][c] = v == '.';
        }
    }

    let outreg = Regex::new(r"^[A-Z]{2}\.|\.[A-Z]{2}$").unwrap();
    let innreg = Regex::new(r"(?<!^)[A-Z]{2}\.|\.[A-Z]{2}(?!$)").unwrap();

    let mut outer = HashMap::new();
    let mut inner = HashMap::new();

    for (r, row) in rows.iter().enumerate() {
        for m in outreg.find_iter(row) {
            let (c, s) = make_portal(m.unwrap());
            outer.insert(s, Coord::new(r as i32, c as i32));
        }
        for m in innreg.find_iter(row) {
            let (c, s) = make_portal(m.unwrap());
            inner.insert(s, Coord::new(r as i32, c as i32));
        }
    }
    for (c, col) in transpose(&rows.into_iter().map(|row| row.chars().collect()).collect())
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .enumerate()
    {
        for m in outreg.find_iter(&col) {
            let (r, s) = make_portal(m.unwrap());
            outer.insert(s, Coord::new(r as i32, c as i32));
        }
        for m in innreg.find_iter(&col) {
            let (r, s) = make_portal(m.unwrap());
            inner.insert(s, Coord::new(r as i32, c as i32));
        }
    }

    let mut portals = HashMap::new();
    for (k, v1) in outer.iter() {
        if let Some(v2) = inner.get(k) {
            portals.insert(*v1, (*v2, -1));
            portals.insert(*v2, (*v1, 1));
        }
    }
    (grid, outer["AA"], outer["ZZ"], portals)
}

pub fn part1(input: &str) -> Option<usize> {
    let (grid, start, finish, portals) = parse_maze(input);
    fn neighbors(
        grid: &Vec<Vec<bool>>,
        portals: &HashMap<Coord<i32>, (Coord<i32>, i32)>,
        st: &Coord<i32>,
    ) -> Vec<Coord<i32>> {
        let mut result = Vec::new();
        if let Some(v) = portals.get(st) {
            result.push(v.0);
        }
        result.extend(
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter_map(|d| {
                    let st2 = st + &Coord::new(d.0, d.1);
                    grid[st2.x as usize][st2.y as usize].then(|| st2)
                }),
        );
        result
    }
    let x = bfs(start, |x| neighbors(&grid, &portals, x))
        .filter_map(|(d, st)| (st == finish).then(|| d))
        .next();
    x
}

pub fn part2(input: &str) -> Option<usize> {
    let (grid, start, finish, portals) = parse_maze(input);
    fn neighbors(
        grid: &Vec<Vec<bool>>,
        portals: &HashMap<Coord<i32>, (Coord<i32>, i32)>,
        st: &(Coord<i32>, i32),
    ) -> Vec<(Coord<i32>, i32)> {
        let mut result = Vec::new();
        if let Some((st2, d)) = portals.get(&st.0) {
            if d + st.1 >= 0 {
                result.push((*st2, d + st.1));
            }
        }
        result.extend(
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter_map(|d| {
                    let st2 = (st.0 + Coord::new(d.0, d.1), st.1);
                    grid[st2.0.x as usize][st2.0.y as usize].then(|| st2)
                }),
        );
        result
    }
    let x = bfs((start, 0), |x| neighbors(&grid, &portals, x))
        .filter_map(|(d, st)| (st == (finish, 0)).then(|| d))
        .next();
    x
}
