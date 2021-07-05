fn read(input: &str, arr: &mut [usize]) {
    let ns: Vec<usize> = input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chain(10..arr.len())
        .collect();
    arr[0] = ns[0];
    for i in 0..ns.len() {
        arr[ns[i]] = ns[(i+1) % ns.len()];
    }
}

fn run(steps: u32, d: &mut [usize]) {
    let m = d.len() - 1;
    let mut curr = d[0];
    for _ in 0..steps {
        let a = d[curr];
        let b = d[a];
        let c = d[b];
        d[curr] = d[c];
        let mut n = if curr > 1 { curr - 1 } else { m };
        while n == a || n == b || n == c {
            n = if n == 1 { m } else { n - 1 };
        }
        d[c] = d[n];
        d[n] = a;
        curr = d[curr];
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
        x = arr[x];
    }
    result
}

pub fn part2(input: &str) -> usize {
    let mut arr = vec![0; 1_000_001];
    read(input, &mut arr);
    run(10_000_000, &mut arr);
    arr[1] * arr[arr[1]]
}
