struct Combos<'a> {
    stack: Vec<(&'a [i32], i32, usize)>,
}

impl<'a> Combos<'a> {
    fn new(input: &'a mut [i32]) -> Self {
        input.sort_unstable_by_key(|k| -k);
        return Self {
            stack: vec![(input, 150, 0)],
        };
    }
}

impl Iterator for Combos<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((containers, nog, len)) = self.stack.pop() {
            if nog == 0 {
                return Some(len);
            }
            if let Some((i, n)) = containers
                .iter()
                .enumerate()
                .skip_while(|(_, &x)| x > nog)
                .next()
            {
                self.stack.push((&containers[i + 1..], nog, len));
                self.stack.push((&containers[i + 1..], nog - n, len + 1));
            }
        }
        None
    }
}

pub fn part1(mut input: Vec<i32>) -> usize {
    Combos::new(&mut input).count()
}

pub fn part2(mut input: Vec<i32>) -> usize {
    let lengths = Combos::new(&mut input).collect::<Vec<_>>();
    let min = lengths.iter().min().unwrap();
    lengths.iter().filter(|&x| x == min).count()
}
