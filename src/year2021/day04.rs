use ahash::AHashSet;
use genawaiter::stack::{let_gen_using, Co};

fn is_winner(brd: &[Vec<u32>], ns: &AHashSet<u32>) -> bool {
    (0..brd.len()).any(|i| {
        (0..brd.len()).all(|j| ns.contains(&brd[i][j]))
            || (0..brd.len()).all(|j| ns.contains(&brd[j][i]))
    })
}

async fn winner_scores(input: &str, co: Co<'_, u32>) {
    let (nums, board_str) = input.split_once("\n\n").unwrap();
    let boards: Vec<Vec<Vec<u32>>> = board_str
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .map(|row| row.split_whitespace().map(|n| n.parse().unwrap()).collect())
                .collect()
        })
        .collect();
    let mut won = vec![false; boards.len()];
    let mut called = AHashSet::new();
    for n in nums.split(',').map(|n| n.parse().unwrap()) {
        called.insert(n);
        for (i, board) in boards.iter().enumerate() {
            if !won[i] && is_winner(board, &called) {
                won[i] = true;
                co.yield_(
                    board
                        .iter()
                        .flatten()
                        .filter(|n| !called.contains(n))
                        .sum::<u32>()
                        * n,
                )
                .await;
                if won.iter().all(|&v| v) {
                    return;
                }
            }
        }
    }
}

pub fn part1(input: &str) -> Option<u32> {
    let_gen_using!(gen, |co| winner_scores(input, co));
    gen.into_iter().next()
}

pub fn part2(input: &str) -> Option<u32> {
    let_gen_using!(gen, |co| winner_scores(input, co));
    gen.into_iter().last()
}
