use std::collections::HashSet;

use crate::creature;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum Terrain {
    Passable,
    Difficult,
    Unpassable,
}

impl Default for Terrain {
    fn default() -> Self { Terrain::Unpassable }
}

#[derive(Debug,Clone,Default)]
pub struct Square {
    pub terrain: Terrain,
    pub occupants: HashSet<creature::CID>,
}
