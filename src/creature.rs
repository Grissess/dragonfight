pub mod ability;
pub mod status;
pub mod size;
pub mod damage;
pub mod alignment;
pub mod health;

pub use self::{
    ability::*,
    status::*,
    size::*,
    damage::*,
    alignment::*,
    health::*,
};

use crate::world;

use std::rc::Rc;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct CID(usize);

pub struct Creature {
    scores: AbilityScores,
    align: Alignment,
    dmgmods: damage::Modifiers,
}

pub struct State {
    id: CID,
    creature: Rc<Creature>,
    loc: world::space::Location,
    health: Health,
}
