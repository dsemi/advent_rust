fn look_and_say(input: &str) -> String {
    let inp = input.as_bytes();
    let mut out = String::new();
    let mut curr = inp[0];
    let mut count = 1;
    for c in &inp[1..] {
        if curr == *c {
            count += 1;
            continue;
        }

        out.push_str(&count.to_string());
        out.push(curr as char);
        curr = *c;
        count = 1;
    }
    out.push_str(&count.to_string());
    out.push(curr as char);
    out
}

pub fn part1(input: &str) -> usize {
    let mut s = input.to_string();
    for _ in 0..40 {
        s = look_and_say(&s);
    }
    s.len()
}

pub fn part2(input: &str) -> usize {
    let mut s = input.to_string();
    for _ in 0..50 {
        s = look_and_say(&s);
    }
    s.len()
}
