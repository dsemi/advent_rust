use hashbrown::HashMap;

#[derive(Default)]
struct Letter {
    towel: bool,
    next: [usize; 26],
}

fn dfs<'a>(cache: &mut HashMap<&'a [u8], usize>, trie: &[Letter], pattern: &'a [u8]) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(&v) = cache.get(pattern) {
        return v;
    }
    let combos = (0..pattern.len())
        .scan(0, |i, idx| {
            *i = trie[*i].next[(pattern[idx] - b'a') as usize];
            Some((*i, idx))
        })
        .take_while(|&(i, _)| i > 0)
        .filter(|&(i, _)| trie[i].towel)
        .map(|(_, idx)| dfs(cache, trie, &pattern[idx + 1..]))
        .sum();
    cache.insert(pattern, combos);
    combos
}

fn solve(input: &str) -> impl Iterator<Item = usize> + use<'_> {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let mut trie = vec![Letter::default()];
    for towel in towels.split(", ") {
        let i = towel.bytes().map(|b| (b - b'a') as usize).fold(0, |i, j| {
            if trie[i].next[j] == 0 {
                trie[i].next[j] = trie.len();
                trie.push(Letter::default());
            }
            trie[i].next[j]
        });
        trie[i].towel = true;
    }
    let mut cache = HashMap::new();
    patterns.lines().map(move |pattern| dfs(&mut cache, &trie, pattern.as_bytes()))
}

pub fn part1(input: &str) -> usize {
    solve(input).filter(|&n| n > 0).count()
}

pub fn part2(input: &str) -> usize {
    solve(input).sum()
}
