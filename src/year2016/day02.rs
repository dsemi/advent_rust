use crate::utils::C;
use hashbrown::HashMap;

fn run(input: &str, pad: &str) -> String {
    let d: HashMap<C<i32>, &str> = pad
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.split(' ')
                .enumerate()
                .filter(|(_, c)| c != &".")
                .map(|(x, c)| (C(x as i32, y as i32), c))
                .collect::<Vec<_>>()
        })
        .collect();
    let mut xy = *d.keys().find(|&k| d[k] == "5").unwrap();
    let mut result = String::new();
    for line in input.lines() {
        xy = line.chars().fold(xy, |a, b| {
            let dir = match b {
                'U' => C(0, -1),
                'D' => C(0, 1),
                'L' => C(-1, 0),
                'R' => C(1, 0),
                _ => panic!("Invalid dir {}", b),
            };
            if d.contains_key(&(a + dir)) { a + dir } else { a }
        });
        result.push_str(d[&xy]);
    }
    result
}

pub fn part1(input: &str) -> String {
    run(
        input,
        "1 2 3\n\
         4 5 6\n\
         7 8 9",
    )
}

pub fn part2(input: &str) -> String {
    run(
        input,
        ". . 1 . .\n\
         . 2 3 4 .\n\
         5 6 7 8 9\n\
         . A B C .\n\
         . . D . .",
    )
}
