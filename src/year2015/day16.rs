fn is_sue(k: &str, x: i32) -> bool {
    match k {
        "children" => x == 3,
        "cats" => x == 7,
        "samoyeds" => x == 2,
        "pomeranians" => x == 3,
        "akitas" => x == 0,
        "vizslas" => x == 0,
        "goldfish" => x == 5,
        "trees" => x == 3,
        "cars" => x == 2,
        "perfumes" => x == 1,
        _ => panic!("Missing compound: {}", k),
    }
}

fn solve(input: &str, f: fn(&str, i32) -> bool) -> Option<usize> {
    input
        .lines()
        .position(|line| {
            line.split_once(": ").unwrap().1.split(", ").all(|attr| {
                let (key, val) = attr.split_once(": ").unwrap();
                f(key, val.parse().unwrap())
            })
        })
        .map(|x| x + 1)
}

pub fn part1(input: &str) -> Option<usize> {
    solve(input, is_sue)
}

pub fn part2(input: &str) -> Option<usize> {
    solve(input, |k, x| match k {
        "cats" => x > 7,
        "pomeranians" => x < 3,
        "goldfish" => x < 5,
        "trees" => x > 3,
        _ => is_sue(k, x),
    })
}
