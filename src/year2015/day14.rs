use crate::utils::*;

fn dists_at_each_second(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            let (speed, fly_time, rest_time) = scan_fmt!(
                line,
                "{*} can fly {} km/s for {} seconds, but then must rest for {} seconds.",
                i32,
                i32,
                i32
            )
            .unwrap();
            vec![speed; fly_time as usize]
                .into_iter()
                .chain(vec![0; rest_time as usize].into_iter())
                .cycle()
                .scan(0, |state, x| {
                    *state += x;
                    Some(*state)
                })
                .take(2503)
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> Option<i32> {
    dists_at_each_second(input)
        .into_iter()
        .map(|x| *x.last().unwrap())
        .max()
}

pub fn part2(input: &str) -> Option<i32> {
    let v = transpose(&dists_at_each_second(input))
        .into_iter()
        .map(|v| {
            let m = *v.iter().max().unwrap();
            v.into_iter()
                .map(move |x| if x == m { 1 } else { 0 })
                .collect()
        })
        .collect();
    transpose(&v).into_iter().map(|x| x.iter().sum()).max()
}
