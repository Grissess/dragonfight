pub mod space;
pub mod time;
pub mod terrain;

use std::collections::{HashMap, VecDeque};

use util::grid::{self, region};

use crate::creature::{self, *};
use crate::rng;

pub struct World<R> {
    bestiary: HashMap<String, creature::Creature>,
    creatures: HashMap<creature::CID, creature::State>,
    initiative: VecDeque<creature::CID>,
    rng: rng::RandState<R>,
    time: time::Time,
    map: region::Region<terrain::Square>,
}

impl<R> World<R> {
    pub fn time(&self) -> time::Time {
        self.time
    }

    pub fn rng(&mut self) -> &mut rng::RandState<R> {
        &mut self.rng
    }
}
