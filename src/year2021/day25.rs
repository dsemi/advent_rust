use impl_ops::*;
use std::ops;

const HEIGHT: usize = 137;

#[derive(PartialEq)]
struct Cucumbers {
    c: [[u64; 4]; HEIGHT],
}

impl ops::Index<usize> for Cucumbers {
    type Output = [u64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.c[index]
    }
}

impl ops::IndexMut<usize> for Cucumbers {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.c[index]
    }
}

macro_rules! impl_operator {
    ($op:tt) => {
        impl_op_ex!($op |a: &Cucumbers, b: &Cucumbers| -> Cucumbers {
            let mut result = Cucumbers {
                c: [[0; 4]; HEIGHT],
            };
            for r in 0..HEIGHT {
                for i in 0..4 {
                    result[r][i] = a[r][i] $op b[r][i];
                }
            }
            result
        });
    }
}

impl_operator!(|);
impl_operator!(&);
impl_operator!(^);

impl Cucumbers {
    fn shift_up(&self) -> Self {
        let mut result = Self {
            c: [[0; 4]; HEIGHT],
        };
        result[HEIGHT - 1] = self[0];
        for r in 1..HEIGHT {
            result[r - 1] = self[r];
        }
        result
    }

    fn shift_down(&self) -> Self {
        let mut result = Self {
            c: [[0; 4]; HEIGHT],
        };
        result[0] = self[HEIGHT - 1];
        for r in 1..HEIGHT {
            result[r] = self[r - 1];
        }
        result
    }

    fn shift_left(&self) -> Self {
        let mut result = Self {
            c: [[0; 4]; HEIGHT],
        };
        for r in 0..HEIGHT {
            result[r][0] = (self[r][0] >> 1) | (self[r][1] << 63);
            result[r][1] = (self[r][1] >> 1) | (self[r][2] << 63);
            result[r][2] = (self[r][2] >> 1) | (self[r][0] << 10);
            result[r][2] &= 0x7ff;
            result[r][3] = 0;
        }
        result
    }

    fn shift_right(&self) -> Self {
        let mut result = Self {
            c: [[0; 4]; HEIGHT],
        };
        for r in 0..HEIGHT {
            result[r][0] = (self[r][0] << 1) | (self[r][2] >> 10);
            result[r][1] = (self[r][1] << 1) | (self[r][0] >> 63);
            result[r][2] = (self[r][2] << 1) | (self[r][1] >> 63);
            result[r][2] &= 0x7ff;
            result[r][3] = 0;
        }
        result
    }
}

fn advance_right(d: &Cucumbers, r: &Cucumbers) -> Cucumbers {
    let result = r.shift_right();
    let blocked = &result & (d | r);
    (result ^ &blocked) | blocked.shift_left()
}

fn advance_down(d: &Cucumbers, r: &Cucumbers) -> Cucumbers {
    let result = d.shift_down();
    let blocked = &result & (d | r);
    (result ^ &blocked) | blocked.shift_up()
}

pub fn part1(input: &str) -> usize {
    fn to_mask(count: usize, inp: &str) -> (u64, u64) {
        let p = inp.as_bytes();
        let mut d_mask = 0;
        let mut r_mask = 0;
        for (i, c) in p.iter().enumerate().take(count) {
            d_mask |= ((*c == b'v') as u64) << i;
            r_mask |= ((*c == b'>') as u64) << i;
        }
        (d_mask, r_mask)
    }

    let mut d = Cucumbers {
        c: [[0; 4]; HEIGHT],
    };
    let mut r = Cucumbers {
        c: [[0; 4]; HEIGHT],
    };
    for ((dr, rr), line) in d.c.iter_mut().zip(r.c.iter_mut()).zip(input.lines()) {
        (dr[0], rr[0]) = to_mask(64, line);
        (dr[1], rr[1]) = to_mask(64, &line[64..]);
        (dr[2], rr[2]) = to_mask(11, &line[128..]);
        (dr[3], rr[3]) = (0, 0);
    }
    for cnt in 1.. {
        let next_r = advance_right(&d, &r);
        let done = next_r == r;
        r = next_r;
        let next_d = advance_down(&d, &r);
        if done && next_d == d {
            return cnt;
        }
        d = next_d;
    }
    unreachable!()
}

pub fn part2(_input: &str) -> String {
    " ".to_string()
}
