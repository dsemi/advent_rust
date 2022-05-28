use crate::utils::bits;
use std::cmp::Ordering::*;
use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

const ROOM_XOR: u32 = 0xffaa5500;
const COST: [u16; 4] = [1, 10, 100, 1000];

const SAFE_SKIPS: &[u32] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x3, 0x7, 0x7, 0x7, 0x7, 0, 0, 0, 0x3, 0x707, 0xf0f, 0xf0f,
    0xf0f, 0, 0, 0, 0x3, 0x707, 0xf0f0f, 0x1f1f1f, 0x1f1f1f, 0,
];

fn read_input(input: &str) -> u32 {
    let mut b = input.as_bytes();
    fn to_glyph(c: u8) -> u32 {
        (c - b'A') as u32
    }
    let mut room = 0;
    for i in 0..4 {
        room |= to_glyph(b[31]) << (8 * i);
        room |= to_glyph(b[45]) << ((8 * i) + 2);
        b = &b[2..];
    }
    room ^ (ROOM_XOR & 0x0f0f0f0f)
}

fn insert_part2(mut room: u32) -> u32 {
    room = (room & 0x03030303) | ((room << 4) & 0xc0c0c0c0);
    room ^ 0x1c2c0c3c
}

fn base_cost(mut room: u32) -> (i32, i32) {
    room ^= ROOM_XOR;
    fn move_cost(dist: i32) -> i32 {
        2 * max(1, dist.abs())
    }
    let mut base = 0;
    let mut second_row = 0;
    for i in 0..4 {
        let glyph0 = (room & 3) as i32;
        let glyph1 = ((room >> 2) & 3) as i32;
        let cost0 = COST[glyph0 as usize] as i32;
        let cost1 = COST[glyph1 as usize] as i32;
        base += cost0 * (move_cost(i - glyph0) + 1);
        base += cost1 * move_cost(i - glyph1);
        second_row += cost1;
        room >>= 8;
    }
    let cost1 = base + second_row * 2 + 3333;
    let cost2 = base + second_row * 4 + 29115;
    (cost1, cost2)
}

#[derive(Clone)]
struct Room {
    room: u32,
}

impl Room {
    fn empty(&self, r: i32) -> bool {
        (self.room >> (8 * r)) & 0xff == 0
    }

    fn get(&self, r: i32) -> i32 {
        r ^ ((self.room >> (8 * r)) & 3) as i32
    }

    fn pop(&mut self, r: i32) {
        let mask1 = 0xff << (8 * r);
        let mask2 = 0x3f << (8 * r);
        self.room = ((self.room >> 2) & mask2) | (self.room & !mask1);
    }
}

#[derive(Clone)]
struct Hall {
    hall: u32,
}

impl Hall {
    fn empty(&self, h: i32) -> bool {
        self.hall & (4 << (4 * h)) == 0
    }

    fn clear(&mut self, h: i32) {
        self.hall &= !(0xf << (4 * h));
    }

    fn set(&mut self, h: i32, g: i32) {
        self.hall |= (4 | g as u32) << (4 * h);
    }

    fn get(&self, h: i32) -> i32 {
        ((self.hall >> (4 * h)) & 3) as i32
    }

    fn mask(&self) -> u32 {
        self.hall & 0x4444444
    }
}

#[derive(Clone)]
struct State {
    room: Room,
    hall: Hall,
}

impl State {
    fn new(hash: u64) -> Self {
        Self {
            room: Room { room: hash as u32 },
            hall: Hall {
                hall: (hash >> 32) as u32,
            },
        }
    }

    fn hash(&self) -> u64 {
        ((self.hall.hall as u64) << 32) | self.room.room as u64
    }

    fn solved(&self) -> bool {
        self.room.room | self.hall.hall == 0
    }

    fn room_l(r: i32) -> i32 {
        r + 1
    }

    fn room_r(r: i32) -> i32 {
        r + 2
    }

    fn obstructed(&self, r: i32, h: i32) -> bool {
        let (lo, hi) = if h <= State::room_l(r) {
            (h + 1, State::room_l(r))
        } else {
            (State::room_r(r), h - 1)
        };
        let mask = (16 << (4 * hi)) - (1 << (4 * lo));
        self.hall.hall & mask != 0
    }

    fn force_one(&mut self) -> bool {
        for b in bits(self.hall.mask()) {
            let h = (b / 4) as i32;
            let r = self.hall.get(h);
            if self.room.empty(r) && !self.obstructed(r as i32, h) {
                self.hall.clear(h);
                return true;
            }
        }
        for r in 0..4 {
            if self.room.empty(r) {
                continue;
            }
            let g = self.room.get(r) as i32;
            if g == r || !self.room.empty(g) {
                continue;
            }
            if !self.obstructed(
                r,
                if r < g {
                    State::room_r(g)
                } else {
                    State::room_l(g)
                },
            ) {
                self.room.pop(r);
                return true;
            }
        }
        false
    }

    fn deadlocked(&self) -> bool {
        let h43 = self.hall.hall & 0x0077000;
        if h43 == 0x0047000 || h43 == 0x0057000 {
            return true;
        }
        let h42 = self.hall.hall & 0x0070700;
        if h42 == 0x0040700 {
            return true;
        }
        let h32 = self.hall.hall & 0x0007700;
        if h32 == 0x0004600 || h32 == 0x0004700 {
            return true;
        }
        false
    }

    fn crowded(&self) -> bool {
        let mut h0 = 0;
        let mut h = (self.hall.hall >> 2) | 0x10000000;
        let mut satisfied = false;
        for i in 0..8 {
            if h & 1 != 0 {
                if h0 < i {
                    let r0 = max(0, h0 - 2);
                    let r1 = max(3, i - 2);
                    let space = i - h0;
                    let mask = 3 << (2 * space);
                    for r in r0..=r1 {
                        let rr = (self.room.room >> (8 * r)) & 0xff;
                        if rr & mask == 0 {
                            satisfied = true;
                        }
                    }
                }
                h0 = i + 1;
            }
            h >>= 4;
        }
        !satisfied
    }

    fn neighbors(&self, skip: u32) -> Vec<(i32, Self, u32)> {
        let mut ns = vec![];
        let mut skip_rooms = 0;
        for i in 0..3 {
            let h = i + 2;
            if !self.hall.empty(h) {
                let mask = 0b1110 << i;
                skip_rooms |= if i < self.hall.get(h) { !mask } else { mask };
            }
        }
        for r in 0..4 {
            if skip_rooms & (1 << r) != 0 || self.room.empty(r) {
                continue;
            }
            let g = self.room.get(r);
            let (lo, hi) = match r.cmp(&g) {
                Less => (State::room_r(r), State::room_l(g)),
                Greater => (State::room_r(g), State::room_l(r)),
                Equal => (State::room_l(r), State::room_r(r)),
            };
            for h in 0..7 {
                if r != g && h >= lo && h <= hi {
                    continue;
                }
                let skip_idx = 8 * r + h;
                if (skip >> skip_idx) & 1 != 0 {
                    continue;
                }
                if !self.hall.empty(h) || self.obstructed(r, h) {
                    continue;
                }
                let mut cost = if h < lo {
                    lo - h
                } else if hi < h {
                    h - hi
                } else {
                    0
                };
                cost *= 2;
                cost -= (((cost == 0) as i32 | (r == g) as i32) == 0) as i32
                    + ((h == 0) as i32 | (h == 6) as i32);
                cost *= 2;
                let mut n = self.clone();
                n.room.pop(r);
                n.hall.set(h, g);
                if n.deadlocked() {
                    continue;
                }
                let mut skips = SAFE_SKIPS[skip_idx as usize];
                while n.force_one() {
                    skips = 0;
                }
                if n.crowded() {
                    continue;
                }
                ns.push((cost * COST[g as usize] as i32, n, skips));
            }
        }
        ns
    }
}

struct Hash<const SIZE: usize, T> {
    table: [(u64, T); SIZE],
}

impl<const SIZE: usize, T> Hash<SIZE, T> {
    fn find(&self, key: u64) -> i32 {
        let mut idx = (key as usize % SIZE) as i32;
        while self.table[idx as usize].0 != 0 && self.table[idx as usize].0 != !key {
            idx += 1;
            idx &= -((idx < SIZE as i32) as i32);
        }
        idx
    }

    fn insert(&mut self, key: u64, value: T) {
        let idx = self.find(key);
        self.table[idx as usize] = (!key, value);
    }

    fn get(&mut self, idx: i32) -> &mut T {
        &mut self.table[idx as usize].1
    }

    fn exists(&self, idx: i32) -> bool {
        self.table[idx as usize].0 != 0
    }
}

fn solve(start: State) -> i32 {
    const TBL_SIZE: usize = 14983;
    let mut cost = Hash::<TBL_SIZE, (u16, u32)> {
        table: [(0, (0, 0)); TBL_SIZE],
    };
    cost.insert(start.hash(), (0, 0));
    let mut q = BinaryHeap::<Reverse<(i32, u64)>>::new();
    q.push(Reverse((0, start.hash())));

    while let Some(Reverse((queue_cost, cur_hash))) = q.pop() {
        let (cur_cost, cur_skips) = *cost.get(cost.find(cur_hash));
        if queue_cost != cur_cost as i32 {
            continue;
        }
        let cur = State::new(cur_hash);
        if cur.solved() {
            break;
        }
        for (delta, state, skips) in cur.neighbors(cur_skips) {
            let hash = state.hash();
            let new_cost = cur_cost as i32 + delta;
            let new_idx = cost.find(hash);
            if !cost.exists(new_idx) {
                cost.insert(hash, (new_cost as u16, skips));
                q.push(Reverse((new_cost, hash)));
            } else {
                let (prev_cost, prev_skips) = cost.get(new_idx);
                match new_cost.cmp(&(*prev_cost as i32)) {
                    Equal => *prev_skips &= skips,
                    Less => {
                        *prev_cost = new_cost as u16;
                        *prev_skips = skips;
                        q.push(Reverse((new_cost, hash)));
                    }
                    Greater => (),
                }
            }
        }
    }

    cost.get(cost.find(0)).0 as i32
}

pub fn part1(input: &str) -> i32 {
    let room = read_input(input);
    let base = base_cost(room).0;
    base + solve(State::new(room as u64))
}

pub fn part2(input: &str) -> i32 {
    let room = read_input(input);
    let base = base_cost(room).1;
    base + solve(State::new(insert_part2(room) as u64))
}
