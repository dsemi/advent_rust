use itertools::Itertools;
use std::collections::HashMap;

use crate::utils::Coord;
use crate::year2018::day13::Turn::*;

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

struct Cart {
    pos: Coord<i32>,
    dir: Coord<i32>,
    turn: Turn,
}

fn turn(c: Coord<i32>, turn: Turn) -> Coord<i32> {
    match turn {
        Left => c * Coord::new(0, 1),
        Straight => c,
        Right => c * Coord::new(0, -1),
    }
}

fn move_cart(cart: &mut Cart, grid: &Vec<Vec<char>>) {
    cart.pos += cart.dir;
    match grid[cart.pos.x as usize][cart.pos.y as usize] {
        '\\' => cart.dir = Coord::new(cart.dir.y, cart.dir.x),
        '/' => cart.dir = Coord::new(-cart.dir.y, -cart.dir.x),
        '+' => {
            cart.dir = turn(cart.dir, cart.turn);
            cart.turn = match cart.turn {
                Left => Straight,
                Straight => Right,
                Right => Left,
            };
        }
        '-' | '|' | '<' | '>' | 'v' | '^' => (),
        x => panic!("Invalid position: {}", x),
    }
}

struct Tracks {
    grid: Vec<Vec<char>>,
    carts: HashMap<Coord<i32>, Cart>,
}

fn parse_tracks(input: &str) -> Tracks {
    let mut result = Tracks {
        grid: Vec::new(),
        carts: HashMap::new(),
    };
    for (r, line) in input.lines().enumerate() {
        result.grid.push(Vec::new());
        for (c, v) in line.chars().enumerate() {
            if "^>v<".contains(v) {
                let pos = Coord::new(r as i32, c as i32);
                let dir = match v {
                    '^' => Coord::new(-1, 0),
                    '>' => Coord::new(0, 1),
                    'v' => Coord::new(1, 0),
                    '<' => Coord::new(0, -1),
                    _ => panic!("Invalid direction: {}", v),
                };
                result.carts.insert(
                    pos,
                    Cart {
                        pos: pos,
                        dir: dir,
                        turn: Left,
                    },
                );
            }
            result.grid[r].push(v);
        }
    }
    result
}

impl Iterator for Tracks {
    type Item = Vec<(i32, i32)>;

    fn next(&mut self) -> Option<Vec<(i32, i32)>> {
        while self.carts.len() > 1 {
            let poss = self.carts.keys().sorted().copied().collect::<Vec<_>>();
            let v = poss
                .into_iter()
                .filter_map(|p| {
                    self.carts.remove(&p).and_then(|mut cart| {
                        move_cart(&mut cart, &self.grid);
                        if self.carts.contains_key(&cart.pos) {
                            self.carts.remove(&cart.pos);
                            Some((cart.pos.y, cart.pos.x))
                        } else {
                            self.carts.insert(cart.pos, cart);
                            None
                        }
                    })
                })
                .collect::<Vec<_>>();
            if !v.is_empty() {
                return Some(v);
            }
        }
        assert!(self.carts.len() <= 1);
        let v = self
            .carts
            .drain()
            .map(|x| (x.0.y, x.0.x))
            .collect::<Vec<_>>();
        (!v.is_empty()).then(|| v)
    }
}

pub fn part1(input: &str) -> Option<(i32, i32)> {
    parse_tracks(input).flatten().next()
}

pub fn part2(input: &str) -> Option<(i32, i32)> {
    parse_tracks(input).flatten().last()
}
