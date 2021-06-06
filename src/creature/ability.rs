#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct Abilities {
    pub _str: isize,
    pub _dex: isize,
    pub _con: isize,
    pub _int: isize,
    pub _wis: isize,
    pub _cha: isize,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct AbilityScores(Abilities);
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct AbilityMods(Abilities);

impl From<AbilityScores> for AbilityMods {
    fn from(ab: AbilityScores) -> Self {
        Self(Abilities {
            _str: ((ab.0)._str - 10) / 2,
            _dex: ((ab.0)._dex - 10) / 2,
            _con: ((ab.0)._con - 10) / 2,
            _int: ((ab.0)._int - 10) / 2,
            _wis: ((ab.0)._wis - 10) / 2,
            _cha: ((ab.0)._cha - 10) / 2,
        })
    }
}
