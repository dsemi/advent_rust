fn aabcc(mut s: [u8; 8], a: u8) -> [u8; 8] {
    s[3] = a;
    s[4] = a;
    s[5] = a + 1;
    s[6] = a + 2;
    s[7] = a + 2;
    s
}

fn next_valid_pw(mut s: [u8; 8]) -> [u8; 8] {
    if (b'g'..=b'o').contains(&s[3]) {
        return aabcc(s, b'p');
    }
    if s[3] <= b'x' {
        let n = aabcc(s, s[3]);
        if n > s {
            return n;
        }
    }
    if s[3] == b'x' {
        s[2] += 1;
        if matches!(s[2], b'i' | b'l' | b'o') {
            s[2] += 1;
        }
        aabcc(s, b'a')
    } else if s[3] == b'f' {
        aabcc(s, b'p')
    } else {
        aabcc(s, s[3] + 1)
    }
}

pub fn part1(input: &[u8]) -> [u8; 8] {
    next_valid_pw(input.try_into().unwrap())
}

pub fn part2(input: &[u8]) -> [u8; 8] {
    next_valid_pw(next_valid_pw(input.try_into().unwrap()))
}
