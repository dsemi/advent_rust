use cached::proc_macro::cached;

const YOU: usize = id("you");
const OUT: usize = id("out");
const SVR: usize = id("svr");
const DAC: usize = id("dac");
const FFT: usize = id("fft");

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

#[cached(key = "(usize, usize)", convert = r#"{ (src, dst) }"#)]
fn dfs(g: &[Vec<usize>], (src, dst): (usize, usize)) -> usize {
    if src == dst { 1 } else { g[src].iter().map(|&src| dfs(g, (src, dst))).sum() }
}

pub fn part1(input: &str) -> usize {
    dfs(&parse(input), (YOU, OUT))
}

pub fn part2(input: &str) -> usize {
    let g = parse(input);
    let mid = (dfs(&g, (DAC, FFT)), (DAC, FFT)).max((dfs(&g, (FFT, DAC)), (FFT, DAC))).1;
    dfs(&g, (SVR, mid.0)) * dfs(&g, mid) * dfs(&g, (mid.1, OUT))
}
