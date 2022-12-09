struct Tree {
    visible_from_edge: bool,
    scenic_score: u32,
}

fn trees(input: &str) -> Vec<Tree> {
    let mut result = Vec::new();
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let mut tree = Tree {
                visible_from_edge: false,
                scenic_score: 1,
            };
            let paths: Vec<Box<dyn Iterator<Item = u32>>> = vec![
                Box::new((0..r).rev().map(|nr| grid[nr][c])),
                Box::new((r + 1..grid.len()).into_iter().map(|nr| grid[nr][c])),
                Box::new((0..c).rev().map(|nc| grid[r][nc])),
                Box::new((c + 1..grid[0].len()).into_iter().map(|nc| grid[r][nc])),
            ];
            for mut path in paths {
                let mut cnt = 0;
                let mut edge = true;
                for x in path.by_ref() {
                    cnt += 1;
                    if x >= grid[r][c] {
                        edge = false;
                        break;
                    }
                }
                tree.visible_from_edge |= edge;
                tree.scenic_score *= cnt;
            }
            result.push(tree);
        }
    }
    result
}

pub fn part1(input: &str) -> usize {
    trees(input)
        .into_iter()
        .filter(|t| t.visible_from_edge)
        .count()
}

pub fn part2(input: &str) -> Option<u32> {
    trees(input).into_iter().map(|t| t.scenic_score).max()
}
