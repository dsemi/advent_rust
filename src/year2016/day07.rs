struct Ip<'a> {
    supernets: Vec<&'a str>,
    hypernets: Vec<&'a str>,
}

fn ips<'a>(input: &'a str) -> impl Iterator<Item = Ip<'a>> + '_ {
    input.lines().map(|line| {
        let mut ip = Ip {
            supernets: Vec::new(),
            hypernets: Vec::new(),
        };
        for (i, part) in line.split(&['[', ']'][..]).enumerate() {
            if i % 2 == 0 {
                ip.supernets.push(part);
            } else {
                ip.hypernets.push(part);
            }
        }
        ip
    })
}

fn has_abba(s: &[u8]) -> bool {
    (0..s.len() - 3).any(|i| s[i] != s[i + 1] && s[i] == s[i + 3] && s[i + 1] == s[i + 2])
}

pub fn part1(input: &str) -> usize {
    ips(input)
        .filter(|ip| {
            ip.supernets.iter().any(|s| has_abba(s.as_bytes()))
                && !ip.hypernets.iter().any(|h| has_abba(h.as_bytes()))
        })
        .count()
}

fn abas(s: &[u8]) -> impl Iterator<Item = (u8, u8)> + '_ {
    (0..s.len() - 2)
        .filter_map(move |i| (s[i] != s[i + 1] && s[i] == s[i + 2]).then(|| (s[i], s[i + 1])))
}

pub fn part2(input: &str) -> usize {
    ips(input)
        .filter(|ip| {
            ip.supernets.iter().any(|s| {
                abas(s.as_bytes()).any(|(a, b)| {
                    let bab = [b, a, b];
                    let babstr = std::str::from_utf8(&bab).unwrap();
                    ip.hypernets.iter().any(|h| h.contains(&babstr))
                })
            })
        })
        .count()
}
