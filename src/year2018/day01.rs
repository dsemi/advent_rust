use hashbrown::HashSet;

pub fn part1(ns: Vec<i64>) -> i64 {
    ns.into_iter().sum()
}

pub fn part2(ns: Vec<i64>) -> Option<i64> {
    let mut s = HashSet::new();
    ns.into_iter()
        .cycle()
        .scan(0, |st, x| Some(std::mem::replace(st, *st + x)))
        .find(|&x| !s.insert(x))
}
