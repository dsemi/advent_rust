use ahash::AHashMap;
use std::iter::once;

struct Node<'a>(&'a dyn Fn(u16, u16) -> u16, &'a str, &'a str);

type Network<'a> = AHashMap<&'a str, Node<'a>>;

fn val<'a>(graph: &Network<'a>, cache: &mut AHashMap<&'a str, u16>, signal: &'a str) -> u16 {
    signal.parse().ok().unwrap_or_else(|| {
        if !cache.contains_key(&signal) {
            let Node(f, a, b) = graph[&signal];
            let v = f(val(graph, cache, a), val(graph, cache, b));
            cache.insert(signal, v);
        }
        cache[&signal]
    })
}

fn lookup<'a>(graph: Network<'a>, signal: &'a str, mut cache: AHashMap<&'a str, u16>) -> u16 {
    val(&graph, &mut cache, signal)
}

fn parse_cmds<'a>(input: &'a str) -> Network<'a> {
    input
        .lines()
        .map(|line| {
            let cmd = line.split(" ").collect::<Vec<_>>();
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
    lookup(parse_cmds(input), "a", AHashMap::new())
}

pub fn part2(input: &str) -> u16 {
    lookup(parse_cmds(input), "a", once(("b", part1(input))).collect())
}
