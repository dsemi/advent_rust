use super::intcode;

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
    prog.output
        .into_iter()
        .map(|x| x as u8 as char)
        .filter(char::is_ascii_digit)
        .collect()
}

pub fn part2(_input: &str) -> String {
    " ".to_string()
}
