#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum GoodEvil {
    Evil,
    Neutral,
    Good,
}

impl GoodEvil {
    pub fn opposite(self) -> Self {
        use GoodEvil::*;

        match self {
            Evil => Good,
            Good => Evil,
            x => x,
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum LawChaos {
    Chaotic,
    Neutral,
    Lawful,
}

impl LawChaos {
    pub fn opposite(self) -> Self {
        use LawChaos::*;

        match self {
            Chaotic => Lawful,
            Lawful => Chaotic,
            x => x,
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct Alignment {
    pub ge: GoodEvil,
    pub lc: LawChaos,
}
