use hashbrown::HashMap;

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<bool>) {
    let mut m = HashMap::new();
    m.insert("start", 0);
    m.insert("end", 1);
    let mut v = vec![vec![], vec![]];
    let mut l = vec![true, true];
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let ai = *m.entry(a).or_insert_with(|| {
            v.push(vec![]);
            l.push(a.chars().all(|c| c.is_lowercase()));
            v.len() - 1
        });
        let bi = *m.entry(b).or_insert_with(|| {
            v.push(vec![]);
            l.push(b.chars().all(|c| c.is_lowercase()));
            v.len() - 1
        });
        v[ai].push(bi);
        v[bi].push(ai);
    }
    (v, l)
}

fn dfs(vis: &mut [u8], l: &[bool], m: &[Vec<usize>], k: usize, mut double: bool) -> usize {
    if k == 1 {
        return 1;
    } else if l[k] && vis[k] > 0 {
        if double || k == 0 {
            return 0;
        }
        double = true;
    }
    vis[k] += 1;
    let sum = m[k].iter().map(|&child| dfs(vis, l, m, child, double)).sum();
    vis[k] -= 1;
    sum
}

pub fn part1(input: &str) -> usize {
    let (v, l) = parse(input);
    dfs(&mut vec![0; v.len()], &l, &v, 0, true)
}

pub fn part2(input: &str) -> usize {
    let (v, l) = parse(input);
    dfs(&mut vec![0; v.len()], &l, &v, 0, false)
}
