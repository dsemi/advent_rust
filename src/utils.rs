use num_complex::Complex;

pub struct Scanl<I, St, F> {
    iter: I,
    f: F,
    state: St,
}
impl<I, St, F> Scanl<I, St, F> {
    pub(super) fn new(iter: I, state: St, f: F) -> Scanl<I, St, F> {
        Scanl { iter, state, f }
    }
}

impl<I, S, F> Iterator for Scanl<I, S, F>
where
    I: Iterator,
    S: Copy,
    F: Fn(S, I::Item) -> S,
{
    type Item = S;

    #[inline]
    fn next(&mut self) -> Option<S> {
        let res = self.state;
        let a = self.iter.next()?;
        self.state = (self.f)(self.state, a);
        Some(res)
    }
}

pub trait ScanlExt: Iterator {
    fn scanl<S, F>(self, initial_state: S, f: F) -> Scanl<Self, S, F>
    where
        Self: Sized,
        F: Fn(S, Self::Item) -> S,
    {
        Scanl::new(self, initial_state, f)
    }
}

impl<I: Iterator> ScanlExt for I {}

pub trait Scanl1Ext: Iterator {
    fn scanl1<F>(mut self: Self, f: F) -> Scanl<Self, Self::Item, F>
    where
        Self: Sized,
        F: Fn(Self::Item, Self::Item) -> Self::Item,
    {
        let initial_state = self.next().unwrap();
        Scanl::new(self, initial_state, f)
    }
}

impl<I: Iterator> Scanl1Ext for I {}


// self.scan(initial_state, |acc: &mut St, x: I::Item| {
// *acc = f(*acc, x);
// Some(*acc)
// })

pub fn unit_dir(c: char) -> Complex<i64> {
    match c {
        '<' => Complex::new(-1, 0),
        '>' => Complex::new(1, 0),
        'v' => Complex::new(0, -1),
        '^' => Complex::new(0, 1),
        _ => panic!("Unknown direction"),
    }
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),* $(,)?) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
