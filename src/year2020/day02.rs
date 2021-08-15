fn count_valid(f: fn(usize, usize, char, &str) -> bool, input: &str) -> usize {
    input.lines()
        .filter(|line| {
            let vs: Vec<&str> = line.split(' ').collect();
            let ns: Vec<usize> = vs[0].split('-').map(|x| x.parse().unwrap()).collect();
            f(ns[0], ns[1], vs[1].chars().next().unwrap(), vs[2])
        })
        .count()
}

pub fn part1(input: &str) -> usize {
    count_valid(|lo, hi, c, s| {
        let count = s.matches(c).count();
        lo <= count && count <= hi
    }, input)
}

pub fn part2(input: &str) -> usize {
    count_valid(|lo, hi, c, s| {
        let chrs: Vec<char> = s.chars().collect();
        (chrs[lo-1] == c) != (chrs[hi-1] == c)
    }, input)
}
