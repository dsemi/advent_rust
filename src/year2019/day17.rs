use super::intcode;

fn parse_grid(input: Vec<i64>) -> Vec<Vec<char>> {
    let inp: String = input.into_iter().map(|x| x as u8 as char).collect();
    inp.lines().map(|line| line.chars().collect()).collect()
}

fn is_scaffold(grid: &[Vec<char>], pos: (usize, usize)) -> bool {
    pos.1 < grid.len() && pos.0 < grid[pos.1].len() && "#^<>v".contains(grid[pos.1][pos.0])
}

pub fn part1(input: &str) -> usize {
    let mut prog = intcode::new(input);
    prog.run();
    let ins = prog.output.drain(..).collect();
    let grid = parse_grid(ins);
    (0..grid.len())
        .map(|y| {
            (0..grid[y].len())
                .filter_map(|x| {
                    [(x, y), (x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
                        .iter()
                        .all(|pos| is_scaffold(&grid, *pos))
                        .then_some(x * y)
                })
                .sum::<usize>()
        })
        .sum()
}

fn keep_moving(grid: &[Vec<char>], pos: (i64, i64), c: &str, d: (i64, i64)) -> Vec<String> {
    let mut p = (pos.0 + d.0, pos.1 + d.1);
    if !is_scaffold(grid, (p.0 as usize, p.1 as usize)) {
        return vec![];
    }
    while is_scaffold(grid, ((p.0 + d.0) as usize, (p.1 + d.1) as usize)) {
        p = (p.0 + d.0, p.1 + d.1);
    }
    let mut result = vec![
        c.to_string(),
        ((p.0 - pos.0).abs() + (p.1 - pos.1).abs()).to_string(),
    ];
    result.extend_from_slice(&go(grid, p, d));
    result
}

fn go(grid: &[Vec<char>], pos: (i64, i64), (x, y): (i64, i64)) -> Vec<String> {
    keep_moving(grid, pos, "L", (y, -x))
        .into_iter()
        .chain(keep_moving(grid, pos, "R", (-y, x)))
        .collect()
}

fn find_path(grid: &[Vec<char>]) -> Vec<String> {
    let (pos, dir) = (0..grid.len())
        .find_map(|r| {
            (0..grid[r].len()).find_map(|c| {
                "^><v"
                    .contains(grid[r][c])
                    .then(|| ((c as i64, r as i64), grid[r][c]))
            })
        })
        .unwrap();
    let res = go(
        grid,
        pos,
        match dir {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Bad dir: {}", dir),
        },
    );
    res.chunks(2).map(|ch| ch.join(",")).collect()
}

fn splits(x: &[String], s: &[String]) -> Vec<Vec<String>> {
    let mut i = 0;
    let mut c = 0;
    let mut result: Vec<Vec<String>> = Vec::new();
    while s.len() <= x.len() && i <= x.len() - s.len() {
        if x[i..i + s.len()].to_vec() == s {
            result.push(x[c..i].to_vec());
            i += s.len();
            c = i;
            continue;
        }
        i += 1;
    }
    result.push(x[c..].to_vec());
    result
}

fn go2(xs: Vec<Vec<String>>, fns: i64) -> Option<Vec<Vec<String>>> {
    if xs.is_empty() {
        return Some(vec![]);
    }
    if fns > 0 {
        for i in 1..=xs[0].len() {
            let candidate = &xs[0][..i].to_vec();
            let fragments = xs
                .iter()
                .flat_map(|x| splits(x, candidate).into_iter().filter(|y| !y.is_empty()))
                .collect();
            if let Some(res) = go2(fragments, fns - 1) {
                return Some(std::iter::once(candidate.clone()).chain(res).collect());
            }
        }
    }
    None
}

fn compress(instrs: Vec<String>) -> Vec<Vec<String>> {
    let repl_map = vec![
        vec!["A".to_string()],
        vec!["B".to_string()],
        vec!["C".to_string()],
    ]
    .into_iter()
    .zip(go2(vec![instrs.clone()], 3).unwrap())
    .collect::<Vec<_>>();
    std::iter::once(repl_map.iter().fold(instrs, |a, b| {
        itertools::Itertools::intersperse(splits(&a, &b.1).into_iter(), b.0.clone())
            .flatten()
            .collect()
    }))
    .chain(repl_map.into_iter().map(|x| x.1))
    .collect()
}

pub fn part2(input: &str) -> Option<i64> {
    let mut prog = intcode::new(input);
    prog.run();
    let ins = prog.output.drain(..).collect();
    let grid = parse_grid(ins);
    let path = find_path(&grid);
    let comped = compress(path);
    let mut inps = comped
        .into_iter()
        .map(|x| x.join(","))
        .chain(vec!["n".to_string()])
        .collect::<Vec<_>>()
        .join("\n");
    inps.push('\n');
    prog = intcode::new(input);
    prog[0] = 2;
    for c in inps.chars() {
        prog.input.push_back(c as i64);
    }
    prog.run();
    prog.output.drain(..).last()
}
