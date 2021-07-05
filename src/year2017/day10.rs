fn reverse<T>(v: &mut Vec<T>, mut lo: usize, mut hi: usize) {
    let len = v.len();
    while lo < hi {
        v.swap(lo % len, hi % len);
        lo += 1;
        hi -= 1;
    }
}

fn hash(n: usize, lens: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = (0..256).collect();
    let mut pos = 0;
    let mut skip_size = 0;
    for _ in 0..n {
        for l in &lens {
            reverse(&mut result, pos, pos + l - 1);
            pos += l + skip_size;
            skip_size += 1;
        }
    }
    result
}

pub fn part1(input: &str) -> usize {
    let res = hash(1, input.split(',').map(|x| x.parse().unwrap()).collect());
    res[0] * res[1]
}

pub fn part2(input: &str) -> String {
    let res = hash(
        64,
        input
            .chars()
            .map(|x| x as u8 as usize)
            .chain(vec![17, 31, 73, 47, 23])
            .collect(),
    );
    hex::encode(
        res.chunks(res.len() / 16)
            .map(|x| x.iter().fold(0, |a, b| a ^ *b) as u8)
            .collect::<Vec<_>>(),
    )
}
