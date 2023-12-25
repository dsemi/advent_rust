use crate::utils::parsers::*;
use std::collections::VecDeque;

#[derive(Clone)]
enum Dir {
    L,
    R,
}

struct Rule {
    write: usize,
    dir: Dir,
    state: usize,
}

fn parse_dir(i: &mut &str) -> PResult<Dir> {
    alt(("left".value(Dir::L), "right".value(Dir::R))).parse_next(i)
}

fn branch(i: &mut &str) -> PResult<Rule> {
    ("  If the current value is ", usize, ":\n").parse_next(i)?;
    let write = delimited("    - Write the value ", usize, ".\n").parse_next(i)?;
    let dir = delimited("    - Move one slot to the ", parse_dir, ".\n").parse_next(i)?;
    let state = delimited("    - Continue with state ", any, '.').parse_next(i)?;
    Ok(Rule {
        write,
        dir,
        state: state as usize - 'A' as usize,
    })
}

fn state(i: &mut &str) -> PResult<[Rule; 2]> {
    ("In state ", any, ":\n").parse_next(i)?;
    let (rule1, rule2) = separated_pair(branch, '\n', branch).parse_next(i)?;
    Ok([rule1, rule2])
}

fn parse_rules(i: &mut &str) -> PResult<(usize, usize, Vec<[Rule; 2]>)> {
    let start = delimited("Begin in state ", any, ".\n").parse_next(i)?;
    let n =
        delimited("Perform a diagnostic checksum after ", usize, " steps.\n\n").parse_next(i)?;
    Ok((
        start as usize - 'A' as usize,
        n,
        separated(1.., state, "\n\n").parse_next(i)?,
    ))
}

pub fn part1(input: &str) -> usize {
    let (mut state, steps, rules) = parse_rules.read(input);
    let mut tape = VecDeque::new();
    tape.push_back(0);
    let mut i = 0;
    for _ in 0..steps {
        let rule = &rules[state][tape[i]];
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

pub fn part2(_input: &str) -> &str {
    " "
}
