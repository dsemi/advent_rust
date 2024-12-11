use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn update(total: &mut usize, id: usize, block: usize, len: usize) {
    *total += id * (len * block + len * (len - 1) / 2);
}

pub fn part1(input: &str) -> usize {
    let bytes: Vec<_> =
        input.bytes().map(|b| (b - b'0') as usize).chain(vec![0; input.len() % 2]).collect();
    let mut segs = bytes.into_iter().tuples().enumerate().collect::<Vec<_>>().into_iter();
    let (mut checksum, mut block) = (0, 0);
    let mut last = (0, 0);
    while let Some((id, (len, mut space))) = segs.next() {
        update(&mut checksum, id, block, len);
        block += len;
        while space > 0 {
            if last.1 == 0 {
                match segs.next_back() {
                    Some((id, (end, _))) => last = (id, end),
                    _ => break,
                }
            }
            let len = space.min(last.1);
            update(&mut checksum, last.0, block, len);
            block += len;
            space -= len;
            last.1 -= len;
        }
    }
    update(&mut checksum, last.0, block, last.1);
    checksum
}

pub fn part2(input: &str) -> usize {
    let mut spaces = vec![BinaryHeap::new(); 10];
    let mut files = Vec::new();
    let mut block = 0;
    for (i, b) in input.bytes().enumerate() {
        let len = (b - b'0') as usize;
        if i % 2 == 0 {
            files.push((block, len));
        } else if len > 0 {
            spaces[len].push(Reverse((block, len)));
        }
        block += len;
    }
    let mut checksum = 0;
    for (id, (mut blk, len)) in files.into_iter().enumerate().rev() {
        let space =
            spaces.iter().skip(len).filter_map(|h| Some(h.peek()?.0).filter(|e| e.0 < blk)).min();
        if let Some((spc_blk, spc_len)) = space {
            blk = spc_blk;
            spaces[spc_len].pop();
            if len < spc_len {
                spaces[spc_len - len].push(Reverse((spc_blk + len, spc_len - len)));
            }
        }
        update(&mut checksum, id, blk, len);
    }
    checksum
}
