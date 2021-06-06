#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Squares(pub usize);
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Feet(pub usize);

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Location(pub util::V2i);

impl Squares {
    pub const FEET_PER: usize = 5;
}

impl From<Squares> for Feet {
    fn from(s: Squares) -> Self {
        Self(s.0 * Squares::FEET_PER)
    }
}

impl From<Feet> for Squares {
    fn from(f: Feet) -> Self {
        Self(f.0 / Squares::FEET_PER)
    }
}
