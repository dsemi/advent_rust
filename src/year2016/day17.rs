use crate::utils::*;
use crypto::digest::Digest;
use crypto::md5::Md5;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Path {
    pos: Coord<i32>,
    st: String,
}

impl Path {
    fn is_done(&self) -> bool {
        self.pos == Coord::new(4, 4)
    }
}

fn neighbors(path: &Path) -> Vec<Path> {
    if path.is_done() {
        return vec![];
    }
    let mut result = Vec::new();
    let mut md5 = Md5::new();
    md5.input_str(&path.st);
    for (c, d) in md5.result_str().chars().zip("UDLR".chars()) {
        if "bcdef".contains(c) {
            let mut path2 = path.clone();
            match d {
                'U' => path2.pos += Coord::new(0, -1),
                'D' => path2.pos += Coord::new(0, 1),
                'L' => path2.pos += Coord::new(-1, 0),
                'R' => path2.pos += Coord::new(1, 0),
                _ => panic!("Bad state"),
            }
            path2.st.push(d);
            if path2.pos.x > 0 && path2.pos.x <= 4 && path2.pos.y > 0 && path2.pos.y <= 4 {
                result.push(path2);
            }
        }
    }
    result
}

pub fn part1(input: &str) -> Option<String> {
    bfs(
        Path {
            pos: Coord::new(1, 1),
            st: input.to_string(),
        },
        neighbors,
    )
    .filter_map(|(_, p)| p.is_done().then(|| p.st[input.len()..].to_string()))
    .next()
}

pub fn part2(input: &str) -> Option<usize> {
    bfs(
        Path {
            pos: Coord::new(1, 1),
            st: input.to_string(),
        },
        neighbors,
    )
    .filter_map(|(d, p)| p.is_done().then(|| d))
    .max()
}
