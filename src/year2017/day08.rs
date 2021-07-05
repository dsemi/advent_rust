use std::collections::HashMap;

fn run_cmd<'a>(reg: &mut HashMap<&'a str, i64>, line: &'a str) -> i64 {
    match line.split_whitespace().collect::<Vec<_>>()[..] {
        [r, op, n, "if", r2, cond, n2] => {
            let cmp_fn: fn(i64, i64) -> bool = match cond {
                "==" => |a, b| a == b,
                "!=" => |a, b| a != b,
                ">" => |a, b| a > b,
                ">=" => |a, b| a >= b,
                "<" => |a, b| a < b,
                "<=" => |a, b| a <= b,
                _ => panic!("Parse cond error: {}", cond),
            };
            if cmp_fn(*reg.get(r2).unwrap_or(&0), n2.parse().unwrap()) {
                let e = reg.entry(r).or_insert(0);
                *e += (if op == "inc" { 1 } else { -1 }) * n.parse::<i64>().unwrap();
            }
            *reg.get(r).unwrap_or(&0)
        }
        _ => panic!("Parse error: {}", line),
    }
}

pub fn part1(input: &str) -> Option<i64> {
    let mut tbl = HashMap::new();
    input.lines().for_each(|line| {
        run_cmd(&mut tbl, line);
    });
    tbl.values().copied().max()
}

pub fn part2(input: &str) -> Option<i64> {
    let mut tbl = HashMap::new();
    input.lines().map(|line| run_cmd(&mut tbl, line)).max()
}
