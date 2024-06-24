use godot::obj::Gd;

use crate::{
    card::card::ICard,
    core::gamecore::GameCoreContext,
    poker::{
        category::{Category, ScoringInfo},
        poker::PokerSprite,
    },
};

use super::{
    joker::{IJoker, IJokerCard, IJokerSpritePath, JokerDisplayInfo, JokerRarity},
    utils::get_gold_display_info,
};

/// 斗牛
#[derive(Default, Clone)]
pub struct Bull {
    pub price: i32,
}

impl ICard for Bull {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for Bull {
    fn initialize(&mut self) {
        self.price = 3;
    }

    fn post_calculate_poker_score(
        &mut self,
        score: &mut ScoringInfo,
        _cards: &mut Vec<Gd<PokerSprite>>,
        _category: Category,
        context: GameCoreContext,
    ) {
        score.chips += Self::CHIPS_PER_GOLD * (context.gold as i64);
    }

    fn get_display_info(&self) -> Gd<JokerDisplayInfo> {
        let display_info = JokerDisplayInfo {
            name: Self::NAME_ZH.into(),
            description: Self::DESCRIPTION_FORMAT
                .replace(
                    "{}",
                    (Self::CHIPS_PER_GOLD * (get_gold_display_info() as i64))
                        .to_string()
                        .as_str(),
                )
                .into(),
            rarity: Self::RARITY,
            price: self.price,
        };
        Gd::from_object(display_info)
    }
}

impl IJokerSpritePath for Bull {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for Bull {}

impl Bull {
    #[allow(dead_code)]
    const ID: i32 = 10;

    const NAME_ZH: &'static str = "斗牛";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Bull";

    const RARITY: JokerRarity = JokerRarity::Uncommon;

    const DESCRIPTION_FORMAT: &'static str = "每拥有$1，+2筹码\n（当前为+{}筹码）";

    const CHIPS_PER_GOLD: i64 = 2;

    const SPRITE_PATH: &'static str = "res://images/jokers/bull.png";

    pub fn new() -> Self {
        let mut joker = Bull::default();
        joker.initialize();
        joker
    }
}

/// 提靴带
#[derive(Default, Clone)]
pub struct Bootstraps {
    pub price: i32,
}

impl ICard for Bootstraps {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for Bootstraps {
    fn initialize(&mut self) {
        self.price = 3;
    }

    fn post_calculate_poker_score(
        &mut self,
        score: &mut ScoringInfo,
        _cards: &mut Vec<Gd<PokerSprite>>,
        _category: Category,
        context: GameCoreContext,
    ) {
        score.mult += Self::MULT_PER_FIVE_GOLD * ((context.gold / 5) as f64);
    }

    fn get_display_info(&self) -> Gd<JokerDisplayInfo> {
        let display_info = JokerDisplayInfo {
            name: Self::NAME_ZH.into(),
            description: Self::DESCRIPTION_FORMAT
                .replace(
                    "{}",
                    (Self::MULT_PER_FIVE_GOLD * ((get_gold_display_info() / 5) as f64))
                        .to_string()
                        .as_str(),
                )
                .into(),
            rarity: Self::RARITY,
            price: self.price,
        };
        Gd::from_object(display_info)
    }
}

impl IJokerSpritePath for Bootstraps {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for Bootstraps {}

impl Bootstraps {
    #[allow(dead_code)]
    const ID: i32 = 11;

    const NAME_ZH: &'static str = "提靴带";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Bootstraps";

    const RARITY: JokerRarity = JokerRarity::Uncommon;

    const DESCRIPTION_FORMAT: &'static str = "每拥有$5，+2筹码\n（当前为+{}倍率）";

    const MULT_PER_FIVE_GOLD: f64 = 2_f64;

    const SPRITE_PATH: &'static str = "res://images/jokers/bootstraps.png";

    pub fn new() -> Self {
        let mut joker = Bootstraps::default();
        joker.initialize();
        joker
    }
}
