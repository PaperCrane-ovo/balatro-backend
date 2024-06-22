use godot::obj::Gd;

use crate::{card::card::ICard, poker::{category::Category, poker::PokerSprite}};

use super::joker::{IJoker, IJokerCard, JokerRarity};

#[derive(Clone)]
pub struct CommonJoker {
    pub name: String,
    pub rarity: JokerRarity,
    pub description: String,
    pub price: i32,
    pub magnification: i32,
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
        self.magnification = 4;
        self.sprite_path = "res://joker/common_joker.png".to_string();
    }
    fn cal_final_chip_mag(
        &mut self,
        score: &mut Vec<i32>,
        _cards: &mut Vec<Gd<PokerSprite>>,
        _category:Category,
    ) {
        score[1] += self.magnification;
    }
}

impl IJokerCard for CommonJoker {}

impl CommonJoker {
    pub fn new() -> Self {
        let mut joker = CommonJoker {
            name: "".to_string(),
            rarity: JokerRarity::Common,
            description: "".to_string(),
            price: 0,
            magnification: 0,
            sprite_path: "".to_string(),
        };
        joker.initialize();
        joker
    }
}
