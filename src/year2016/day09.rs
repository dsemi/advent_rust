use lazy_static::lazy_static;
use regex::Regex;

struct Marker {
    data_len: usize,
    repeat: usize,
    marker_len: usize,
}

fn parse_marker(input: &str) -> Option<Marker> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\((\d+)x(\d+)\)").unwrap();
    }
    RE.captures(input).map(|cap| Marker {
        data_len: cap[1].parse().unwrap(),
        repeat: cap[2].parse().unwrap(),
        marker_len: cap[0].len(),
    })
}

fn decompressed_len(f: fn(&str) -> usize, input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }
    if let Some(marker) = parse_marker(input) {
        let tot_len = marker.marker_len + marker.data_len;
        let repeated_chars = &input[marker.marker_len..tot_len];
        marker.repeat * f(repeated_chars) + decompressed_len(f, &input[tot_len..])
    } else {
        1 + decompressed_len(f, &input[1..])
    }
}

pub fn part1(input: &str) -> usize {
    decompressed_len(|x| x.len(), input)
}

pub fn part2(input: &str) -> usize {
    decompressed_len(|x| part2(x), input)
}
