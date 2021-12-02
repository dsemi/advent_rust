fn run(input: &str, horz: &mut usize, depth: &mut usize, aim: &mut usize) {
    for line in input.lines() {
        let (cmd, ns) = line.split_once(' ').unwrap();
        let n: usize = ns.parse().unwrap();
        match cmd {
            "forward" => {
                *horz += n;
                *depth += *aim * n
            }
            "down" => *aim += n,
            "up" => *aim -= n,
            _ => panic!("Bad input"),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (mut horz, mut depth, mut aim) = (0, 0, 0);
    run(input, &mut horz, &mut aim, &mut depth);
    horz * depth
}

pub fn part2(input: &str) -> usize {
    let (mut horz, mut depth, mut aim) = (0, 0, 0);
    run(input, &mut horz, &mut depth, &mut aim);
    horz * depth
}
