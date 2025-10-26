fn ap(nums: &mut Vec<i64>, ops: &mut Vec<char>) {
    let b = nums.pop().unwrap();
    let a = nums.pop().unwrap();
    match ops.pop().unwrap() {
        '+' => nums.push(a + b),
        '*' => nums.push(a * b),
        _ => panic!("Invalid op"),
    }
}

fn calc<I: Iterator<Item = char>>(s: &mut I, prec: fn(char) -> u8) -> i64 {
    let mut nums = vec![];
    let mut ops = vec![];
    while let Some(c) = s.next() {
        match c {
            '0'..='9' => nums.push(c as i64 - '0' as i64),
            '(' => nums.push(calc(s, prec)),
            ')' => break,
            '+' | '*' => {
                if !ops.is_empty() && prec(c) <= prec(ops[ops.len() - 1]) {
                    ap(&mut nums, &mut ops);
                }
                ops.push(c);
            }
            _ => (),
        }
    }
    while !ops.is_empty() {
        ap(&mut nums, &mut ops);
    }
    nums.pop().unwrap()
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(|line| calc(&mut line.chars(), |_| 1)).sum()
}

pub fn part2(input: &str) -> i64 {
    input.lines().map(|line| calc(&mut line.chars(), |x| if x == '+' { 2 } else { 1 })).sum()
}
