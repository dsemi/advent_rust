use itertools::iterate;

fn fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn sum_module_fuels(f: fn(i64) -> i64, input: &str) -> i64 {
    input.lines().map(|line| f(line.parse().unwrap())).sum()
}

pub fn part1(input: &str) -> i64 {
    sum_module_fuels(fuel, input)
}

pub fn part2(input: &str) -> i64 {
    sum_module_fuels(
        |x| {
            iterate(x, |&v| fuel(v))
                .skip(1)
                .take_while(|&v| v > 0)
                .sum()
        },
        input,
    )
}
