const fn id(i: &str) -> usize {
    let (a, b, c) = (i.as_bytes()[0], i.as_bytes()[1], i.as_bytes()[2]);
    676 * (a as usize - 97) + 26 * (b as usize - 97) + (c as usize - 97)
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; 17576];
    for mut ids in input.lines().map(|line| line.split_whitespace().map(id)) {
        graph[ids.next().unwrap()].extend(ids);
    }
    graph
}

fn dfs(g: &[Vec<usize>], c: &mut [u64], src: usize, dst: usize) -> u64 {
    if src == dst {
        return 1;
    }
    if c[src] == u64::MAX {
        c[src] = g[src].iter().map(|&src| dfs(g, c, src, dst)).sum()
    }
    c[src]
}

pub fn part1(input: &str) -> u64 {
    dfs(&parse(input), &mut vec![u64::MAX; 17576], id("you"), id("out"))
}

pub fn part2(input: &str) -> u64 {
    let g = parse(input);
    let f = |src, dst| dfs(&g, &mut vec![u64::MAX; 17576], src, dst);
    let (dac, fft) = (id("dac"), id("fft"));
    let (paths, (a, b)) = (f(dac, fft), (dac, fft)).max((f(fft, dac), (fft, dac)));
    f(id("svr"), a) * paths * f(b, id("out"))
}
