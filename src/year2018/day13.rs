use crate::utils::Coord;
use crate::year2018::day13::Turn::*;
use ahash::AHashMap;
use genawaiter::stack::{let_gen_using, Co};
use std::collections::hash_map::Entry::Occupied;

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

fn move_cart(cart: &mut Cart, grid: &[Vec<char>]) {
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
    carts: AHashMap<Coord<i32>, Cart>,
}

fn parse_tracks(input: &str) -> Tracks {
    let mut result = Tracks {
        grid: Vec::new(),
        carts: AHashMap::new(),
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
                        pos,
                        dir,
                        turn: Left,
                    },
                );
            }
            result.grid[r].push(v);
        }
    }
    result
}

impl Tracks {
    async fn tick(&mut self, co: Co<'_, (i32, i32)>) {
        while self.carts.len() > 1 {
            let mut ps = self.carts.keys().copied().collect::<Vec<_>>();
            ps.sort_unstable();
            for p in ps {
                if let Some(mut cart) = self.carts.remove(&p) {
                    move_cart(&mut cart, &self.grid);
                    if let Occupied(e) = self.carts.entry(cart.pos) {
                        e.remove();
                        co.yield_((cart.pos.y, cart.pos.x)).await;
                    } else {
                        self.carts.insert(cart.pos, cart);
                    }
                }
            }
        }
        for p in self.carts.drain() {
            co.yield_((p.0.y, p.0.x)).await;
        }
    }
}

pub fn part1(input: &str) -> Option<(i32, i32)> {
    let mut tracks = parse_tracks(input);
    let_gen_using!(gen, |co| tracks.tick(co));
    gen.into_iter().next()
}

pub fn part2(input: &str) -> Option<(i32, i32)> {
    let mut tracks = parse_tracks(input);
    let_gen_using!(gen, |co| tracks.tick(co));
    gen.into_iter().last()
}
