use crate::utils::Combinations;
use streaming_iterator::StreamingIterator;

fn quantum_entanglement(n: i64, wts: Vec<i64>) -> Option<i64> {
    let group_size: i64 = wts.iter().copied().sum::<i64>() / n;
    (1..wts.len()).find_map(|i| {
        // Combinations are in order.
        Combinations::new(&wts, i)
            .filter(|combo| combo.iter().copied().sum::<i64>() == group_size)
            .map_deref(|combo| combo.iter().copied().product())
            .next()
    })
}

pub fn part1(input: Vec<i64>) -> Option<i64> {
    quantum_entanglement(3, input)
}

pub fn part2(input: Vec<i64>) -> Option<i64> {
    quantum_entanglement(4, input)
}
