fn look_and_say(n: usize, input: &str) -> usize {
    let mut inp: Vec<u8> = input.bytes().collect();
    let mut out = Vec::with_capacity(inp.len());
    for _ in 0..n {
        let mut curr = inp[0];
        let mut count = 1;
        for c in &inp[1..] {
            if curr == *c {
                count += 1;
                continue;
            }
            assert!(count < 10);
            out.push(count + b'0');
            out.push(curr);
            curr = *c;
            count = 1;
        }
        assert!(count < 10);
        out.push(count + b'0');
        out.push(curr);
        std::mem::swap(&mut inp, &mut out);
        out.clear();
    }
    inp.len()
}

pub fn part1(input: &str) -> usize {
    look_and_say(40, input)
}

pub fn part2(input: &str) -> usize {
    look_and_say(50, input)
}
