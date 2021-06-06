use std::ops::{Add, Sub, Mul};

// Constrain T: rand::Rng
pub struct RandState<R>(R);

#[derive(Debug,Clone)]
pub enum RandValue {
    Const(isize),
    Sum(Vec<RandValue>),
    Product(Vec<RandValue>),
    Negate(Box<RandValue>),
    Die { faces: usize, times: usize},
}

impl Mul<usize> for RandValue {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        use RandValue::*;

        match self {
            Const(i) => Const(i*rhs as isize),
            Product(mut v) => {
                v.push(Const(rhs as isize));
                Product(v)
            },
            Die { faces, times } => Die { faces, times: times*rhs },
            other => Product(vec![other, Const(rhs as isize)]),
        }
    }
}

impl Add<isize> for RandValue {
    type Output = Self;
    fn add(self, rhs: isize) -> Self {
        use RandValue::*;

        match self {
            Const(i) => Const(i+rhs),
            Sum(mut v) => {
                v.push(Const(rhs));
                Sum(v)
            },
            other => Sum(vec![other, Const(rhs)]),
        }
    }
}

impl Add<usize> for RandValue {
    type Output = Self;
    fn add(self, rhs: usize) -> Self { self + (rhs as isize) }
}

impl Add for RandValue {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        use RandValue::*;

        match self {
            Sum(mut v) => match rhs {
                Sum(mut u) => {
                    v.extend(u.drain(..));
                    Sum(v)
                },
                other => {
                    v.push(other);
                    Sum(v)
                },
            },
            other => match rhs {
                Sum(mut v) => {
                    // Abusing commutativity
                    v.push(other);
                    Sum(v)
                },
                neither => Sum(vec![other, neither]),
            },
        }
    }
}

impl RandValue {
    pub const D4: RandValue = RandValue::Die { faces: 4, times: 1 };
    pub const D6: RandValue = RandValue::Die { faces: 6, times: 1 };
    pub const D8: RandValue = RandValue::Die { faces: 8, times: 1 };
    pub const D10: RandValue = RandValue::Die { faces: 10, times: 1 };
    pub const D12: RandValue = RandValue::Die { faces: 12, times: 1 };
    pub const D20: RandValue = RandValue::Die { faces: 20, times: 1 };
    pub const D100: RandValue = RandValue::Die { faces: 100, times: 1 };

    pub const FLAT_CHECK: RandValue = Self::D20;

    pub fn eval<R: rand::Rng>(&self, rng: &mut RandState<R>) -> isize {
        use RandValue::*;

        match self {
            &Const(i) => i,
            Sum(v) => v.iter().map(|x| x.eval(rng)).sum(),
            Product(v) => v.iter().map(|x| x.eval(rng)).product(),
            Negate(x) => -(x.eval(rng)),
            Die { faces, times } => {
                let f = *faces as isize;
                std::iter::repeat_with(
                    || rng.0.gen_range(1 ..= f)
                ).take(*times).sum()
            },
        }
    }
}
