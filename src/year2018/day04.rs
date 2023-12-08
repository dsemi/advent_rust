use crate::utils::parsers2::*;
use ahash::AHashMap;
use itertools::Itertools;
use Record::*;

enum Record {
    GuardChange(usize),
    SleepToggle(usize),
}

fn parse_records(input: &str) -> impl Iterator<Item = Record> + '_ {
    input.lines().sorted().map(|line| {
        let (time, rest) = line.split_once("] ").unwrap();
        if rest.starts_with("Guard") {
            GuardChange(
                rest.chars()
                    .filter(char::is_ascii_digit)
                    .collect::<String>()
                    .usize(),
            )
        } else {
            SleepToggle(time[time.rfind(':').unwrap() + 1..].usize())
        }
    })
}

fn guard_sleep_freqs<I>(records: I) -> AHashMap<usize, Vec<usize>>
where
    I: IntoIterator<Item = Record>,
{
    let (mut guard, mut last_m, mut st) = (0, 0, 0);
    let mut result: AHashMap<usize, Vec<usize>> = AHashMap::new();
    for record in records {
        match record {
            GuardChange(guard_num) => {
                if last_m > 0 {
                    for i in last_m..60 {
                        result.get_mut(&guard).unwrap()[i] += st;
                    }
                }
                guard = guard_num;
                st = 0;
                last_m = 0;
                result.entry(guard).or_insert_with(|| vec![0; 60]);
            }
            SleepToggle(minute) => {
                for i in last_m..minute {
                    result.get_mut(&guard).unwrap()[i] += st;
                }
                st ^= 1;
                last_m = minute;
            }
        }
    }
    for i in last_m..60 {
        result.get_mut(&guard).unwrap()[i] += st;
    }
    result
}

pub fn part1(input: &str) -> usize {
    let sleep_freqs = guard_sleep_freqs(parse_records(input));
    let n = sleep_freqs
        .iter()
        .map(|(k, v)| (v.iter().sum::<usize>(), k))
        .max()
        .unwrap()
        .1;
    n * sleep_freqs[n].iter().position_max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let sleep_freqs = guard_sleep_freqs(parse_records(input));
    let m = sleep_freqs
        .iter()
        .map(|(k, v)| v.iter().enumerate().map(|x| (x.1, x.0, k)).max().unwrap())
        .max()
        .unwrap();
    m.1 * m.2
}
