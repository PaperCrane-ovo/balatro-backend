use std::fmt;

use godot::obj::Gd;

use crate::{
    card::card::ICard,
    poker::{
        category::{Category, ScoringInfo},
        poker::PokerSprite,
    },
};

use super::joker::{IJoker, IJokerCard, IJokerSpritePath, JokerRarity};

/// 备用裤子
#[derive(Default, Clone)]
pub struct SpareTrousers {
    pub name: String,
    pub rarity: JokerRarity,
    pub description_format: String,
    pub description: String,
    pub price: i32,
    pub mult: i64,
    pub sprite_path: String,
}

impl ICard for SpareTrousers {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for SpareTrousers {
    fn initialize(&mut self) {
        self.name = "Spare Trousers".to_string();
        self.rarity = JokerRarity::Uncommon;
        self.price = 4;
        self.mult = 0;
        self.sprite_path = "res://joker/common_joker.png".to_string();
        self.description_format =
            "如果打出的牌中包含\n两对\n则这张小丑牌获得+2倍率\n当前为{}".to_string();
        self.description = self
            .description_format
            .replace("{}", self.mult.to_string().as_str());
    }

    fn on_play_card(&mut self, _cards: &Vec<Gd<PokerSprite>>, pokerhand: Category) {
        if pokerhand == Category::TwoPair {
            self.mult += Self::MULT_ADD;
            self.description = self
                .description_format
                .replace("{}", self.mult.to_string().as_str());
        }
    }

    fn cal_final_chip_mag(
        &mut self,
        score: &mut ScoringInfo,
        _cards: &mut Vec<Gd<PokerSprite>>,
        _category: Category,
    ) {
        score.mult += self.mult as f64;
    }
}

impl IJokerSpritePath for SpareTrousers {
    fn get_sprite_path(&self) -> String {
        self.sprite_path.clone()
    }
}

impl IJokerCard for SpareTrousers {}

impl SpareTrousers {
    const MULT_ADD: i64 = 2;

    pub fn new() -> Self {
        let mut joker = SpareTrousers::default();
        joker.initialize();
        joker
    }
}
