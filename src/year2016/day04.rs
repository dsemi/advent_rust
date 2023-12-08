use crate::utils::parsers::*;
use crate::utils::*;

struct Room<'a> {
    name: &'a str,
    sector_id: i64,
    checksum: &'a str,
}

fn parse_rooms(input: &str) -> impl Iterator<Item = Room<'_>> + '_ {
    input.lines().map(|line| {
        let (name, rest) = line.rsplit_once('-').unwrap();
        let (sector, rest2) = rest.split_once('[').unwrap();
        Room {
            name,
            sector_id: sector.i64(),
            checksum: &rest2[..rest2.len() - 1],
        }
    })
}

impl Room<'_> {
    fn is_real(&self) -> bool {
        let checksum: String = self
            .name
            .replace('-', "")
            .chars()
            .most_common_ordered()
            .into_iter()
            .take(5)
            .map(|x| x.0)
            .collect();
        self.checksum == checksum
    }
}

pub fn part1(input: &str) -> i64 {
    parse_rooms(input)
        .filter(|room| room.is_real())
        .map(|room| room.sector_id)
        .sum()
}

fn rotate(n: i64, c: char) -> char {
    if c == ' ' {
        return '-';
    }
    ((c as i64 - n - 97).rem_euclid(26) + 97) as u8 as char
}

pub fn part2(input: &str) -> Option<i64> {
    parse_rooms(input)
        .filter(|room| {
            room.name.contains(
                &"northpole"
                    .chars()
                    .map(|x| rotate(room.sector_id, x))
                    .collect::<String>(),
            )
        })
        .map(|room| room.sector_id)
        .next()
}
