use super::intcode;
use rayon::prelude::*;

pub fn part1(input: &str) -> i64 {
    intcode::new(input).run_no_io(12, 2)
}

pub fn part2(input: &str) -> Option<i64> {
    let prog = intcode::new(input);
    (0..100).find_map(|noun| {
        (0..100).into_par_iter().find_map_any(|verb| {
            (prog.clone().run_no_io(noun, verb) == 19690720).then(|| 100 * noun + verb)
        })
    })
}
