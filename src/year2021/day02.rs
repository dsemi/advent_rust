fn run(input: &str) -> (i32, i32, i32) {
    let (mut horz, mut depth, mut aim) = (0, 0, 0);
    for line in input.lines() {
        let (cmd, ns) = line.split_once(' ').unwrap();
        let n: i32 = ns.parse().unwrap();
        match cmd {
            "forward" => {
                horz += n;
                depth += aim * n
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => panic!("Bad input"),
        }
    }
    (horz, depth, aim)
}

pub fn part1(input: &str) -> i32 {
    let (horz, _, depth) = run(input);
    horz * depth
}

pub fn part2(input: &str) -> i32 {
    let (horz, depth, _) = run(input);
    horz * depth
}
