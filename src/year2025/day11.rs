use cached::proc_macro::cached;

const LEN: usize = 17576;
const YOU: usize = id("you");
const OUT: usize = id("out");
const SVR: usize = id("svr");
const DAC: usize = id("dac");
const FFT: usize = id("fft");

const fn id(i: &str) -> usize {
    let &[a, b, c] = i.as_bytes() else { unreachable!() };
    676 * (a as usize - 97) + 26 * (b as usize - 97) + (c as usize - 97)
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; LEN];
    for mut ids in input.lines().map(|line| line.split_whitespace().map(|w| id(&w[..3]))) {
        graph[ids.next().unwrap()].extend(ids);
    }
    graph
}

#[cached(key = "(usize, usize)", convert = r#"{ (src, dst) }"#)]
fn dfs(g: &[Vec<usize>], src: usize, dst: usize) -> usize {
    if src == dst { 1 } else { g[src].iter().map(|&src| dfs(g, src, dst)).sum() }
}

pub fn part1(input: &str) -> usize {
    dfs(&parse(input), YOU, OUT)
}

pub fn part2(input: &str) -> usize {
    let g = parse(input);
    let p1 = dfs(&g, SVR, DAC) * dfs(&g, DAC, FFT) * dfs(&g, FFT, OUT);
    let p2 = dfs(&g, SVR, FFT) * dfs(&g, FFT, DAC) * dfs(&g, DAC, OUT);
    p1 + p2
}
