fn advance(grid: &mut [u8], quads: &[u8], xor_in: u16, xor_out: u8, dim: usize) {
    let mut prev = [0_u8; 100];
    for r in 0..dim {
        let mut quad = 0_u16;
        for c in 0..dim {
            let p = 100 * r + c;
            quad = (quad & 0x3333) << 2;
            quad |= ((prev[c] as u16) << 8) | grid[p] as u16;
            prev[c] = grid[p];
            grid[p] = quads[(quad ^ xor_in) as usize] ^ xor_out;
        }
    }
}

fn run(input: &str, times: usize) -> u32 {
    let (iea, img) = input.split_once("\n\n").unwrap();
    let mut rules = [0_u8; 2048];
    let mut idx = 0;
    for c in iea.bytes() {
        rules[idx] = c & 1;
        idx = (idx + 0x88 + 1) & 0x777;
    }
    let mut quads = [0_u8; 65536];
    for i in 0..quads.len() {
        let mut quad = 0;
        quad |= rules[(i >> 5) & 0x777] << 5;
        quad |= rules[(i >> 4) & 0x777] << 4;
        quad |= rules[(i >> 1) & 0x777] << 1;
        quad |= rules[(i >> 0) & 0x777] << 0;
        quads[i] = quad;
    }
    let inp = img.as_bytes();
    let mut grid = [0_u8; 10000];
    idx = 0;
    for r in 0..50 {
        for c in 0..50 {
            let mut quad = 0;
            quad |= (inp[idx + 0] & 1) << 5;
            quad |= (inp[idx + 1] & 1) << 4;
            quad |= (inp[idx + 101] & 1) << 1;
            quad |= (inp[idx + 102] & 1) << 0;
            grid[100 * r + c] = quad;
            idx += 2;
        }
        idx += 102;
    }
    let xor_in = if quads[0] != 0 { 0xffff } else { 0 };
    let xor_out = if quads[0] != 0 { 0x33 } else { 0 };
    for step in (0..times).step_by(2) {
        advance(&mut grid, &quads, 0, xor_out, 50 + (step + 1));
        advance(&mut grid, &quads, xor_in, 0, 50 + (step + 2));
    }
    grid.into_iter().map(|x| x.count_ones()).sum()
}

pub fn part1(input: &str) -> u32 {
    run(input, 2)
}

pub fn part2(input: &str) -> u32 {
    run(input, 50)
}
