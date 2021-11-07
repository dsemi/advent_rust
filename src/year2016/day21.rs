use scan_fmt::scan_fmt as scanf;

fn rot_chr_idx(i: usize) -> usize {
    if i >= 4 {
        i + 2
    } else {
        i + 1
    }
}

fn move_p<T>(s: &mut Vec<T>, i: usize, j: usize) {
    let c = s.remove(i);
    s.insert(j, c);
}

fn run_program<'a, I: Iterator<Item = &'a str>>(input: String, instrs: I, invert: bool) -> String {
    let mut mem: Vec<char> = input.chars().collect();
    for line in instrs {
        if let Ok((x, y)) = scanf!(line, "swap position {} with position {}", usize, usize) {
            mem.swap(x, y);
        } else if let Ok((a, b)) = scanf!(line, "swap letter {} with letter {}", char, char) {
            let x = mem.iter().position(|x| *x == a).unwrap();
            let y = mem.iter().position(|x| *x == b).unwrap();
            mem.swap(x, y);
        } else if let Ok((d, x)) = scanf!(line, "rotate {} {}", String, usize) {
            if invert && d == "right" || !invert && d == "left" {
                mem.rotate_left(x);
            } else {
                mem.rotate_right(x);
            }
        } else if let Ok(c) = scanf!(line, "rotate based on position of letter {}", char) {
            if invert {
                for i in 0.. {
                    if rot_chr_idx(mem.iter().position(|x| *x == c).unwrap()) == i {
                        break;
                    }
                    mem.rotate_left(1);
                }
            } else {
                let i = (mem.len() - rot_chr_idx(mem.iter().position(|x| *x == c).unwrap()))
                    .rem_euclid(mem.len());
                mem.rotate_left(i);
            }
        } else if let Ok((x, y)) = scanf!(line, "reverse positions {} through {}", usize, usize) {
            mem[x..=y].reverse();
        } else if let Ok((x, y)) = scanf!(line, "move position {} to position {}", usize, usize) {
            let (i, j) = if invert { (y, x) } else { (x, y) };
            move_p(&mut mem, i, j);
        } else {
            panic!("Parse error: {}", line);
        }
    }
    mem.into_iter().collect()
}

pub fn part1(input: &str) -> String {
    run_program("abcdefgh".to_owned(), input.lines(), false)
}

pub fn part2(input: &str) -> String {
    run_program("fbgdceah".to_owned(), input.lines().rev(), true)
}
