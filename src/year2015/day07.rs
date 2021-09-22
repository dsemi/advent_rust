use ahash::AHashMap;

use crate::utils::Cache;

struct Node<'a>(&'a dyn Fn(u16, u16) -> u16, &'a str, &'a str);

type Network<'a> = AHashMap<&'a str, Node<'a>>;

fn lookup<'a>(graph: Network<'a>, signal: &'a str) -> u16 {
    let func = |cache: &mut Cache<&'a str, u16>, sig: &'a str| {
        sig.parse().unwrap_or_else(|_| {
            let Node(f, a, b) = graph[&sig];
            f(cache.get(a), cache.get(b))
        })
    };
    Cache::from(&func).get(signal)
}

fn parse_cmds(input: &str) -> Network<'_> {
    input
        .lines()
        .map(|line| {
            let cmd = line.split(' ').collect::<Vec<_>>();
            let node = match cmd[..cmd.len() - 2] {
                [a, "AND", b] => Node(&|a, b| a & b, a, b),
                [a, "OR", b] => Node(&|a, b| a | b, a, b),
                [a, "LSHIFT", b] => Node(&|a, b| a << b, a, b),
                [a, "RSHIFT", b] => Node(&|a, b| a >> b, a, b),
                ["NOT", b] => Node(&|_, b| !b, "1", b),
                [b] => Node(&|_, b| b, "1", b),
                _ => panic!("Bad parse {}", line),
            };
            (cmd[cmd.len() - 1], node)
        })
        .collect()
}

pub fn part1(input: &str) -> u16 {
    lookup(parse_cmds(input), "a")
}

pub fn part2(input: &str) -> u16 {
    let mut inp = input.to_string();
    inp.push_str(&format!("\n{} -> b", part1(input)));
    lookup(parse_cmds(&inp), "a")
}
