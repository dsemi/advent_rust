fn process(input: &str) -> (u32, u64) {
    let (line, input) = input.split_once('\n').unwrap();
    let mut splits = 0;
    let mut top: Vec<_> = line.bytes().map(|b| u64::from(b == b'S')).collect();
    input.lines().for_each(|line| {
        line.bytes().enumerate().filter(|&(_, b)| b == b'^').for_each(|(i, _)| {
            top[i - 1] += top[i];
            top[i + 1] += top[i];
            splits += u32::from(std::mem::take(&mut top[i]) > 0);
        })
    });
    (splits, top.into_iter().sum())
}

pub fn part1(input: &str) -> u32 {
    process(input).0
}

pub fn part2(input: &str) -> u64 {
    process(input).1
}
