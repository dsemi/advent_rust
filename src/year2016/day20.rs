use crate::utils::parsers::*;
use std::cmp::max;
use std::iter::Peekable;

type IpRange = (u64, u64);

struct IpRanges<I> {
    ips: I,
}

impl<I> IpRanges<I> {
    fn new(ips: I) -> IpRanges<Peekable<I>>
    where
        I: Iterator<Item = IpRange>,
    {
        IpRanges { ips: ips.peekable() }
    }
}

impl<I> Iterator for IpRanges<Peekable<I>>
where
    I: Iterator<Item = IpRange>,
{
    type Item = IpRange;

    fn next(&mut self) -> Option<IpRange> {
        let mut curr = self.ips.next()?;
        while self.ips.peek().is_some() && self.ips.peek()?.0 <= curr.1 + 1 {
            curr.1 = max(curr.1, self.ips.next()?.1);
        }
        Some(curr)
    }
}

fn parse_ip_filters(input: &str) -> impl Iterator<Item = IpRange> {
    let mut ips = lines(sep2(u64, '-')).read(input);
    ips.sort_unstable();
    IpRanges::new(ips.into_iter())
}

pub fn part1(input: &str) -> u64 {
    let (a, b) = parse_ip_filters(input).next().unwrap();
    if a > 0 { 0 } else { b + 1 }
}

pub fn part2(input: &str) -> u64 {
    2_u64.pow(32) - parse_ip_filters(input).map(|(a, b)| b - a + 1).sum::<u64>()
}
