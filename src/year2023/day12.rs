use crate::utils::parsers::*;

fn solve(input: &str, reps: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let (pat, splits) = line.split_once(' ').unwrap();
            let mut pat = vec![pat.to_owned(); reps].join("?");
            pat.push('.');
            let pat = pat.as_bytes();
            let n = pat.len();
            let splits = list(usize).read(splits);
            let mut splits: Vec<_> = vec![splits; reps].into_iter().flatten().collect();
            let k = splits.len();
            splits.push(n + 1);
            let mut dp = vec![vec![vec![0; n + 2]; k + 2]; n + 1];
            dp[0][0][0] = 1;
            for i in 0..n {
                for j in 0..k + 1 {
                    for p in 0..n + 1 {
                        let cur = dp[i][j][p];
                        if cur == 0 {
                            continue;
                        }
                        if pat[i] == b'.' || pat[i] == b'?' {
                            if p == 0 || p == splits[j - 1] {
                                dp[i + 1][j][0] += cur;
                            }
                        }
                        if pat[i] == b'#' || pat[i] == b'?' {
                            dp[i + 1][j + (if p == 0 { 1 } else { 0 })][p + 1] += cur;
                        }
                    }
                }
            }
            dp[n][k][0]
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, 1)
}

pub fn part2(input: &str) -> usize {
    solve(input, 5)
}
