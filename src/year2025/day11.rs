use crate::utils::parsers::*;
use hashbrown::HashMap;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input.lines().map(|line| separated_pair(alpha1, ": ", spaced(alpha1)).read(line)).collect()
}

fn dfs<'a, const N: usize>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<([bool; N], &'a str), usize>,
    conds: &[fn(&'a str) -> bool; N],
    mut st: [bool; N],
    k: &'a str,
) -> usize {
    st.iter_mut().zip(conds).for_each(|(v, cond)| *v = *v || cond(k));
    if k == "out" {
        return usize::from(st.iter().all(|&b| b));
    }
    if let Some(&v) = cache.get(&(st, k)) {
        return v;
    }
    let v = graph.get(k).unwrap_or(&vec![]).iter().map(|k| dfs(graph, cache, conds, st, k)).sum();
    cache.insert((st, k), v);
    v
}

pub fn part1(input: &str) -> usize {
    dfs(&parse(input), &mut HashMap::new(), &[], [], "you")
}

pub fn part2(input: &str) -> usize {
    let graph = parse(input);
    dfs(&graph, &mut HashMap::new(), &[|k| k == "dac", |k| k == "fft"], [false, false], "svr")
}
