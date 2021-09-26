use genawaiter::stack::{let_gen_using, Co};

use crate::year2019::intcode;

struct Packet {
    address: i64,
    x: i64,
    y: i64,
}

enum Signal {
    ToNat(i64),
    FromNat(i64),
}

struct Network {
    computers: Vec<intcode::Program>,
    x: i64,
    y: i64,
}

impl Network {
    fn new(input: &str) -> Self {
        let p = intcode::new(input);
        let mut computers = Vec::new();
        for i in 0..50 {
            let mut prog = p.clone();
            prog.input.push_back(i);
            computers.push(prog);
        }
        Self {
            computers,
            x: 0,
            y: 0,
        }
    }

    async fn run(&mut self, co: Co<'_, Signal>) {
        loop {
            let packets = self
                .computers
                .iter_mut()
                .filter_map(|comp| {
                    comp.run();
                    comp.recv(3).map(|ns| Packet {
                        address: ns[0],
                        x: ns[1],
                        y: ns[2],
                    })
                })
                .collect::<Vec<_>>();
            if !packets.is_empty() {
                for packet in packets {
                    if packet.address == 255 {
                        co.yield_(Signal::ToNat(packet.y)).await;
                        self.x = packet.x;
                        self.y = packet.y;
                    } else {
                        self.computers[packet.address as usize]
                            .input
                            .push_back(packet.x);
                        self.computers[packet.address as usize]
                            .input
                            .push_back(packet.y);
                    }
                }
            } else {
                let mut all_inp = true;
                for comp in self.computers.iter_mut() {
                    comp.input.push_back(-1);
                    comp.run();
                    all_inp = all_inp && comp.output.len() < 3;
                }
                if all_inp {
                    co.yield_(Signal::FromNat(self.y)).await;
                    self.computers[0].input.push_back(self.x);
                    self.computers[0].input.push_back(self.y);
                }
            }
        }
    }
}

pub fn part1(input: &str) -> Option<i64> {
    let mut network = Network::new(input);
    let_gen_using!(gen, |co| network.run(co));
    gen.into_iter()
        .filter_map(|p| match p {
            Signal::ToNat(v) => Some(v),
            _ => None,
        })
        .next()
}

pub fn part2(input: &str) -> i64 {
    let mut network = Network::new(input);
    let_gen_using!(gen, |co| network.run(co));
    gen.into_iter()
        .filter_map(|p| match p {
            Signal::FromNat(v) => Some(v),
            _ => None,
        })
        .try_fold(0, |a, b| if a == b { Err(a) } else { Ok(b) })
        .unwrap_err()
}
