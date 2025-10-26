use hybrid_array::Array;
use hybrid_array::sizes::U16;
use md5::{Digest, Md5};
use rayon::prelude::*;

const CHUNK_SIZE: usize = 64_000;

fn hashes<F, F2, T>(input: &str, pred: F, transform: F2) -> impl Iterator<Item = T>
where
    F: Fn(&Array<u8, U16>) -> bool + Copy + Send + Sync,
    F2: Fn(Array<u8, U16>) -> T + Copy + Send + Sync,
    T: Send,
{
    let mut hasher = Md5::new();
    hasher.update(input);
    (0..).step_by(CHUNK_SIZE).flat_map(move |n| {
        (n..n + CHUNK_SIZE)
            .into_par_iter()
            .map(|i| hasher.clone().chain_update(i.to_string()).finalize())
            .filter(pred)
            .map(transform)
            .collect::<Vec<_>>()
    })
}

pub fn part1(input: &str) -> String {
    hashes(
        input,
        |output| output[0] == 0 && output[1] == 0 && output[2] < 16,
        |output| std::char::from_digit(output[2] as u32, 16).unwrap(),
    )
    .take(8)
    .collect()
}

pub fn part2(input: &str) -> String {
    hashes(
        input,
        |output| output[0] == 0 && output[1] == 0 && output[2] < 8,
        |output| (output[2] as usize, std::char::from_digit((output[3] >> 4) as u32, 16).unwrap()),
    )
    .try_fold(([' '; 8], 0), |(mut arr, mut i), (n, c)| {
        if arr[n] == ' ' {
            arr[n] = c;
            i += 1;
            if i == arr.len() {
                return Err(arr.iter().collect());
            }
        }
        Ok((arr, i))
    })
    .unwrap_err()
}
