use crate::utils::parsers::*;
use crate::utils::*;
use num_rational::Ratio;

fn machine(i: &mut &str) -> PResult<[C<i64>; 3]> {
    let (_, ax, _, ay) = ("Button A: X+", i64, ", Y+", i64).parse_next(i)?;
    let (_, bx, _, by) = ("\nButton B: X+", i64, ", Y+", i64).parse_next(i)?;
    let (_, tx, _, ty) = ("\nPrize: X=", i64, ", Y=", i64).parse_next(i)?;
    Ok([C(ax, ay), C(bx, by), C(tx, ty)])
}

fn solve([C(ax, ay), C(bx, by), C(tx, ty)]: [C<i64>; 3]) -> Option<i64> {
    let nb = Ratio::new(ay * tx - ax * ty, ay * bx - ax * by);
    let nb = nb.is_integer().then(|| nb.to_integer())?;
    let na = Ratio::new(tx - nb * bx, ax);
    let na = na.is_integer().then(|| na.to_integer())?;
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
