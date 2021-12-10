use ahash::AHashSet;

fn is_winner(brd: &[Vec<u32>], ns: &AHashSet<u32>) -> bool {
    (0..brd.len()).any(|i| {
        (0..brd.len()).all(|j| ns.contains(&brd[i][j]))
            || (0..brd.len()).all(|j| ns.contains(&brd[j][i]))
    })
}

fn winner_scores(input: &str) -> impl Iterator<Item = u32> + '_ {
    let (nums, board_str) = input.split_once("\n\n").unwrap();
    let mut boards: Vec<Vec<Vec<u32>>> = board_str
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .map(|row| row.split_whitespace().map(|n| n.parse().unwrap()).collect())
                .collect()
        })
        .collect();
    let mut called = AHashSet::new();
    nums.split(',').flat_map(move |n| {
        let n = n.parse().unwrap();
        called.insert(n);
        // drain_filter (can also mutate w/ drain_filter, removes need for HashSet,
        // set called ns to 0).
        let mut winners = vec![];
        boards.retain(|b| {
            if is_winner(b, &called) {
                winners.push(
                    b.iter()
                        .flatten()
                        .filter(|n| !called.contains(n))
                        .sum::<u32>()
                        * n,
                );
                return false;
            }
            true
        });
        winners
    })
}

pub fn part1(input: &str) -> Option<u32> {
    winner_scores(input).next()
}

pub fn part2(input: &str) -> Option<u32> {
    winner_scores(input).last()
}
