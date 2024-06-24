use godot::obj::Gd;

use crate::{
    card::card::ICard,
    core::gamecore::GameCoreContext,
    poker::{
        category::{Category, ScoringInfo},
        poker::PokerSprite,
    },
};

use super::joker::{IJoker, IJokerCard, IJokerSpritePath, JokerDisplayInfo, JokerRarity};

/// 二重奏
#[derive(Default, Clone)]
pub struct TheDuo {
    pub price: i32,
}

impl ICard for TheDuo {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for TheDuo {
    fn initialize(&mut self) {
        self.price = 4;
    }

    fn post_calculate_poker_score(
        &mut self,
        score_info: &mut ScoringInfo,
        _: &mut Vec<Gd<PokerSprite>>,
        pokerhand: Category,
        _: GameCoreContext,
    ) {
        match pokerhand {
            Category::OnePair
            | Category::TwoPair
            | Category::ThreeOfAKind
            | Category::FourOfAKind
            | Category::FullHouse => {
                score_info.mult *= Self::MULTER;
            }
            _ => {}
        }
    }

    fn get_display_info(&self) -> Gd<JokerDisplayInfo> {
        let display_info = JokerDisplayInfo {
            name: Self::NAME_ZH.into(),
            description: Self::DESCRIPTION.into(),
            rarity: Self::RARITY,
            price: self.price,
        };
        Gd::from_object(display_info)
    }
}

impl IJokerSpritePath for TheDuo {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for TheDuo {}

impl TheDuo {
    #[allow(dead_code)]
    const ID: i32 = 12;

    const NAME_ZH: &'static str = "二重奏";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "The Duo";

    const RARITY: JokerRarity = JokerRarity::Rare;

    const DESCRIPTION: &'static str = "如果打出的牌中包含对子\n×2倍率";

    const MULTER: f64 = 2_f64;

    const SPRITE_PATH: &'static str = "res://images/jokers/the_duo.png";

    pub fn new() -> Self {
        let mut joker = TheDuo::default();
        joker.initialize();
        joker
    }
}

/// 秩序
#[derive(Default, Clone)]
pub struct TheOrder {
    pub price: i32,
}

impl ICard for TheOrder {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for TheOrder {
    fn initialize(&mut self) {
        self.price = 4;
    }

    fn post_calculate_poker_score(
        &mut self,
        score_info: &mut ScoringInfo,
        _: &mut Vec<Gd<PokerSprite>>,
        pokerhand: Category,
        _: GameCoreContext,
    ) {
        match pokerhand {
            Category::Straight | Category::StraightFlush | Category::RoyalFlush => {
                score_info.mult *= Self::MULTER;
            }
            _ => {}
        }
    }

    fn get_display_info(&self) -> Gd<JokerDisplayInfo> {
        let display_info = JokerDisplayInfo {
            name: Self::NAME_ZH.into(),
            description: Self::DESCRIPTION.into(),
            rarity: Self::RARITY,
            price: self.price,
        };
        Gd::from_object(display_info)
    }
}

impl IJokerSpritePath for TheOrder {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for TheOrder {}

impl TheOrder {
    #[allow(dead_code)]
    const ID: i32 = 13;

    const NAME_ZH: &'static str = "秩序";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "The Order";

    const RARITY: JokerRarity = JokerRarity::Rare;

    const DESCRIPTION: &'static str = "如果打出的牌中包含顺子\n×3倍率";

    const MULTER: f64 = 3_f64;

    const SPRITE_PATH: &'static str = "res://images/jokers/the_order.png";

    pub fn new() -> Self {
        let mut joker = TheOrder::default();
        joker.initialize();
        joker
    }
}
