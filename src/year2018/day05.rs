fn react(s: &str) -> usize {
    let mut chs: Vec<char> = Vec::new();
    for c in s.chars() {
        if !chs.is_empty()
            && chs[chs.len() - 1] != c
            && chs[chs.len() - 1].to_ascii_lowercase() == c.to_ascii_lowercase()
        {
            chs.pop();
        } else {
            chs.push(c);
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
