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

fn is_valid(s: &str) -> bool {
    let mut result = false;
    let b = s.as_bytes();
    for i in 0..b.len()-2 {
        if b[i] + 2 == b[i+1] + 1 && b[i+1] + 1 == b[i+2] {
            result = true;
            break;
        }
    }
    if !result {
        return false
    }
    let mut cnt = 0;
    let mut i = 0;
    while i < b.len() - 1 {
        if b[i] == b[i+1] {
            cnt += 1;
            i += 1;
        }
        i += 1;
    }
    return cnt >= 2
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
