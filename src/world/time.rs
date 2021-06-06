use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::time::Duration;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Round(pub usize);  // Essentially "Instant" at the round level
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Rounds(pub usize);  // Essentially "Duration" at the round level
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Turn(pub usize);  // "Instant" for turns
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Turns(pub usize);  // "Duration" for turns

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Time {
    pub round: Round,
    pub turn: Turn,
}

macro_rules! derive_op {
    ($type:ident, $rhs:ident, $trait:ident, $meth:ident, $binop:tt, $atrait:ident, $assn: ident, $abinop: tt) => {
        impl $trait<$rhs> for $type {
            type Output = Self;
            fn $meth(self, o: $rhs) -> Self {
                Self(self.0 $binop o.0)
            }
        }

        impl $atrait<$rhs> for $type {
            fn $assn(&mut self, o: $rhs) {
                self.0 $abinop o.0;
            }
        }
    }
}

macro_rules! impl_arith {
    ($type:ident) => {
        derive_op!($type, $type, Add, add, +, AddAssign, add_assign, +=);
        derive_op!($type, $type, Sub, sub, -, SubAssign, sub_assign, -=);
        derive_op!($type, $type, Mul, mul, *, MulAssign, mul_assign, *=);
        derive_op!($type, $type, Div, div, /, DivAssign, div_assign, /=);
    };
    ($type:ident, $rhs:ident) => {
        derive_op!($type, $rhs, Add, add, +, AddAssign, add_assign, +=);
        derive_op!($type, $rhs, Sub, sub, -, SubAssign, sub_assign, -=);
        derive_op!($type, $rhs, Mul, mul, *, MulAssign, mul_assign, *=);
        derive_op!($type, $rhs, Div, div, /, DivAssign, div_assign, /=);
    }
}

macro_rules! impl_types {
    ($inst:ident, $dur:ident) => {
        impl $dur {
            pub const ZERO: $dur = $dur(0);
        }

        impl_arith!($dur);
        impl_arith!($inst, $dur);

        impl Sub for $inst {
            type Output = $dur;
            fn sub(self, o: Self) -> $dur {
                $dur(self.0 - o.0)
            }
        }
    }
}

impl_types!(Round, Rounds);
impl_types!(Turn, Turns);

impl Rounds {
    pub const SECS_PER: usize = 6;
    pub const ONE_MINUTE: usize = 60 / Self::SECS_PER;
    pub const TEN_MINUTES: usize = 10 * Self::ONE_MINUTE;
    pub const ONE_HOUR: usize = 60 * Self::ONE_MINUTE;
}

impl From<Duration> for Round {
    fn from(d: Duration) -> Self {
        Self(d.as_secs() as usize / Rounds::SECS_PER)
    }
}

impl Into<Duration> for Round {
    fn into(self) -> Duration {
        Duration::new((self.0 * Rounds::SECS_PER) as u64, 0)
    }
}
