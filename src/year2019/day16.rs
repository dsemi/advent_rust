use crate::utils::parsers::*;
use num::integer::lcm;
use std::cmp::min;

pub fn part1(input: &str) -> String {
    let mut ns: Vec<i64> = input.chars().map(|x| x.to_digit(10).unwrap() as i64).collect();
    for _ in 0..100 {
        ns = (0..ns.len())
            .map(|n| {
                let pos = (n..ns.len())
                    .step_by((n + 1) * 4)
                    .map(|i| {
                        let end = min(ns.len(), i + n + 1);
                        ns[i..end].iter().sum::<i64>()
                    })
                    .sum::<i64>();
                let neg = (n + (n + 1) * 2..ns.len())
                    .step_by((n + 1) * 4)
                    .map(|i| {
                        let end = min(ns.len(), i + n + 1);
                        ns[i..end].iter().sum::<i64>()
                    })
                    .sum::<i64>();
                (pos - neg).abs() % 10
            })
            .collect();
    }
    ns[..8].iter().map(|x| std::char::from_digit(*x as u32, 10).unwrap()).collect()
}

const PASCAL_PERIOD: usize = 16000;
const DIAG: [i32; PASCAL_PERIOD] = {
    let mut pascal = [0; PASCAL_PERIOD];
    let mut i = 0;
    let mut v = 1;
    while i < PASCAL_PERIOD {
        pascal[i] = v;
        i += 1;
        v = (v + 1) % 10;
    }
    let mut p = 2;
    while p < 100 {
        pascal[0] = 1;
        let mut index = 1;
        while index < PASCAL_PERIOD {
            pascal[index] = (pascal[index - 1] + pascal[index]) % 10;
            index += 1;
        }
        p += 1;
    }
    pascal
};

pub fn part2(input: &str) -> String {
    let offset = input[..7].usize();
    let ns: Vec<i32> = input.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    assert!(offset > ns.len() * 10000 / 2, "Offset is not large enough");

    let n_len = ns.len();
    let ds: Vec<i32> = ns.into_iter().cycle().skip(offset % n_len).take(n_len).collect();
    let joint_cycle = lcm(PASCAL_PERIOD, n_len);
    let tot_len = n_len * 10000 - offset;
    let num_cycles = tot_len / joint_cycle;

    (0..8)
        .map(|i| {
            let sum_first: i32 = ds
                .iter()
                .cycle()
                .skip(i)
                .take(joint_cycle)
                .enumerate()
                .map(|(idx, dig)| DIAG[idx % PASCAL_PERIOD] * *dig)
                .sum();
            let sum_last: i32 = ds
                .iter()
                .cycle()
                .take(tot_len)
                .skip(i + num_cycles * joint_cycle)
                .enumerate()
                .map(|(idx, dig)| DIAG[idx % PASCAL_PERIOD] * *dig)
                .sum();
            ((sum_first * num_cycles as i32 + sum_last) % 10).to_string()
        })
        .collect()
}
