use crate::world::space::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl Size {
    pub fn space(self) -> Squares {
        use Size::*;
        Squares(match self {
            Tiny => 0,
            Small | Medium => 1,
            Large => 2,
            Huge => 3,
            Gargantuan => 4,  // XXX "or more"
        })
    }
}
