fn react(s: &str) -> usize {
    let mut chs: Vec<char> = Vec::new();
    for c in s.chars() {
        match chs.last() {
            Some(x) if *x != c && x.to_ascii_lowercase() == c.to_ascii_lowercase() => {
                chs.pop();
            }
            _ => {
                chs.push(c);
            }
        }
    }
    chs.len()
}

pub fn part1(input: &str) -> usize {
    react(input)
}

pub fn part2(input: &str) -> Option<usize> {
    ('a'..'z')
        .map(|c| react(&input.replace(&[c, c.to_ascii_uppercase()][..], "")))
        .min()
}
