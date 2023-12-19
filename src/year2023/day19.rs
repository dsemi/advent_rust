use crate::utils::parsers::*;
use crate::utils::*;
use ahash::AHashMap;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use Label::*;

#[derive(Copy, Clone)]
enum Label<'a> {
    Accept,
    Reject,
    Named(&'a str),
}

#[derive(Clone)]
enum Rule<'a> {
    Label(Label<'a>),
    Cond(usize, Ordering, i64, Label<'a>),
}

fn cond<'a>(i: &mut &'a str) -> PResult<Rule<'a>> {
    let (idx, o, b, label) = (
        alt(('x'.value(0), 'm'.value(1), 'a'.value(2), 's'.value(3))),
        alt(('<'.value(Less), '>'.value(Greater))),
        terminated(i64, ':'),
        alt(('A'.value(Accept), 'R'.value(Reject), alpha1.map(Named))),
    )
        .parse_next(i)?;
    Ok(Rule::Cond(idx, o, b, label))
}

fn rule<'a>(i: &mut &'a str) -> PResult<Rule<'a>> {
    alt((
        cond,
        'A'.value(Rule::Label(Accept)),
        'R'.value(Rule::Label(Reject)),
        alpha1.map(|n| Rule::Label(Named(n))),
    ))
    .parse_next(i)
}

fn workflow<'a>(i: &mut &'a str) -> PResult<(&'a str, Vec<Rule<'a>>)> {
    (alpha1, delimited('{', list(rule), '}')).parse_next(i)
}

fn part(i: &mut &str) -> PResult<Vec<i64>> {
    delimited('{', list(preceded((any, '='), i64)), '}').parse_next(i)
}

fn accepted<'a>(workflows: &AHashMap<&'a str, Vec<Rule<'a>>>, part: &[i64]) -> bool {
    let mut k = "in";
    loop {
        for rule in workflows[k].iter() {
            match rule {
                Rule::Cond(i, o, b, _) if part[*i].cmp(b) != *o => (),
                Rule::Label(lbl) | Rule::Cond(_, _, _, lbl) => match lbl {
                    Accept => return true,
                    Reject => return false,
                    Named(l) => {
                        k = l;
                        break;
                    }
                },
            }
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows: AHashMap<_, _> = workflows.lines().map(|line| workflow.read(line)).collect();
    parts
        .lines()
        .map(|line| part.read(line))
        .filter(|part| accepted(&workflows, part))
        .map(|part| part.into_iter().sum::<i64>())
        .sum()
}

fn valid_parts<'a>(
    workflows: &AHashMap<&'a str, Vec<Rule<'a>>>,
    workflow: &[Rule],
) -> Vec<[Interval; 4]> {
    let rule = &workflow[0];
    match rule {
        Rule::Label(Accept) => vec![[Interval::new(1, 4001); 4]],
        Rule::Label(Reject) => vec![],
        Rule::Label(Named(lbl)) => valid_parts(workflows, &workflows[lbl]),
        &Rule::Cond(i, o, b, lbl) => valid_parts(workflows, &[Rule::Label(lbl)])
            .into_iter()
            .filter_map(|mut part| {
                if o == Less {
                    part[i].clamp_hi(b);
                } else {
                    part[i].clamp_lo(b + 1);
                }
                part[i].valid().then_some(part)
            })
            .chain(
                valid_parts(workflows, &workflow[1..])
                    .into_iter()
                    .filter_map(|mut part| {
                        if o == Less {
                            part[i].clamp_lo(b);
                        } else {
                            part[i].clamp_hi(b + 1);
                        }
                        part[i].valid().then_some(part)
                    }),
            )
            .collect(),
    }
}

pub fn part2(input: &str) -> i64 {
    let workflows = input.split_once("\n\n").unwrap().0;
    let workflows: AHashMap<_, _> = workflows.lines().map(|line| workflow.read(line)).collect();
    valid_parts(&workflows, &workflows["in"])
        .into_iter()
        .map(|part| part.into_iter().map(|int| int.len()).product::<i64>())
        .sum()
}
