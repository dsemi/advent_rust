pub fn part1(input: &str) -> Option<usize> {
    let n: usize = input.parse().unwrap();
    let lim: usize = n / 10 + 1;
    let mut vec = vec![0; lim];
    for i in 1..lim {
        for j in (i..lim).step_by(i) {
            vec[j] += 10*i;
        }
    }
    vec.into_iter().position(|x| x >= n)
}

pub fn part2(input: &str) -> Option<usize> {
    let n: usize = input.parse().unwrap();
    let lim: usize = n / 11 + 1;
    let mut vec = vec![0; lim];
    for i in 1..lim {
        for j in (i..lim).step_by(i).take(50) {
            vec[j] += 11*i;
        }
    }
    vec.into_iter().position(|x| x >= n)
}
