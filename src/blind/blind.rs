use godot::prelude::*;

use super::{BlindState, BlindType};

#[derive(GodotClass)]
#[class(init)]
pub struct Blind {
    #[var]
    pub blind_type: BlindType,
    #[var]
    pub hp: i32,
    #[var]
    pub award: i32,
    #[var]
    pub state: BlindState,
}

impl Blind {
    pub fn new(blind_type: BlindType) -> Self {
        Self {
            blind_type,
            hp: 0,
            award: 0,
            state: BlindState::NotChoose,
        }
    }
}

#[godot_api]
impl Blind {
    #[func]
    pub fn init_hp(&mut self, cur_ante: i32) {
        let base_hp = [100, 300, 800, 2000, 5000, 11000, 20000, 35000, 50000];
        self.hp = match self.blind_type {
            BlindType::SmallBlind => base_hp[cur_ante as usize],
            BlindType::BigBlind => (base_hp[cur_ante as usize] as f64 * 1.5) as i32,
            BlindType::BossBlind => base_hp[cur_ante as usize] * 2,
        }
    }
    pub fn init_award(&mut self) {
        self.award = match self.blind_type {
            BlindType::SmallBlind => 3,
            BlindType::BigBlind => 4,
            BlindType::BossBlind => 5,
        };
    }
}
