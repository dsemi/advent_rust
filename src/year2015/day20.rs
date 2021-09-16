use std::cmp::min;

const PRIMES: &[u64] = &[2, 3, 5, 7, 11, 13];

fn solve(goal: u64, prime_index: i32) -> u64 {
    if prime_index < 0 {
        return goal;
    }

    let p = PRIMES[prime_index as usize];
    let mut p_power = 1;
    let mut p_sum = 1;
    let mut best = solve(goal, prime_index - 1);
    while p_sum < goal {
        p_power *= p;
        p_sum += p_power;

        let subgoal = (goal + p_sum - 1) / p_sum;
        best = min(best, p_power * solve(subgoal, prime_index - 1));
    }
    best
}

pub fn part1(input: &str) -> u64 {
    let n: u64 = input.parse().unwrap();
    solve(n / 10, PRIMES.len() as i32 - 1)
}

pub fn part2(input: &str) -> Option<usize> {
    let n: u32 = input.parse().unwrap();
    let mut vec = vec![0; 1000000];
    for i in 1..vec.len() {
        for j in (i..vec.len()).step_by(i).take(50) {
            vec[j] += 11 * i as u32;
        }
    }
    vec.into_iter().position(|x| x >= n)
}
