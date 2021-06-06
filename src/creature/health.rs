#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct Health {
    pub hp: usize,
    pub max_hp: usize,
    pub temp_hp: usize,
}

impl Health {
    pub fn take_damage(&mut self, mut amount: usize) {
        if self.temp_hp > 0 {
            if amount > self.temp_hp {
                amount -= self.temp_hp;
                self.temp_hp = 0;
            } else {
                self.temp_hp -= amount;
                return;
            }
        }

        self.hp = self.hp.saturating_sub(amount);
    }

    pub fn take_healing(&mut self, amount: usize) {
        self.hp = std::cmp::min(self.hp + amount, self.max_hp);
    }

    pub fn take_temp_hp(&mut self, amount: usize) {
        self.temp_hp = std::cmp::max(self.temp_hp, amount);
    }

    pub fn damaged(&self, amount: usize) -> Health {
        let mut res = self.clone();
        res.take_damage(amount);
        res
    }

    pub fn healed(&self, amount: usize) -> Health {
        let mut res = self.clone();
        res.take_healing(amount);
        res
    }

    pub fn with_temp_hp(&self, amount: usize) -> Health {
        let mut res = self.clone();
        res.take_temp_hp(amount);
        res
    }

    pub fn is_down(&self) -> bool {
        // The latter SHOULD imply the former, but...
        self.hp == 0 && self.temp_hp == 0
    }
}
