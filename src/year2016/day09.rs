use crate::utils::parsers::*;

fn marker(i: &str) -> ModalResult<(&str, (usize, usize))> {
    delimited('(', sep2(usize, 'x'), ')').parse_peek(i)
}

fn decompressed_len(f: fn(&str) -> usize, input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }
    if let Ok((rem, (data_len, repeat))) = marker(input) {
        repeat * f(&rem[..data_len]) + decompressed_len(f, &rem[data_len..])
    } else {
        1 + decompressed_len(f, &input[1..])
    }
}

pub fn part1(input: &str) -> usize {
    decompressed_len(|x| x.len(), input)
}

pub fn part2(input: &str) -> usize {
    decompressed_len(part2, input)
}
