use crate::utils::parsers::*;
use hashbrown::HashMap;
use phf::{Map, phf_map};
use std::iter::{once, repeat_n};

type Cache = HashMap<(Vec<u8>, usize), usize>;
type Pt = (usize, usize);

const KEYPAD: Map<u8, Pt> = phf_map! {
    b'7' => (0, 0),
    b'8' => (0, 1),
    b'9' => (0, 2),
    b'4' => (1, 0),
    b'5' => (1, 1),
    b'6' => (1, 2),
    b'1' => (2, 0),
    b'2' => (2, 1),
    b'3' => (2, 2),
    b' ' => (3, 0),
    b'0' => (3, 1),
    b'A' => (3, 2),
};

const REMOTE: Map<u8, Pt> = phf_map! {
    b' ' => (0, 0),
    b'^' => (0, 1),
    b'A' => (0, 2),
    b'<' => (1, 0),
    b'v' => (1, 1),
    b'>' => (1, 2),
};

fn dfs(cache: &mut Cache, buttons: &[u8], depth: usize, limit: usize) -> usize {
    if depth == limit {
        return buttons.len();
    }

    let key = (buttons.to_vec(), depth);
    if let Some(&previous) = cache.get(&key) {
        return previous;
    }

    let pad = if depth == 0 { KEYPAD } else { REMOTE };
    let mut shortest = usize::MAX;
    path(&mut vec![], &pad, buttons, pad[&b'A'], &mut |path| {
        let presses = path
            .split_inclusive(|&b| b == b'A')
            .map(|chunk| dfs(cache, chunk, depth + 1, limit))
            .sum();
        shortest = shortest.min(presses);
    });
    cache.insert(key, shortest);
    shortest
}

fn path<F: FnMut(&[u8])>(curr: &mut Vec<u8>, pad: &Map<u8, Pt>, keys: &[u8], from: Pt, f: &mut F) {
    if keys.is_empty() {
        f(curr);
        return;
    }

    let to = pad[&keys[0]];

    let dr = from.0.abs_diff(to.0);
    const UD: [u8; 2] = [b'^', b'v'];
    let rv = UD[(to.0 > from.0) as usize];

    let dc = from.1.abs_diff(to.1);
    const LR: [u8; 2] = [b'<', b'>'];
    let cv = LR[(to.1 > from.1) as usize];

    if (from.0, to.1) != pad[&b' '] {
        curr.extend(repeat_n(cv, dc).chain(repeat_n(rv, dr)).chain(once(b'A')));
        path(curr, pad, &keys[1..], to, f);
        curr.truncate(curr.len() - dr - dc - 1);
    }
    if (to.0, from.1) != pad[&b' '] {
        curr.extend(repeat_n(rv, dr).chain(repeat_n(cv, dc)).chain(once(b'A')));
        path(curr, pad, &keys[1..], to, f);
        curr.truncate(curr.len() - dr - dc - 1);
    }
}

pub fn part1(input: &str) -> usize {
    let cache = &mut HashMap::new();
    input.lines().map(|c| dfs(cache, c.as_bytes(), 0, 3) * c[..c.len() - 1].usize()).sum()
}

pub fn part2(input: &str) -> usize {
    let cache = &mut HashMap::new();
    input.lines().map(|c| dfs(cache, c.as_bytes(), 0, 26) * c[..c.len() - 1].usize()).sum()
}
