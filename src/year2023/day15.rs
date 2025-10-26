fn hash(input: &str) -> usize {
    input.bytes().fold(0, |v, b| 17 * (v + b as usize) % 256)
}

pub fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

pub fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    for instr in input.split(',') {
        if let Some((label, n)) = instr.split_once('=') {
            let h = hash(label);
            if let Some(e) = boxes[h].iter_mut().find(|(k, _)| *k == label) {
                e.1 = n.parse().unwrap();
            } else {
                boxes[h].push((label, n.parse().unwrap()));
            }
        } else {
            let label = &instr[0..instr.len() - 1];
            let h = hash(label);
            if let Some(i) = boxes[h].iter().position(|(k, _)| *k == label) {
                boxes[h].remove(i);
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(n, b)| {
            b.into_iter().enumerate().map(move |(s, (_, len))| (n + 1) * (s + 1) * len)
        })
        .sum()
}
