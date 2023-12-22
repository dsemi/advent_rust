use crate::utils::C;
use ahash::AHashMap;
use genawaiter::stack::{let_gen_using, Co};
use std::collections::hash_map::Entry::Occupied;
use Turn::*;

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

struct Cart {
    pos: C<i32>,
    dir: C<i32>,
    turn: Turn,
}

fn turn(c: C<i32>, turn: Turn) -> C<i32> {
    match turn {
        Left => c * C(0, 1),
        Straight => c,
        Right => c * C(0, -1),
    }
}

fn move_cart(cart: &mut Cart, grid: &[Vec<char>]) {
    cart.pos += cart.dir;
    match grid[cart.pos.0 as usize][cart.pos.1 as usize] {
        '\\' => cart.dir = C(cart.dir.1, cart.dir.0),
        '/' => cart.dir = C(-cart.dir.1, -cart.dir.0),
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
    carts: AHashMap<C<i32>, Cart>,
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
                let pos = C(r as i32, c as i32);
                let dir = match v {
                    '^' => C(-1, 0),
                    '>' => C(0, 1),
                    'v' => C(1, 0),
                    '<' => C(0, -1),
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
                        co.yield_((cart.pos.1, cart.pos.0)).await;
                    } else {
                        self.carts.insert(cart.pos, cart);
                    }
                }
            }
        }
        for p in self.carts.drain() {
            co.yield_((p.0 .1, p.0 .0)).await;
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
