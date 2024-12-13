use crate::utils::parsers::*;
use crate::utils::*;

fn machine(i: &mut &str) -> PResult<[C<i64>; 3]> {
    let (_, ax, _, ay) = ("Button A: X+", i64, ", Y+", i64).parse_next(i)?;
    let (_, bx, _, by) = ("\nButton B: X+", i64, ", Y+", i64).parse_next(i)?;
    let (_, tx, _, ty) = ("\nPrize: X=", i64, ", Y=", i64).parse_next(i)?;
    Ok([C(ax, ay), C(bx, by), C(tx, ty)])
}

fn solve([C(ax, ay), C(bx, by), C(tx, ty)]: [C<i64>; 3]) -> Option<i64> {
    let nb = (ay * tx - ax * ty) / (ay * bx - ax * by);
    let na = (tx - nb * bx) / ax;
    (na * ax + nb * bx == tx && na * ay + nb * by == ty).then_some(3 * na + nb)
}

pub fn part1(input: &str) -> i64 {
    input.split("\n\n").map(|line| machine.read(line)).filter_map(solve).sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|line| machine.read(line))
        .map(|[a, b, t]| [a, b, t + C(10000000000000, 10000000000000)])
        .filter_map(solve)
        .sum()
}
