use itertools::Itertools;

fn quantum_entanglement(n: i64, s: &str) -> i64 {
    let wts: Vec<i64> = s.lines().map(|x| x.parse().unwrap()).collect();
    let group_size: i64 = wts.iter().copied().sum::<i64>() / n;
    let mut i = 1;
    loop {
        let min_qe = wts.iter()
            .combinations(i)
            .filter(|combo| combo.iter().copied().sum::<i64>() == group_size)
            .map(|combo| combo.into_iter().product())
            .min();
        if let Some(m) = min_qe {
            return m;
        }
        i += 1;
    }
}

pub fn part1(input: &str) -> i64 {
    quantum_entanglement(3, input)
}

pub fn part2(input: &str) -> i64 {
    quantum_entanglement(4, input)
}
