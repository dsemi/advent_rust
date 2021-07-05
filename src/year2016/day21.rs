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

fn run_program(input: String, instrs: Vec<&str>, invert: bool) -> String {
    let mut mem: Vec<char> = input.chars().collect();
    for line in instrs {
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["swap", "position", x, "with", "position", y] => {
                mem.swap(x.parse().unwrap(), y.parse().unwrap());
            }
            ["swap", "letter", a, "with", "letter", b] => {
                let x = mem
                    .iter()
                    .position(|x| *x == a.chars().next().unwrap())
                    .unwrap();
                let y = mem
                    .iter()
                    .position(|x| *x == b.chars().next().unwrap())
                    .unwrap();
                mem.swap(x, y);
            }
            ["rotate", "right", x, _] => {
                if invert {
                    mem.rotate_left(x.parse().unwrap());
                } else {
                    mem.rotate_right(x.parse().unwrap());
                }
            }
            ["rotate", "left", x, _] => {
                if invert {
                    mem.rotate_right(x.parse().unwrap());
                } else {
                    mem.rotate_left(x.parse().unwrap());
                }
            }
            ["rotate", "based", "on", "position", "of", "letter", c] => {
                let ch = c.chars().next().unwrap();
                if invert {
                    for i in 0.. {
                        if rot_chr_idx(mem.iter().position(|x| *x == ch).unwrap()) == i {
                            break;
                        }
                        mem.rotate_left(1);
                    }
                } else {
                    let i = (mem.len()
                        - rot_chr_idx(
                            mem.iter()
                                .position(|x| *x == ch)
                                .unwrap(),
                        ))
                    .rem_euclid(mem.len());
                    mem.rotate_left(i);
                }
            }
            ["reverse", "positions", x, "through", y] => {
                mem[x.parse().unwrap()..y.parse::<usize>().unwrap()+1].reverse();
            },
            ["move", "position", x, "to", "position", y] => {
                if invert {
                    move_p(&mut mem, y.parse().unwrap(), x.parse().unwrap());
                } else {
                    move_p(&mut mem, x.parse().unwrap(), y.parse().unwrap());
                }
            }
            _ => panic!("Parse error: {}", line),
        }
    }
    mem.into_iter().collect()
}

pub fn part1(input: &str) -> String {
    run_program("abcdefgh".to_owned(), input.lines().collect(), false)
}

pub fn part2(input: &str) -> String {
    run_program("fbgdceah".to_owned(), input.lines().rev().collect(), true)
}
