use gcd::Gcd;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::collections::HashMap;

type Coord = (i32, i32);

fn parse_coords(input: &str) -> Vec<Coord> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, v)| v == '#')
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn dist(a: Coord, b: Coord) -> i32 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

fn theta(a: Coord, b: Coord) -> Coord {
    let (x, y) = (b.0 - a.0, b.1 - a.1);
    let gcd = (x.abs() as u32).gcd(y.abs() as u32) as i32;
    (x / gcd, y / gcd)
}

fn cmp(a: &Coord, b: &Coord) -> Ordering {
    if a.0 >= 0 && b.0 < 0 {
        return Less;
    } else if a.0 < 0 && b.0 >= 0 {
        return Greater;
    } else if a.0 == 0 && b.0 == 0 {
        return a.1.cmp(&b.1);
    }

    let det = a.0 * (-b.1) - (-a.1) * b.0;
    det.cmp(&0)
}

fn visibilities(pt: &Coord, pts: &Vec<Coord>) -> Vec<Vec<Coord>> {
    let mut m: HashMap<Coord, Vec<Coord>> = HashMap::new();
    for p in pts.iter() {
        if p != pt {
            let e = m.entry(theta(*pt, *p)).or_insert(vec![]);
            (*e).push(*p);
        }
    }
    let mut vec: Vec<Coord> = m.keys().copied().collect();
    vec.sort_by(cmp);
    vec.into_iter()
        .map(|k| {
            let mut v = m[&k].clone();
            v.sort_by_key(|x| dist(*pt, *x));
            v
        })
        .collect()
}

fn max_detected(asts: Vec<Coord>) -> Vec<Vec<Coord>> {
    asts.iter()
        .map(|ast| visibilities(ast, &asts))
        .max_by_key(|x| x.len())
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    max_detected(parse_coords(input)).len()
}

pub fn part2(input: &str) -> i32 {
    let mut m: Vec<std::vec::IntoIter<Coord>> = max_detected(parse_coords(input))
        .into_iter()
        .map(|x| x.into_iter())
        .collect();
    let mut i = 0;
    loop {
        for pts in m.iter_mut() {
            match pts.next() {
                Some((a, b)) => {
                    i += 1;
                    if i == 200 {
                        return 100 * a + b;
                    }
                }
                None => continue,
            }
        }
    }
}
