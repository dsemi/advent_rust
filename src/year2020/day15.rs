use bit_set::BitSet;

fn solve(n: u32, input: &str) -> u32 {
    let mut m = vec![0; n as usize];
    let mut filter = BitSet::new();
    let mut j = 1;
    for v in input.split(',') {
        let k = v.parse().unwrap();
        m[k] = j;
        filter.insert(k);
        j += 1;
    }
    let mut result = 0;
    for i in j..n {
        if result < i >> 6 {
            result = std::mem::replace(&mut m[result as usize], i);
            if result != 0 {
                result = i - result;
            }
        } else if !filter.insert(result as usize) {
            result = i - std::mem::replace(&mut m[result as usize], i);
        } else {
            m[result as usize] = i;
            result = 0;
        }
    }
    result
}

pub fn part1(input: &str) -> u32 {
    solve(2020, input)
}

pub fn part2(input: &str) -> u32 {
    solve(30_000_000, input)
}
