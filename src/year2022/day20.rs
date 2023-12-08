use crate::utils::parsers2::*;
use itertools::Itertools;

const BIN_SIZE: usize = 32;
const CLUSTER_SIZE: usize = 16;

#[derive(Clone, Copy, Default)]
struct Item {
    val: i64,
    g_idx: usize,
}

#[derive(Clone, Copy)]
struct Addr {
    bin: usize,
    off: usize,
}

fn mix(input: &str, scale: i64, times: usize) -> i64 {
    let mut bins: Vec<Vec<Item>> = input
        .lines()
        .enumerate()
        .map(|(i, x)| Item {
            val: x.i64() * scale,
            g_idx: i,
        })
        .chunks(BIN_SIZE)
        .into_iter()
        .map(|c| c.collect())
        .collect();
    let mut clusters: Vec<usize> = vec![0; bins.len() / CLUSTER_SIZE + 1];
    for (i, bin) in bins.iter().enumerate() {
        clusters[i / CLUSTER_SIZE] += bin.len();
    }
    let mut addrs: Vec<Addr> = bins
        .iter()
        .enumerate()
        .flat_map(|(i, bin)| {
            bin.iter()
                .enumerate()
                .map(move |x| Addr { bin: i, off: x.0 })
        })
        .collect();
    let m = addrs.len() as i64 - 1;
    for _ in 0..times {
        for k in 0..addrs.len() {
            let a = addrs[k];
            let x = bins[a.bin][a.off];
            for i in a.off..bins[a.bin].len() - 1 {
                bins[a.bin][i] = bins[a.bin][i + 1];
                addrs[bins[a.bin][i].g_idx].off = i;
            }
            bins[a.bin].pop();

            let c_id = a.bin / CLUSTER_SIZE;
            clusters[c_id] -= 1;

            let mut g_idx = a.off;
            g_idx += clusters.iter().take(c_id).sum::<usize>();
            g_idx += (c_id * CLUSTER_SIZE..a.bin)
                .map(|i| bins[i].len())
                .sum::<usize>();
            g_idx = (g_idx as i64 + x.val).rem_euclid(m) as usize;
            let (mut bin, mut off) = (0, 0);
            while off + clusters[bin / CLUSTER_SIZE] <= g_idx {
                off += clusters[bin / CLUSTER_SIZE];
                bin += CLUSTER_SIZE;
            }
            while off + bins[bin].len() <= g_idx {
                off += bins[bin].len();
                bin += 1;
            }
            off = g_idx - off;
            let c_id = bin / CLUSTER_SIZE;
            clusters[c_id] += 1;

            bins[bin].push(Default::default());
            for i in (off + 1..bins[bin].len()).rev() {
                bins[bin][i] = bins[bin][i - 1];
                addrs[bins[bin][i].g_idx].off = i;
            }
            bins[bin][off] = x;
            addrs[k] = Addr { bin, off };
        }
        bins = bins
            .into_iter()
            .flatten()
            .enumerate()
            .inspect(|(i, x)| {
                addrs[x.g_idx] = Addr {
                    bin: i / BIN_SIZE,
                    off: i % BIN_SIZE,
                }
            })
            .map(|x| x.1)
            .chunks(BIN_SIZE)
            .into_iter()
            .map(|c| c.collect())
            .collect();
        clusters.iter_mut().for_each(|v| *v = 0);
        for (i, bin) in bins.iter().enumerate() {
            clusters[i / CLUSTER_SIZE] += bin.len();
        }
    }
    let flat: Vec<Item> = bins.into_iter().flatten().collect();
    let z_idx = flat.iter().position(|x| x.val == 0).unwrap();
    flat[(z_idx + 1000) % flat.len()].val
        + flat[(z_idx + 2000) % flat.len()].val
        + flat[(z_idx + 3000) % flat.len()].val
}

pub fn part1(input: &str) -> i64 {
    mix(input, 1, 1)
}

pub fn part2(input: &str) -> i64 {
    mix(input, 811589153, 10)
}
