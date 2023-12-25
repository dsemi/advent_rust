use crate::utils::parsers::*;
use crate::utils::*;
use nalgebra::{Matrix3, Matrix6, Vector3, Vector6};
use num_traits::Zero;
use std::ops::Neg;

#[derive(Copy, Clone, Debug)]
struct Ray {
    pt: C3<i64>,
    dir: C3<i64>,
}

impl Ray {
    fn intersect_2d(&self, other: &Self) -> Option<C<f64>> {
        let C3(dx, dy, _) = other.pt - self.pt;
        let det = other.dir.0 * self.dir.1 - other.dir.1 * self.dir.0;
        if det == 0 {
            return None;
        }
        let u_numer = dy * other.dir.0 - dx * other.dir.1;
        if u_numer.signum() != det.signum() {
            return None;
        }
        let v_numer = dy * self.dir.0 - dx * self.dir.1;
        if v_numer.signum() != det.signum() {
            return None;
        }
        let pt = C(self.pt.0 as f64, self.pt.1 as f64);
        let dir = C(self.dir.0 as f64, self.dir.1 as f64);
        Some(pt + dir * u_numer as f64 / det as f64)
    }
}

fn ray(i: &mut &str) -> PResult<Ray> {
    let (pt, dir) = sep2(coord3(i64).output_into(), '@').parse_next(i)?;
    Ok(Ray { pt, dir })
}

pub fn part1(input: &str) -> usize {
    let rays = lines(ray).read(input);
    const LO: f64 = 200000000000000.0;
    const HI: f64 = 400000000000000.0;
    let mut ints = 0;
    for (i, r1) in rays.iter().enumerate() {
        for r2 in rays.iter().skip(i + 1) {
            if let Some(C(x, y)) = r1.intersect_2d(r2) {
                if x >= LO && x <= HI && y >= LO && y <= HI {
                    ints += 1;
                }
            }
        }
    }
    ints
}

fn cross_matrix<T: Copy + Neg<Output = T> + Zero>(v: &Vector3<T>) -> Matrix3<T> {
    let z = T::zero();
    Matrix3::new(z, -v[2], v[1], v[2], z, -v[0], -v[1], v[0], z)
}

pub fn part2(input: &str) -> i64 {
    let rays = lines(ray).read(input);
    let stones: Vec<_> = rays
        .into_iter()
        .map(|ray| {
            (
                Vector3::new(ray.pt.0 as f64, ray.pt.1 as f64, ray.pt.2 as f64),
                Vector3::new(ray.dir.0 as f64, ray.dir.1 as f64, ray.dir.2 as f64),
            )
        })
        .collect();
    let mut rhs = Vector6::zeros();
    rhs.view_mut((0, 0), (3, 1))
        .copy_from(&(stones[1].0.cross(&stones[1].1) - stones[0].0.cross(&stones[0].1)));
    rhs.view_mut((3, 0), (3, 1))
        .copy_from(&(stones[2].0.cross(&stones[2].1) - stones[0].0.cross(&stones[0].1)));
    let mut m = Matrix6::zeros();
    m.view_mut((0, 0), (3, 3))
        .copy_from(&(cross_matrix(&stones[0].1) - cross_matrix(&stones[1].1)));
    m.view_mut((3, 0), (3, 3))
        .copy_from(&(cross_matrix(&stones[0].1) - cross_matrix(&stones[2].1)));
    m.view_mut((0, 3), (3, 3))
        .copy_from(&(cross_matrix(&stones[1].0) - cross_matrix(&stones[0].0)));
    m.view_mut((3, 3), (3, 3))
        .copy_from(&(cross_matrix(&stones[2].0) - cross_matrix(&stones[0].0)));
    let result = m.try_inverse().unwrap() * rhs;
    (result[0] + result[1] + result[2]) as i64
}
