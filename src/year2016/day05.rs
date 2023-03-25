use generic_array::GenericArray;
use md5::{Digest, Md5};
use rayon::prelude::*;

const CHUNK_SIZE: usize = 64_000;

pub fn part1(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    (0..)
        .step_by(CHUNK_SIZE)
        .flat_map(|n| {
            (n..n + CHUNK_SIZE)
                .into_par_iter()
                .filter_map(|i| {
                    let mut h = hasher.clone();
                    h.update(&i.to_string());
                    let mut output = GenericArray::default();
                    h.finalize_into(&mut output);
                    (output[0] == 0 && output[1] == 0 && output[2] < 16)
                        .then(|| std::char::from_digit(output[2] as u32, 16).unwrap())
                })
                .collect::<Vec<_>>()
        })
        .take(8)
        .collect()
}

pub fn part2(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    (0..)
        .step_by(CHUNK_SIZE)
        .flat_map(|n| {
            (n..n + CHUNK_SIZE)
                .into_par_iter()
                .filter_map(|i| {
                    let mut h = hasher.clone();
                    h.update(&i.to_string());
                    let mut output = GenericArray::default();
                    h.finalize_into(&mut output);
                    (output[0] == 0 && output[1] == 0 && output[2] < 8).then(|| {
                        (
                            output[2] as usize,
                            std::char::from_digit((output[3] >> 4) as u32, 16).unwrap(),
                        )
                    })
                })
                .collect::<Vec<_>>()
        })
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
