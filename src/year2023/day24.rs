use crate::utils::parsers::*;
use crate::utils::*;
use nalgebra::stack;

fn intersect_2d((ap, av): &(C3<i64>, C3<i64>), (bp, bv): &(C3<i64>, C3<i64>)) -> Option<C<f64>> {
    let C(dx, dy) = bp.xy() - ap.xy();
    let det = bv.0 * av.1 - bv.1 * av.0;
    let u_numer = dy * bv.0 - dx * bv.1;
    let v_numer = dy * av.0 - dx * av.1;
    if det == 0 || u_numer.signum() != det.signum() || v_numer.signum() != det.signum() {
        return None;
    }
    Some(ap.xy().as_() + av.xy().as_() * (u_numer as f64 / det as f64))
}

pub fn part1(input: &str) -> usize {
    let rays = lines(sep2(c3(i64), '@')).read(input);
    const LO: f64 = 200000000000000.0;
    const HI: f64 = 400000000000000.0;
    rays.iter()
        .enumerate()
        .flat_map(|(i, r1)| std::iter::repeat(r1).zip(&rays[i + 1..]))
        .filter_map(|(r1, r2)| {
            intersect_2d(r1, r2).filter(|C(x, y)| (LO..=HI).contains(x) && (LO..=HI).contains(y))
        })
        .count()
}

pub fn part2(input: &str) -> i64 {
    let [(ap, av), (bp, bv), (cp, cv)] = lines_iter(input, sep2(c3(f64).map(C3::vec), '@'))
        .take(3)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let rhs = stack![ bp.cross(&bv) - ap.cross(&av);
                      cp.cross(&cv) - ap.cross(&av) ];
    let m = stack![ av.cross_matrix() - bv.cross_matrix(), bp.cross_matrix() - ap.cross_matrix();
                    av.cross_matrix() - cv.cross_matrix(), cp.cross_matrix() - ap.cross_matrix() ];
    let result = m.try_inverse().unwrap() * rhs;
    (result[0] + result[1] + result[2]) as i64
}
