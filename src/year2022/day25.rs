const IN: &str = "=-012";
const OUT: [char; 5] = ['0', '1', '2', '=', '-'];

pub fn part1(input: &str) -> String {
    let mut num = 0;
    for line in input.lines() {
        for (i, c) in line.chars().rev().enumerate() {
            num += 5i64.pow(i as u32) * (IN.chars().position(|i| i == c).unwrap() as i64 - 2);
        }
    }
    let mut res = Vec::new();
    while num > 0 {
        res.push(OUT[(num % 5) as usize]);
        num = (num + 2) / 5;
    }
    res.into_iter().rev().collect()
}

pub fn part2(_input: &str) -> String {
    " ".to_owned()
}
