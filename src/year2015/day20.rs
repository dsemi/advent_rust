const LEN: usize = 1000000;

pub fn part1(input: &str) -> Option<usize> {
    let n: u32 = input.parse().unwrap();
    let mut vec = vec![0; LEN];
    for i in 1..vec.len(){
        for j in (i..vec.len()).step_by(i) {
            vec[j] += 10 * i as u32;
        }
    }
    vec.into_iter().position(|x| x >= n)
}

pub fn part2(input: &str) -> Option<usize> {
    let n: u32 = input.parse().unwrap();
    let mut vec = vec![0; LEN];
    for i in 1..vec.len() {
        for j in (i..vec.len()).step_by(i).take(50) {
            vec[j] += 11 * i as u32;
        }
    }
    vec.into_iter().position(|x| x >= n)
}
