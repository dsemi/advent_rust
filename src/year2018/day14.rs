struct Step {
    rs: Vec<u8>,
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
            let tot = elf1_score + elf2_score;
            if tot >= 10 {
                self.rs.push(1);
                self.rs.push(tot % 10);
            } else {
                self.rs.push(tot);
            }
            self.elf1 = (elf1_score as usize + self.elf1 + 1) % self.rs.len();
            self.elf2 = (elf2_score as usize + self.elf2 + 1) % self.rs.len();
        }
        let ans = (self.rs[self.idx] + b'0') as char;
        self.idx += 1;
        Some(ans)
    }
}

pub fn part1(n: usize) -> String {
    Step::new().skip(n).take(10).collect()
}

pub fn part2(input: &str) -> usize {
    let mut rs = String::new();
    for (i, c) in Step::new().enumerate() {
        rs.push(c);
        if rs.ends_with(input) {
            return i - input.len() + 1;
        }
    }
    unreachable!()
}
