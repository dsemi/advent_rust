use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

fn parse_ings(s: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    s.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" (contains ").collect();
            let (ingredients, allergens) = (parts[0], parts[1]);
            (
                ingredients.split(" ").collect(),
                allergens[..allergens.len() - 1].split(", ").collect(),
            )
        })
        .collect()
}

fn allergens<'a>(foods: Vec<(Vec<&'a str>, Vec<&'a str>)>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut m: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();
    for (ings, alls) in foods {
        let ingset: HashSet<&str> = HashSet::from_iter(ings);
        for allergen in alls {
            let e = m.entry(allergen).or_insert(ingset.clone());
            e.retain(|x| ingset.contains(x));
        }
    }
    m
}

pub fn part1(input: &str) -> usize {
    let foods = parse_ings(input);
    let alls = allergens(foods.clone());
    let ingredients: Vec<&str> = foods.into_iter().flat_map(|x| x.0).collect();
    let mut safe: HashSet<&str> = HashSet::from_iter(ingredients.clone());
    for v in alls.values() {
        for x in v {
            safe.remove(x);
        }
    }
    ingredients
        .into_iter()
        .filter(|&i| safe.contains(&i))
        .count()
}

pub fn part2(input: &str) -> String {
    let mut alls = allergens(parse_ings(input));
    let mut done = BTreeMap::new();
    while !alls.is_empty() {
        for (k, v) in &alls {
            if v.len() == 1 {
                done.insert(k.clone(), v.iter().next().unwrap().clone());
            }
        }
        let s: HashSet<&str> = done.values().map(|x| *x).collect();
        alls = alls
            .into_iter()
            .filter(|(k, _)| !done.contains_key(k))
            .map(|(k, v)| (k, &v - &s))
            .collect();
    }
    itertools::Itertools::intersperse(done.values().copied(), ",").collect()
}
