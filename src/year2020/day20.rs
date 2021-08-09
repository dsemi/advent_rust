use ahash::AHashMap;

type Row = Vec<bool>;
type Grid = Vec<Row>;

#[derive(Clone)]
struct Tile {
    num: u64,
    grid: Grid,
}

fn parse_tiles(s: &str) -> Vec<Tile> {
    s.split("\n\n")
        .map(|x| {
            let mut gen = x.lines();
            let n: u64 = gen
                .next()
                .unwrap()
                .split(&[' ', ':'][..])
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let grid = gen.map(|x| x.chars().map(|x| x == '#').collect()).collect();
            Tile { num: n, grid: grid }
        })
        .collect()
}

fn transpose(t: &mut Grid) {
    for i in 0..t.len() {
        for j in i + 1..t.len() {
            let tmp = t[i][j];
            t[i][j] = t[j][i];
            t[j][i] = tmp;
        }
    }
}

fn orientations(tile: &Grid) -> Vec<Grid> {
    let mut t = tile.clone();
    let mut v = Vec::new();
    for _ in 0..4 {
        v.push(t.clone());
        transpose(&mut t);
        v.push(t.clone());
        t.reverse();
    }
    assert!(&t == tile);
    v
}

fn find_corners(tiles: &Vec<Tile>) -> (Vec<u64>, AHashMap<Row, Vec<Tile>>) {
    let mut m = AHashMap::new();
    for tile in tiles {
        for t in orientations(&tile.grid) {
            let hash: Row = t.iter().map(|row| row[0]).collect();
            let e = m.entry(hash).or_insert(vec![]);
            e.push(Tile {
                num: tile.num,
                grid: t,
            });
        }
    }
    let mut m2 = AHashMap::new();
    for v in m.values() {
        assert!(v.len() <= 2);
        if v.len() == 1 {
            let e = m2.entry(v[0].num).or_insert(0);
            *e += 1;
        }
    }
    (
        m2.into_iter()
            .filter(|&(_, v)| v == 4)
            .map(|(k, _)| k)
            .collect(),
        m,
    )
}

pub fn part1(input: &str) -> u64 {
    let tiles = parse_tiles(input);
    let corners = find_corners(&tiles).0;
    corners.into_iter().product()
}

fn place_tiles(tiles: Vec<Tile>) -> (Vec<Tile>, usize) {
    let size = (tiles.len() as f64).sqrt().floor() as usize;
    let mut grid: Vec<Tile> = Vec::new();
    let (corners, m) = find_corners(&tiles);
    let mut start = tiles
        .into_iter()
        .filter(|x| corners.iter().any(|n| x.num == *n))
        .next()
        .unwrap();
    while m[&start
        .grid
        .iter()
        .map(|row| row[row.len() - 1])
        .collect::<Row>()]
        .len()
        < 2
        || m[&start.grid[start.grid.len() - 1]].len() < 2
    {
        transpose(&mut start.grid);
        start.grid.reverse();
    }
    for r in 0..size {
        for c in 0..size {
            grid.push(if r == 0 && c == 0 {
                start.clone()
            } else if c == 0 {
                let prev = &grid[(r - 1) * size + c];
                m[&prev.grid[prev.grid.len() - 1]]
                    .iter()
                    .filter(|t| t.num != prev.num)
                    .map(|t| {
                        let mut t2 = t.clone();
                        transpose(&mut t2.grid);
                        t2
                    })
                    .next()
                    .unwrap()
            } else {
                let prev = &grid[r * size + c - 1];
                m[&prev
                    .grid
                    .iter()
                    .map(|row| row[row.len() - 1])
                    .collect::<Row>()]
                    .iter()
                    .filter(|t| t.num != prev.num)
                    .cloned()
                    .next()
                    .unwrap()
            });
        }
    }
    (grid, size)
}

fn find_sea_monsters(pic: &Vec<u128>) -> u32 {
    let mons = [
        0b00000000000000000010,
        0b10000110000110000111,
        0b01001001001001001000,
    ];
    let cnt: u32 = mons.iter().map(|&x: &u128| x.count_ones()).sum();
    pic.windows(3)
        .map(|wind| {
            let mut rs: Vec<u128> = wind.iter().map(|x| *x).collect();
            let mut tot = 0;
            while rs.iter().any(|x| x > &0) {
                if rs.iter().zip(&mons).all(|(a, b)| a & b == *b) {
                    tot += cnt;
                }
                for i in 0..rs.len() {
                    rs[i] >>= 1;
                }
            }
            tot
        })
        .sum()
}

pub fn part2(input: &str) -> Option<u32> {
    let (mut grid, size) = place_tiles(parse_tiles(input));
    let mut inner_size = 0;
    for tile in grid.iter_mut() {
        tile.grid = tile.grid[1..tile.grid.len() - 1]
            .into_iter()
            .map(|row| row[1..row.len() - 1].into_iter().copied().collect())
            .collect();
        inner_size = tile.grid.len();
    }

    let mut pic = Vec::new();
    for chunk in grid.chunks(size) {
        for row in 0..inner_size {
            let mut r = Vec::new();
            for t in chunk {
                r.extend(t.grid[row].clone());
            }
            pic.push(r);
        }
    }

    for p in orientations(&pic) {
        let p2 = p
            .into_iter()
            .map(|row| {
                row.iter()
                    .fold(0 as u128, |acc, x| (acc << 1) + (*x as u128))
            })
            .collect();
        let ms = find_sea_monsters(&p2);
        if ms != 0 {
            let tot: u32 = p2.into_iter().map(|row| row.count_ones()).sum();
            return Some(tot - ms);
        }
    }
    None
}
