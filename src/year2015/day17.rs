use itertools::Itertools;

fn all_combos(input: &str) -> impl Iterator<Item = Vec<Vec<i32>>> {
    let xs: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    (1..=xs.len()).map(move |n| {
        xs.clone()
            .into_iter()
            .combinations(n)
            .filter(|combo| combo.into_iter().sum::<i32>() == 150)
            .collect()
    })
}

pub fn part1(input: &str) -> usize {
    all_combos(input).map(|v| v.len()).sum()
}

pub fn part2(input: &str) -> Option<usize> {
    all_combos(input).find(|v| !v.is_empty()).map(|v| v.len())
}
