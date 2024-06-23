use godot::obj::Gd;

use crate::{
    card::card::ICard,
    poker::{
        category::{Category, ScoringInfo},
        poker::PokerSprite,
    },
};

use super::joker::{IJoker, IJokerCard, IJokerSpritePath, JokerDisplayInfo, JokerRarity};

/// 小丑
#[derive(Default, Clone)]
pub struct CommonJoker {
    pub name: String,
    pub rarity: JokerRarity,
    pub description: String,
    pub price: i32,
    pub mult: i64,
    pub sprite_path: String,
}

impl ICard for CommonJoker {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for CommonJoker {
    fn initialize(&mut self) {
        self.name = "Common Joker".to_string();
        self.rarity = JokerRarity::Common;
        self.description = "+4 倍率".to_string();
        self.price = 4;
        self.mult = 4;
        self.sprite_path = "res://images/jokers/common_joker.jpg".to_string();
    }

    fn cal_final_chip_mag(
        &mut self,
        score: &mut ScoringInfo,
        _cards: &mut Vec<Gd<PokerSprite>>,
        _category: Category,
    ) {
        score.mult += self.mult as f64;
    }
    
    fn get_display_info(&self) -> Gd<super::joker::JokerDisplayInfo> {
        let display_info = JokerDisplayInfo{
            name:self.name.clone().into(),
            description:self.description.clone().into(),
            rarity:self.rarity,
            price:self.price,
        };
        Gd::from_object(display_info)
    }
}


impl IJokerSpritePath for CommonJoker {
    fn get_sprite_path(&self) -> String {
        self.sprite_path.clone()
    }
}

impl IJokerCard for CommonJoker {}

impl CommonJoker {
    pub fn new() -> Self {
        let mut joker = CommonJoker::default();
        joker.initialize();
        joker
    }
}
