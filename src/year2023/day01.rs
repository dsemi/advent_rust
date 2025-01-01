pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let tens = line.bytes().find(u8::is_ascii_digit).unwrap() - b'0';
            let ones = line.bytes().rfind(u8::is_ascii_digit).unwrap() - b'0';
            10 * tens as usize + ones as usize
        })
        .sum()
}

const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part2(input: &str) -> usize {
    let digs = DIGITS.iter().enumerate();
    let repls: Vec<_> = WORDS.iter().enumerate().chain(digs).collect();
    input
        .lines()
        .map(|line| {
            let tens = (0..line.len())
                .map(|i| &line[i..])
                .find_map(|s| repls.iter().find(|x| s.starts_with(x.1)).map(|x| x.0 + 1))
                .unwrap();
            let ones = (1..line.len() + 1)
                .rev()
                .map(|i| &line[..i])
                .find_map(|s| repls.iter().find(|x| s.ends_with(x.1)).map(|x| x.0 + 1))
                .unwrap();
            10 * tens + ones
        })
        .sum()
}
