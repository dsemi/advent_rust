use std::collections::BTreeSet;

fn solve(x: usize, ns: BTreeSet<i64>) -> Option<i64> {
    fn go(n: usize, c: i64, xs: BTreeSet<i64>) -> Option<i64> {
        if n == 1 {
            xs.contains(&c).then(|| c)
        } else {
            xs.iter()
                .find_map(|x2| go(n - 1, c - x2, xs.clone().split_off(&(x2 + 1))).map(|x3| x2 * x3))
        }
    }

    go(x, 2020, ns)
}

pub fn part1(input: BTreeSet<i64>) -> Option<i64> {
    solve(2, input)
}

pub fn part2(input: BTreeSet<i64>) -> Option<i64> {
    solve(3, input)
}
