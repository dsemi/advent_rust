fn read(input: &str, arr: &mut [u32]) {
    let ns: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .chain(10..arr.len() as u32)
        .collect();
    arr[0] = ns[0];
    for i in 0..ns.len() {
        arr[ns[i] as usize] = ns[(i + 1) % ns.len()];
    }
}

fn run(steps: u32, d: &mut [u32]) {
    let m = (d.len() - 1) as u32;
    let mut curr = d[0];
    for _ in 0..steps {
        let a = d[curr as usize];
        let b = d[a as usize];
        let c = d[b as usize];
        let mut n = curr;
        while n == curr || n == a || n == b || n == c {
            n = if n == 1 { m } else { n - 1 };
        }
        d[curr as usize] = d[c as usize];
        d[c as usize] = d[n as usize];
        d[n as usize] = a;
        curr = d[curr as usize];
    }
}

pub fn part1(input: &str) -> String {
    let mut arr = vec![0; 10];
    read(input, &mut arr);
    run(100, &mut arr);
    let mut result = String::new();
    let mut x = arr[1];
    while x != 1 {
        result.push_str(&x.to_string());
        x = arr[x as usize];
    }
    result
}

pub fn part2(input: &str) -> u64 {
    let mut arr = vec![0; 1_000_001];
    read(input, &mut arr);
    run(10_000_000, &mut arr);
    arr[1] as u64 * arr[arr[1] as usize] as u64
}
