use scan_fmt::scan_fmt as scanf;
use std::collections::VecDeque;

enum Dir {
    L,
    R,
}

struct Rule {
    write: usize,
    dir: Dir,
    state: usize,
}

fn parse_branch<'a, I: Iterator<Item = &'a str>>(gen: &mut I, idx: usize) -> Option<Rule> {
    assert!(gen.next()? == format!("  If the current value is {idx}:"));
    let write = scanf!(gen.next()?, "    - Write the value {}.", usize).unwrap();
    let dir = scanf!(gen.next()?, "    - Move one slot to the {}.", String).unwrap();
    let state = scanf!(gen.next()?, "    - Continue with state {}.", char).unwrap();
    Some(Rule {
        write,
        dir: if dir == "left" { Dir::L } else { Dir::R },
        state: state as usize - 'A' as usize,
    })
}

fn parse_state(input: &str) -> Option<[Rule; 2]> {
    let mut gen = input.lines();
    scanf!(gen.next()?, "In state {}:", char).unwrap();
    let rule1 = parse_branch(&mut gen, 0)?;
    let rule2 = parse_branch(&mut gen, 1)?;
    Some([rule1, rule2])
}

fn parse_rules(input: &str) -> (usize, usize, Vec<[Rule; 2]>) {
    let mut gen = input.split("\n\n");
    let (start, n) = scanf!(
        gen.next().unwrap(),
        "Begin in state {}.\nPerform a diagnostic checksum after {} steps.",
        char,
        usize
    )
    .unwrap();
    (
        start as usize - 'A' as usize,
        n,
        gen.map(|st| parse_state(st).unwrap()).collect(),
    )
}

pub fn part1(input: &str) -> usize {
    let (mut state, steps, rules) = parse_rules(input);
    let mut tape = VecDeque::new();
    tape.push_back(0);
    let mut i = 0;
    for _ in 0..steps {
        let rule = &rules[state as usize][tape[i]];
        tape[i] = rule.write;
        match rule.dir {
            Dir::L => {
                if i == 0 {
                    tape.push_front(0);
                } else {
                    i -= 1;
                }
            }
            Dir::R => {
                i += 1;
                if i >= tape.len() {
                    tape.push_back(0);
                }
            }
        }
        state = rule.state;
    }
    tape.into_iter().sum()
}

pub fn part2(_input: &str) -> String {
    " ".to_string()
}
