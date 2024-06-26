use godot::obj::Gd;

use crate::{
    card::card::ICard,
    core::gamecore::GameCoreContext,
    poker::{
        category::{Category, ScoringInfo},
        poker::PokerSprite,
    },
};

use super::joker::{IJoker, IJokerCard, IJokerSpritePath, JokerRarity};

/// 备用裤子
#[derive(Default, Clone)]
pub struct SpareTrousers {
    pub description: String,
    pub price: i32,
    pub mult: i64,
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
        self.price = 3;
        self.mult = Self::BASE_MULT;
        self.description = Self::DESCRIPTION_FORMAT.replace("{}", self.mult.to_string().as_str());
    }

    fn on_play_hand(&mut self, _: &Vec<Gd<PokerSprite>>, pokerhand: Category) {
        match pokerhand {
            Category::FullHouse | Category::TwoPair => {
                self.mult += Self::MULT_ADD;
                self.description =
                    Self::DESCRIPTION_FORMAT.replace("{}", self.mult.to_string().as_str());
            }
            _ => {}
        };
    }

    fn post_calculate_poker_score(
        &mut self,
        score: &mut ScoringInfo,
        _cards: &mut Vec<Gd<PokerSprite>>,
        _category: Category,
        _: GameCoreContext,
    ) {
        score.mult += self.mult as f64;
    }

    fn get_display_info(&self) -> Gd<super::joker::JokerDisplayInfo> {
        let display_info = super::joker::JokerDisplayInfo {
            name: Self::NAME_ZH.into(),
            description: self.description.clone().into(),
            rarity: Self::RARITY,
            price: self.price,
        };
        Gd::from_object(display_info)
    }
}

impl IJokerSpritePath for SpareTrousers {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for SpareTrousers {}

impl SpareTrousers {
    #[allow(dead_code)]
    const ID: i32 = 5;

    const NAME_ZH: &'static str = "备用裤子";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Spare Trousers";

    const RARITY: JokerRarity = JokerRarity::Uncommon;

    const DESCRIPTION_FORMAT: &'static str =
        "如果打出的牌中包含两对\n则这张小丑牌获得+2倍率\n（当前为+{}倍率）";

    const BASE_MULT: i64 = 0;

    const MULT_ADD: i64 = 2;

    // TODO
    const SPRITE_PATH: &'static str = "res://images/jokers/spare_trousers.png";

    pub fn new() -> Self {
        let mut joker = SpareTrousers::default();
        joker.initialize();
        joker
    }
}
