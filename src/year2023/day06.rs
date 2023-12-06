fn nums(i: &str) -> Vec<u64> {
    i.split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect()
}

fn race(time: u64, dist: u64) -> u64 {
    // hold^2 - hold*time + dist = 0
    // use isqrt when that stabilizes.
    let root = (((time * time - 4 * dist) as f64).sqrt()) as u64;
    let mut start = (time - root).div_ceil(2);
    let mut end = (time + root) / 2;
    if (time - start) * start > dist {
        start -= 1;
    }
    if (time - end) * end > dist {
        end += 1;
    }
    end - start - 1
}

pub fn part1(input: &str) -> u64 {
    let (times, dists) = input.split_once('\n').unwrap();
    let times = nums(times);
    let dists = nums(dists);
    times
        .into_iter()
        .zip(dists)
        .map(|(time, win_dist)| race(time, win_dist))
        .product()
}

fn squish(i: &str) -> String {
    i.chars().filter(char::is_ascii_digit).collect()
}

pub fn part2(input: &str) -> u64 {
    let (times, dists) = input.split_once('\n').unwrap();
    let time = squish(times).parse().unwrap();
    let win_dist = squish(dists).parse().unwrap();
    race(time, win_dist)
}
