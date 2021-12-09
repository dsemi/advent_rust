use super::intcode;
use regex::Regex;

const INSTRS: &str = "north\n\
                      east\n\
                      take astrolabe\n\
                      south\n\
                      take space law space brochure\n\
                      north\n\
                      west\n\
                      north\n\
                      north\n\
                      north\n\
                      north\n\
                      take weather machine\n\
                      north\n\
                      take antenna\n\
                      west\n\
                      south\n";

pub fn part1(input: &str) -> String {
    let mut prog = intcode::new(input);
    for c in INSTRS.chars() {
        prog.input.push_back(c as i64);
    }
    prog.run();
    let mut s = String::new();
    for v in prog.output.drain(..) {
        s.push(v as u8 as char);
    }
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(&s)
        .last()
        .unwrap()
        .as_str()
        .to_string()
}

pub fn part2(_input: &str) -> String {
    "".to_string()
}
