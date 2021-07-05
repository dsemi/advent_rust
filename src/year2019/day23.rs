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

fn parse_network(input: &str) -> Network {
    let p = intcode::new(input);
    let mut computers = Vec::new();
    for i in 0..50 {
        let mut prog = p.clone();
        prog.input.push_back(i);
        computers.push(prog);
    }
    Network {
        computers: computers,
        x: 0,
        y: 0,
    }
}

impl Iterator for Network {
    type Item = Vec<Signal>;

    fn next(&mut self) -> Option<Vec<Signal>> {
        loop {
            let mut result = Vec::new();
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
                        result.push(Signal::ToNat(packet.y));
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
                    result.push(Signal::FromNat(self.y));
                    self.computers[0].input.push_back(self.x);
                    self.computers[0].input.push_back(self.y);
                }
            }
            if !result.is_empty() {
                return Some(result);
            }
        }
    }
}

pub fn part1(input: &str) -> Option<i64> {
    parse_network(input)
        .flatten()
        .filter_map(|p| match p {
            Signal::ToNat(v) => Some(v),
            _ => None,
        })
        .next()
}

pub fn part2(input: &str) -> i64 {
    parse_network(input)
        .flatten()
        .filter_map(|p| match p {
            Signal::FromNat(v) => Some(v),
            _ => None,
        })
        .try_fold(0, |a, b| if a == b { Err(a) } else { Ok(b) })
        .unwrap_err()
}
