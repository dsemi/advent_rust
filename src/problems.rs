use advent::{make_problems, make_ptypes};

trait PType {
    fn to(&self) -> String;
}

make_ptypes!();

impl<T: PType> PType for Option<T> {
    fn to(&self) -> String {
        self.as_ref().unwrap().to()
    }
}

impl<T: PType, E: PType> PType for Result<T, E> {
    fn to(&self) -> String {
        match self {
            Ok(t)  => t.to(),
            Err(e) => e.to(),
        }
    }
}

impl<T: PType> PType for (T, T) {
    fn to(&self) -> String {
        format!("{},{}", self.0.to(), self.1.to())
    }
}

impl<T: PType> PType for (T, T, T) {
    fn to(&self) -> String {
        format!("{},{},{}", self.0.to(), self.1.to(), self.2.to())
    }
}

fn wrap<T>(f: &'static dyn Fn(&str) -> T) -> Box<dyn Fn(&str) -> String> where T: PType {
    Box::new(move |x| f(x).to())
}

macro_rules! make_prob {
    ($y:ident, $d:ident) => {
        (wrap(&crate::$y::$d::part1),
         wrap(&crate::$y::$d::part2))
    }
}

make_problems!();
