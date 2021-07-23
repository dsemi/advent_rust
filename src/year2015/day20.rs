const LEN: usize = 1000000;

pub fn part1(input: &str) -> Option<usize> {
    let n: u32 = input.parse().unwrap();
    let mut vec = vec![0; LEN];
    for i in 1..vec.len() as u32 {
        for j in (i..vec.len() as u32).step_by(i as usize) {
            vec[j as usize] += 10 * i;
        }
    }
    vec.into_iter().position(|x| x >= n)
}

pub fn part2(input: &str) -> Option<usize> {
    let n: u32 = input.parse().unwrap();
    let mut vec = vec![0; LEN];
    for i in 1..vec.len() as u32 {
        for j in (i..vec.len() as u32).step_by(i as usize).take(50) {
            vec[j as usize] += 11 * i;
        }
    }
    vec.into_iter().position(|x| x >= n)
}
