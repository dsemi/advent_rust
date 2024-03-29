use crate::utils::*;
use md5::{Digest, Md5};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Path {
    pos: C<i32>,
    st: String,
}

impl Path {
    fn is_done(&self) -> bool {
        self.pos == C(4, 4)
    }
}

fn neighbors(path: &Path) -> Vec<Path> {
    if path.is_done() {
        return vec![];
    }
    let mut result = Vec::new();
    let mut md5 = Md5::new();
    md5.update(&path.st);
    for (c, d) in hex::encode(md5.finalize()).chars().zip("UDLR".chars()) {
        if "bcdef".contains(c) {
            let mut path2 = path.clone();
            match d {
                'U' => path2.pos += C(0, -1),
                'D' => path2.pos += C(0, 1),
                'L' => path2.pos += C(-1, 0),
                'R' => path2.pos += C(1, 0),
                _ => panic!("Bad state"),
            }
            path2.st.push(d);
            if path2.pos.0 > 0 && path2.pos.0 <= 4 && path2.pos.1 > 0 && path2.pos.1 <= 4 {
                result.push(path2);
            }
        }
    }
    result
}

pub fn part1(input: &str) -> Option<String> {
    bfs(
        Path {
            pos: C(1, 1),
            st: input.to_string(),
        },
        neighbors,
    )
    .find_map(|(_, p)| p.is_done().then(|| p.st[input.len()..].to_string()))
}

pub fn part2(input: &str) -> Option<usize> {
    bfs(
        Path {
            pos: C(1, 1),
            st: input.to_string(),
        },
        neighbors,
    )
    .filter_map(|(d, p)| p.is_done().then_some(d))
    .max()
}
