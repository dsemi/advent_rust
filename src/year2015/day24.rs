use itertools::Itertools;

fn quantum_entanglement(n: i64, s: &str) -> Option<i64> {
    let wts: Vec<i64> = s.lines().map(|x| x.parse().unwrap()).collect();
    let group_size: i64 = wts.iter().map(|x| *x).sum::<i64>() / n;
    let mut i = 1;
    loop {
        let qes: Vec<i64> = wts.iter()
            .combinations(i)
            .filter(|combo| combo.iter().map(|x| *x).sum::<i64>() == group_size)
            .map(|combo| combo.into_iter().product())
            .collect();
        if !qes.is_empty() {
            return qes.into_iter().min();
        }
        i += 1;
    }
}

pub fn part1(input: &str) -> Option<i64> {
    quantum_entanglement(3, input)
}

pub fn part2(input: &str) -> Option<i64> {
    quantum_entanglement(4, input)
}
