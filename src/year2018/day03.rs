use crate::utils::parsers::*;

struct Claim {
    num: usize,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

fn claim(i: &mut &str) -> Result<Claim> {
    let num = delimited('#', usize, " @ ").parse_next(i)?;
    let (x, y) = terminated(coord(usize), ": ").parse_next(i)?;
    let (w, h) = sep2(usize, 'x').parse_next(i)?;
    Ok(Claim { num, x0: x, y0: y, x1: x + w, y1: y + h })
}

fn coord_freq(claims: &[Claim]) -> Vec<Vec<usize>> {
    let max_x = claims.iter().map(|c| c.x1).max().unwrap();
    let max_y = claims.iter().map(|c| c.y1).max().unwrap();
    let mut result = vec![vec![0; max_y + 1]; max_x + 1];
    for claim in claims {
        for row in result.iter_mut().take(claim.x1).skip(claim.x0) {
            for e in row.iter_mut().take(claim.y1).skip(claim.y0) {
                *e += 1;
            }
        }
    }
    result
}

pub fn part1(input: &str) -> usize {
    coord_freq(&lines(claim).read(input))
        .into_iter()
        .map(|col| col.into_iter().filter(|&x| x > 1).count())
        .sum()
}

pub fn part2(input: &str) -> Option<usize> {
    let claims = lines(claim).read(input);
    let grid = coord_freq(&claims);
    claims.into_iter().find_map(|claim| {
        (claim.x0..claim.x1)
            .all(|x| (claim.y0..claim.y1).all(|y| grid[x][y] == 1))
            .then_some(claim.num)
    })
}
