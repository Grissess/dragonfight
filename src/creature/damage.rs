use std::ops::Deref;
use std::collections::{HashMap, HashSet};

use super::alignment::*;
use crate::rng::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Bleed,
    Acid,
    Cold,
    Electricity,
    Fire,
    Sonic,
    Chaotic,
    Evil,
    Good,
    Lawful,
    Mental,
    Poison,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum DamageKind {
    Physical,
    Energy,
    Aligned,
    Mental,
    Poison,
}

#[derive(Debug,Clone)]
pub struct Damage {
    pub tp: DamageType,
    pub magical: bool,
    pub amount: RandValue,
    pub prec_amount: Option<RandValue>,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct DamageResult {
    pub tp: DamageType,
    pub magical: bool,
    pub amount: usize,
    pub prec_amount: usize,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum Spec {
    Type(DamageType),
    Kind(DamageKind),
    NonMagical,
    Precision,
    All,
}

#[derive(Debug,Clone)]
pub struct Modifiers {
    pub resistances: HashMap<Spec, usize>,
    pub weaknesses: HashMap<Spec, usize>,
    pub immunities: HashSet<Spec>,
}

#[derive(Debug,Clone,Copy,Default)]
struct ModTest {
    resistance: Option<usize>,
    weakness: Option<usize>,
    immunity: bool,
}

impl DamageType {
    pub fn kind(self) -> DamageKind {
        use DamageType::*;

        match self {
            Slashing | Piercing | Bludgeoning | Bleed => DamageKind::Physical,
            Acid | Cold | Electricity | Fire | Sonic => DamageKind::Energy,
            Chaotic | Evil | Good | Lawful => DamageKind::Aligned,
            Mental => DamageKind::Mental,
            Poison => DamageKind::Poison,
        }
    }

    pub fn affects_align(self, align: Alignment) -> bool {
        use DamageType::*;

        match self {
            Chaotic => align.lc == LawChaos::Lawful,
            Evil => align.ge == GoodEvil::Good,
            Good => align.ge == GoodEvil::Evil,
            Lawful => align.lc == LawChaos::Chaotic,
            _ => false,
        }
    }
}

impl Damage {
    pub fn eval<R: rand::Rng>(&self, rng: &mut RandState<R>) -> DamageResult {
        DamageResult {
            tp: self.tp,
            magical: self.magical,
            amount: std::cmp::max(self.amount.eval(rng), 0) as usize,
            prec_amount: std::cmp::max(self.amount.eval(rng), 0) as usize,
        }
    }
}

impl Modifiers {
    fn test(&self, spec: Spec) -> ModTest {
        ModTest {
            resistance: self.resistances.get(&spec).map(|&x| x),
            weakness: self.weaknesses.get(&spec).map(|&x| x),
            immunity: self.immunities.contains(&spec),
        }
    }

    fn apply(&self, dmg: DamageResult) -> Option<DamageResult> {
        let mtype = self.test(Spec::Type(dmg.tp));
        let mkind = self.test(Spec::Kind(dmg.tp.kind()));
        let mnonmag = if dmg.magical { Default::default() } else { self.test(Spec::NonMagical) };
        let mall = self.test(Spec::All);

        let results = &[mtype, mkind, mnonmag, mall];
        if results.iter().any(|t| t.immunity) {
            return None;
        }
        let weakness: usize = results.iter().filter_map(|x| x.weakness).sum();
        let resistance: usize = results.iter().filter_map(|x| x.resistance).sum();
        let mut damage = dmg.amount;

        let mprec = self.test(Spec::Precision);
        let prec_damage = if mprec.immunity {
            0
        } else {
            (dmg.prec_amount + mprec.weakness.unwrap_or(0)).saturating_sub(mprec.resistance.unwrap_or(0))
        };
        damage += prec_damage;

        damage = (damage + weakness).saturating_sub(resistance);
        if damage == 0 {
            None
        } else {
            Some(DamageResult { amount: damage, prec_amount: 0, ..dmg })
        }
    }
}
