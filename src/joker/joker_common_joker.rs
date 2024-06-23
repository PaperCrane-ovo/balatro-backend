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
    pub price: i32,
    pub mult: i64,
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
        self.price = 4;
        self.mult = CommonJoker::BASE_MULT;
    }

    fn post_card_played(
        &mut self,
        score: &mut ScoringInfo,
        _cards: &mut Vec<Gd<PokerSprite>>,
        _category: Category,
    ) {
        score.mult += self.mult as f64;
    }

    fn get_display_info(&self) -> Gd<super::joker::JokerDisplayInfo> {
        let display_info = JokerDisplayInfo {
            name: Self::NAME_ZH.into(),
            description: Self::DESCRIPTION.into(),
            rarity: Self::RARITY,
            price: self.price,
        };
        Gd::from_object(display_info)
    }
}

impl IJokerSpritePath for CommonJoker {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for CommonJoker {}

impl CommonJoker {
    #[allow(dead_code)]
    const ID: i32 = 0;

    const NAME_ZH: &'static str = "小丑";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Joker";

    const RARITY: JokerRarity = JokerRarity::Common;

    const DESCRIPTION: &'static str = "+4倍率";

    const BASE_MULT: i64 = 4;

    const SPRITE_PATH: &'static str = "res://images/jokers/common_joker.png";

    pub fn new() -> Self {
        let mut joker = CommonJoker::default();
        joker.initialize();
        joker
    }
}
