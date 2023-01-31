use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use nom::ToUsize;

pub fn usize(input: &str) -> IResult<&str, usize> {
    u32(input).map(|(i, x)| (i, x.to_usize()))
}

fn marker(i: &str) -> IResult<&str, (usize, usize)> {
    delimited(tag("("), separated_pair(usize, tag("x"), usize), tag(")"))(i)
}

fn decompressed_len(f: fn(&str) -> usize, input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }
    if let Ok((rest, (data_len, repeat))) = marker(input) {
        repeat * f(&rest[..data_len]) + decompressed_len(f, &rest[data_len..])
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
