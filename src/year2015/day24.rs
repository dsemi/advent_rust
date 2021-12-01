use streaming_iterator::StreamingIterator;

use crate::utils::Combinations;

fn quantum_entanglement(n: i64, wts: Vec<i64>) -> i64 {
    let group_size: i64 = wts.iter().copied().sum::<i64>() / n;
    let mut i = 1;
    loop {
        let min_qe = Combinations::new(&wts, i)
            .filter(|combo| combo.iter().copied().sum::<i64>() == group_size)
            .map_deref(|combo| combo.iter().copied().product())
            .min();
        if let Some(m) = min_qe {
            return m;
        }
        i += 1;
    }
}

pub fn part1(input: Vec<i64>) -> i64 {
    quantum_entanglement(3, input)
}

pub fn part2(input: Vec<i64>) -> i64 {
    quantum_entanglement(4, input)
}
