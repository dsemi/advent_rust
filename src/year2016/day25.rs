use super::assembunny;

pub fn part1(input: &str) -> Option<i64> {
    let ssim = assembunny::parse_instrs(input);
    (0..).find(|i| {
        let mut sim = ssim.clone();
        sim.regs[0] = *i;
        sim.take(10)
            .zip([0, 1].iter().cycle())
            .all(|(a, b)| a == *b)
    })
}

pub fn part2(_: &str) -> String {
    let input: &str = include_str!("../../inputs/2016/bonuschallenge.txt").trim_end();
    let sim = assembunny::parse_instrs(input);
    let output: String = sim.into_iter().map(|x| x as u8 as char).collect();
    super::day08::part2(&output)
}
