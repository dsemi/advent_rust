fn solve(input: &[u8], nchars: usize) -> Option<usize> {
    let mut i = nchars;
    while i < input.len() {
        let unique = input[i - nchars..i].iter().rev().enumerate().try_fold(0u32, |set, (j, b)| {
            let bit = 1 << (b - b'a');
            if set & bit == 0 { Ok(set | bit) } else { Err(nchars - j) }
        });
        match unique {
            Ok(_) => return Some(i),
            Err(n) => i += n,
        }
    }
    None
}

pub fn part1(input: &[u8]) -> Option<usize> {
    solve(input, 4)
}

pub fn part2(input: &[u8]) -> Option<usize> {
    solve(input, 14)
}
