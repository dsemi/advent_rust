unsafe fn increment(s: &mut str) {
    let b = s.as_bytes_mut();
    for i in (0..b.len()).rev() {
        if b[i] == b'z' {
            b[i] = b'a';
        } else {
            b[i] += 1;
            if b[i] == b'i' || b[i] == b'o' || b[i] == b'l' {
                b[i] += 1;
                for x in i+1..b.len() {
                    b[x] = b'a';
                }
            }
            break;
        }
    }
}

lazy_static! {
    static ref INCREASING_TRIPLETS: Vec<String> = ('a'..='z')
        .collect::<Vec<char>>()
        .windows(3)
        .map(|x| x.iter().collect())
        .collect();
    static ref LETTER_PAIRS: Vec<String> = ('a'..='z').map(|c| [c, c].iter().collect()).collect();
}

fn is_valid(s: &str) -> bool {
    INCREASING_TRIPLETS.iter().any(|t| s.contains(t))
        // && !"iol".chars().any(|c| s.contains(c))
        && LETTER_PAIRS.iter().filter(|&p| s.contains(p)).count() >= 2
}

unsafe fn next_valid_pw(s: &mut str) {
    increment(s);
    while !is_valid(s) {
        increment(s);
    }
}

pub fn part1(input: &str) -> String {
    let mut s = input.to_string();
    unsafe {
        next_valid_pw(&mut s);
    }
    s.to_string()
}

pub fn part2(input: &str) -> String {
    let mut s = input.to_string();
    unsafe {
        next_valid_pw(&mut s);
        next_valid_pw(&mut s);
    }
    s.to_string()
}
