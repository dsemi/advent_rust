use crate::utils::parsers::*;

struct Tree {
    val: Vec<usize>,
    children: Vec<Tree>,
}

impl Tree {
    fn sum(&self) -> usize {
        self.val.iter().sum::<usize>() + self.children.iter().map(|t| t.sum()).sum::<usize>()
    }
}

fn parse_nodes(input: &str) -> Tree {
    let mut ns = input.split_whitespace().map(usize::read);
    fn parse_node<I: Iterator<Item = usize>>(ns: &mut I) -> Tree {
        let n = ns.next().unwrap();
        let m = ns.next().unwrap();
        let nodes = (0..n).map(|_| parse_node(ns)).collect();
        let vals = (0..m).map(|_| ns.next().unwrap()).collect();
        Tree {
            val: vals,
            children: nodes,
        }
    }
    parse_node(&mut ns)
}

pub fn part1(input: &str) -> usize {
    parse_nodes(input).sum()
}

pub fn part2(input: &str) -> usize {
    let tree = parse_nodes(input);
    let mut stack = vec![&tree];
    let mut result = 0;
    while let Some(node) = stack.pop() {
        if node.children.is_empty() {
            result += node.val.iter().sum::<usize>();
            continue;
        }
        for i in &node.val {
            if i - 1 < node.children.len() {
                stack.push(&node.children[i - 1]);
            }
        }
    }
    result
}
