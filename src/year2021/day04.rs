use crate::utils::parsers::*;

fn is_winner(brd: &[Vec<i32>]) -> bool {
    (0..brd.len())
        .any(|i| (0..brd.len()).all(|j| brd[i][j] == -1) || (0..brd.len()).all(|j| brd[j][i] == -1))
}

fn winner_scores(input: &str) -> impl Iterator<Item = i32> + use<'_> {
    let (nums, boards) = input.split_once("\n\n").unwrap();
    let mut boards: Vec<Vec<Vec<_>>> =
        separated(1.., lines(repeat(1.., strip(i32))), "\n\n").read(boards);
    nums.split(',').flat_map(move |n| {
        let n = n.i32();
        // Use drain_filter when it stabilizes.
        let mut winners = vec![];
        boards.retain_mut(|b| {
            for row in b.iter_mut() {
                for v in row.iter_mut() {
                    if *v == n {
                        *v = -1;
                    }
                }
            }
            if is_winner(b) {
                winners.push(b.iter().flatten().filter(|n| *n != &-1).sum::<i32>() * n);
                return false;
            }
            true
        });
        winners
    })
}

pub fn part1(input: &str) -> Option<i32> {
    winner_scores(input).next()
}

pub fn part2(input: &str) -> Option<i32> {
    winner_scores(input).last()
}
