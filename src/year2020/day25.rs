use ahash::AHashMap;
use mod_exp::mod_exp;

pub fn part1(parts: Vec<i64>) -> Option<i64> {
    let (card, door) = (parts[0], parts[1]);
    let md = 20201227;
    let m = (md as f64).sqrt().ceil() as i64;
    let mut tbl = AHashMap::new();
    let mut n = 1;
    for i in 0..m {
        tbl.insert(n, i);
        n = n * 7 % md;
    }
    let factor = mod_exp(7, md - m - 1, md);
    n = door;
    for i in 0..m {
        if let Some(v) = tbl.get(&n) {
            return Some(mod_exp(card, i * m + v, md));
        }
        n = n * factor % md;
    }
    None
}

pub fn part2(_input: &str) -> String {
    "".to_string()
}
