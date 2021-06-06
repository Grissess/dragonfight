use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use super::{
    CID,
    Damage,
    DamageResult,
};
use crate::world::{World, time::*};
use crate::rng::{RandValue, RandState};
use rand::Rng;

#[derive(Debug,Clone)]
pub struct DmgRef(pub Rc<Damage>);

impl PartialEq for DmgRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for DmgRef {}

impl Hash for DmgRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state);
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
#[allow(non_camel_case_types)]
pub enum Effect {
    // Stored in the Status map:
    Frightened { level: usize, },
    Sickened { level: usize, },
    Slowed { level: usize, },
    Stunned { level: usize, },
    Dying { level: usize, },
    Doomed { level: usize, },
    Wounded { level: usize, },
    Unconscious { until: Option<Round>, },
    Restrained { until: Option<Round>, },
    Immobilized { until: Option<Round>, },
    Paralyzed { until: Option<Round>, },
    Flat_Footed { until: Option<Round>, },
    Prone { until: Option<Round>, },
    Blinded { until: Option<Round>, },
    Grappled { holder: CID, },
    Grappling { holding: CID, },
    DemoralizedImmune { to: CID, until: Round, },
    PersistentDamage { dmg: DmgRef },
    // "Virtual": cause other effects to be stored in the Status map:
    Demoralize { level: usize, by: CID, dur: Option<Rounds>, },
    BecomeDying,  // At the default Wounded level
}

#[derive(Debug,Clone,Default)]
pub struct Current {
    pub frightened: usize,
    pub sickened: usize,
    pub slowed: usize,
    pub stunned: usize,
    pub dying: usize,
    pub wounded: usize,
    pub doomed: usize,
    pub unconscious: bool,
    pub immobilized: bool,
    pub paralyzed: bool,
    pub blinded: bool,
    pub prone: bool,
    pub dead: bool,
    pub flat_footed: bool,
    pub check_mod: isize,
    pub save_mod: isize,
    pub ac_mod: isize,
    pub dc_mod: isize,
    pub speed_mod: f32,
    pub actions_gained: usize,
}

pub struct Status {
    effects: HashSet<Effect>,
}

impl Status {
    pub const DEFAULT_DYING: usize = 4;
    pub const DEFAULT_ACTIONS: usize = 3;
    pub const DEFAULT_PD_DC: isize = 15;

    pub fn new() -> Self {
        Self {
            effects: HashSet::new(),
        }
    }

    pub fn before_turn<R: Rng>(&mut self, world: &mut World<R>) -> Current {
        let mut cur = self.current(world);
        if cur.stunned > 0 {
            if cur.stunned > cur.actions_gained {
                cur.stunned -= cur.actions_gained;
                cur.actions_gained = 0;
            } else {
                cur.actions_gained -= cur.stunned;
            }
        }
        cur
    }

    pub fn after_turn<R: Rng>(&mut self, world: &mut World<R>) -> Option<Vec<DamageResult>> {
        // clear effects ending on _end_ of next turn:
        use Effect::*;

        let tm = world.time();
        let mut damage: Option<Vec<DamageResult>> = None;
        let mut new_effects: Option<Vec<Effect>> = None;

        fn ensure_vec<T>(v: &mut Option<Vec<T>>) -> &mut Vec<T> {
            if v.is_none() {
                v.replace(Vec::new());
            }
            v.as_mut().unwrap()
        }

        self.effects = self.effects
            .drain()
            .filter(|eff| {
                match eff {
                    &DemoralizedImmune { until, .. } if until <= tm.round => false,
                    &Unconscious { until: Some(t), .. } if t <= tm.round => false,
                    &Restrained { until: Some(t), .. } if t <= tm.round => false,
                    &Immobilized { until: Some(t), .. } if t <= tm.round => false,
                    &Paralyzed { until: Some(t), .. } if t <= tm.round => false,
                    &Flat_Footed { until: Some(t), .. } if t <= tm.round => false,
                    &Prone { until: Some(t), .. } if t <= tm.round => false,
                    &Blinded { until: Some(t), ..} if t <= tm.round => false,
                    &Frightened { level } => {
                        if level > 1 {
                            ensure_vec(&mut new_effects).push(Frightened { level: level - 1 });
                        }
                        false
                    },
                    PersistentDamage { dmg } => {
                        ensure_vec(&mut damage).push(dmg.0.eval(world.rng()));
                        RandValue::FLAT_CHECK.eval(world.rng()) < Self::DEFAULT_PD_DC
                    },
                    _ => true,
                }
            })
            .collect();
        if let Some(mut effects) = new_effects {
            self.effects.extend(effects.drain(..));
        }
        damage
    }

    pub fn current<R: Rng>(&self, world: &mut World<R>) -> Current {
        use Effect::*;

        let mut cur = Current::default();
        for eff in &self.effects {
            match eff {
                // These should only ever be present once, an invariant that add() maintains
                Frightened { level } => cur.frightened += level,
                Sickened { level } => cur.sickened += level,
                Slowed { level } => cur.slowed += level,
                Stunned { level } => cur.stunned += level,
                Dying { level } => cur.sickened += level,
                Doomed { level } => cur.doomed += level,
                Wounded { level } => cur.wounded += level,
                Unconscious { .. } => {
                    cur.unconscious = true;
                    cur.prone = true;
                    cur.flat_footed = true;
                    cur.blinded = true;
                },
                Immobilized { .. } | Restrained { .. } | Grappled { .. } => {
                    cur.immobilized = true;
                    cur.flat_footed = true;
                },
                Paralyzed { .. } => {
                    cur.immobilized = true;
                    cur.paralyzed = true;
                    cur.flat_footed = true;
                },
                Flat_Footed { .. } => cur.flat_footed = true,
                Prone { .. } => {
                    cur.prone = true;
                    cur.flat_footed = true;
                },
                Blinded { .. } => cur.blinded = true,
                _ => (),
            }
        }
        cur.speed_mod = 1.0;
        cur.dead = cur.dying >= Self::DEFAULT_DYING.saturating_sub(cur.doomed);
        if cur.flat_footed { cur.ac_mod -= 2; }
        if cur.frightened > 0 {
            cur.check_mod -= cur.frightened as isize;
            cur.dc_mod -= cur.frightened as isize;
        }
        cur.actions_gained = Self::DEFAULT_ACTIONS.saturating_sub(cur.slowed);
        cur
    }
}
