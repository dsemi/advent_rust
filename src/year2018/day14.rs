struct Step {
    rs: Vec<u32>,
    elf1: usize,
    elf2: usize,
    idx: usize,
}

impl Step {
    fn new() -> Self {
        Step {
            rs: vec![3, 7],
            elf1: 0,
            elf2: 1,
            idx: 0,
        }
    }
}

impl Iterator for Step {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.idx >= self.rs.len() {
            let elf1_score = self.rs[self.elf1];
            let elf2_score = self.rs[self.elf2];
            self.rs.extend(digits(elf1_score + elf2_score));
            self.elf1 = (elf1_score as usize + self.elf1 + 1).rem_euclid(self.rs.len());
            self.elf2 = (elf2_score as usize + self.elf2 + 1).rem_euclid(self.rs.len());
        }
        let ans = std::char::from_digit(self.rs[self.idx], 10);
        self.idx += 1;
        ans
    }
}

fn digits(n: u32) -> Vec<u32> {
    if n < 10 {
        return vec![n];
    }
    vec![1, n % 10]
}

pub fn part1(input: &str) -> String {
    let n = input.parse().unwrap();
    Step::new().skip(n).take(10).collect()
}

pub fn part2(input: &str) -> usize {
    let mut rs = String::new();
    let mut s = Step::new();
    for _ in 0..input.len() {
        rs.push(s.next().unwrap());
    }
    let mut c = 0;
    while &rs[c..] != input {
        rs.push(s.next().unwrap());
        c += 1;
    }
    c
}
