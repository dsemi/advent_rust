fn dragon_checksum(desired_len: usize, input: &str) -> String {
    let sz = {
        let mut x = desired_len;
        while x.is_multiple_of(2) {
            x /= 2;
        }
        desired_len / x
    };
    let mut ns: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    while ns.len() < desired_len {
        let c = ns.len();
        ns.push(0);
        for i in (0..c).rev() {
            ns.push(ns[i] ^ 1);
        }
    }
    (0..desired_len)
        .step_by(sz)
        .map(|n| if ns[n..n + sz].iter().sum::<u32>() % 2 == 1 { '0' } else { '1' })
        .collect()
}

pub fn part1(input: &str) -> String {
    dragon_checksum(272, input)
}

pub fn part2(input: &str) -> String {
    dragon_checksum(35651584, input)
}
