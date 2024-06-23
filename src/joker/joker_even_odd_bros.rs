/// 奇偶
use godot::obj::Gd;

use crate::{
    card::card::ICard,
    poker::{
        category::{Category, ScoringInfo},
        poker::{PokerRank, PokerSprite},
    },
};

use super::joker::{IJoker, IJokerCard, IJokerSpritePath, JokerDisplayInfo, JokerRarity};

/// Odd Todd
#[derive(Default, Clone)]
pub struct OddTodd {
    pub price: i32,
}

impl ICard for OddTodd {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for OddTodd {
    fn initialize(&mut self) {
        self.price = 4;
    }

    fn on_calculate_poker_score(
        &mut self,
        score_info: &mut ScoringInfo,
        hands: &mut Vec<Gd<PokerSprite>>,
        _: Category,
    ) {
        for i in hands.iter_mut().map(|x| x.bind().poker) {
            match i.rank {
                PokerRank::A
                | PokerRank::Three
                | PokerRank::Five
                | PokerRank::Seven
                | PokerRank::Nine => {
                    score_info.chips += Self::CHIPS_ADD;
                }
                _ => {}
            };
        }
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

impl IJokerSpritePath for OddTodd {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for OddTodd {}

impl OddTodd {
    #[allow(dead_code)]
    const ID: i32 = 8;

    const NAME_ZH: &'static str = "奇数托德";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Odd Todd";

    const RARITY: JokerRarity = JokerRarity::Common;

    const DESCRIPTION: &'static str = "打出的点数为奇数的牌\n在计分时给予+31筹码";

    const CHIPS_ADD: i64 = 31;

    const SPRITE_PATH: &'static str = "res://images/jokers/odd_todd.png";

    pub fn new() -> Self {
        let mut joker = OddTodd::default();
        joker.initialize();
        joker
    }
}

/// Even Steven
#[derive(Default, Clone)]
pub struct EvenSteven {
    pub price: i32,
}

impl ICard for EvenSteven {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for EvenSteven {
    fn initialize(&mut self) {
        self.price = 4;
    }

    fn on_calculate_poker_score(
        &mut self,
        score_info: &mut ScoringInfo,
        hands: &mut Vec<Gd<PokerSprite>>,
        _: Category,
    ) {
        for i in hands.iter_mut().map(|x| x.bind().poker) {
            match i.rank {
                PokerRank::Two
                | PokerRank::Four
                | PokerRank::Six
                | PokerRank::Eight
                | PokerRank::Ten => {
                    score_info.mult += Self::MULT_ADD;
                }
                _ => {}
            };
        }
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

impl IJokerSpritePath for EvenSteven {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for EvenSteven {}

impl EvenSteven {
    #[allow(dead_code)]
    const ID: i32 = 9;

    const NAME_ZH: &'static str = "偶数史蒂文";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Even Steven";

    const RARITY: JokerRarity = JokerRarity::Common;

    const DESCRIPTION: &'static str = "打出的点数为偶数的牌\n在计分时给予+4倍率";

    const MULT_ADD: f64 = 4_f64;

    const SPRITE_PATH: &'static str = "res://images/jokers/even_steven.png";

    pub fn new() -> Self {
        let mut joker = EvenSteven::default();
        joker.initialize();
        joker
    }
}
