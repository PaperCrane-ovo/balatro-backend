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

/// 约里克
#[derive(Default, Clone)]
pub struct Yorick {
    pub price: i32,
    pub poker_discarded: i32,
    pub multer: f64,
}

impl ICard for Yorick {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for Yorick {
    fn initialize(&mut self) {
        self.price = 10;
        self.multer = Self::MULTER_BASE;
    }

    fn on_discard_poker(&mut self, pokers: &Vec<Gd<PokerSprite>>, _: Category) {
        self.poker_discarded += pokers.len() as i32;
        if self.poker_discarded >= Self::MULTER_ADD_COND {
            self.multer += (self.poker_discarded / Self::MULTER_ADD_COND) as f64 * Self::MULTER_ADD;
            self.poker_discarded %= Self::MULTER_ADD_COND;
        }
    }

    fn post_calculate_poker_score(
        &mut self,
        score_info: &mut ScoringInfo,
        _: &mut Vec<Gd<PokerSprite>>,
        _: Category,
        _: GameCoreContext,
    ) {
        score_info.mult *= self.multer;
    }

    fn get_display_info(&self) -> Gd<JokerDisplayInfo> {
        let display_info = JokerDisplayInfo {
            name: Self::NAME_ZH.into(),
            description: Self::DESCRIPTION_FORMAT
                .replace("{0}", &self.poker_discarded.to_string())
                .replace("{1}", &self.multer.to_string())
                .into(),
            rarity: Self::RARITY,
            price: self.price,
        };
        Gd::from_object(display_info)
    }
}

impl IJokerSpritePath for Yorick {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for Yorick {}

impl Yorick {
    #[allow(dead_code)]
    const ID: i32 = 14;

    const NAME_ZH: &'static str = "约里克";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Yorick";

    const RARITY: JokerRarity = JokerRarity::Rare;

    const DESCRIPTION_FORMAT: &'static str =
        "每弃掉23[{0}]张牌\n这张小丑获得×1倍率\n（当前为×{1}倍率）";

    const MULTER_BASE: f64 = 1_f64;
    const MULTER_ADD: f64 = 1_f64;
    const MULTER_ADD_COND: i32 = 23;

    const SPRITE_PATH: &'static str = "res://images/jokers/yorick.png";

    pub fn new() -> Self {
        let mut joker = Yorick::default();
        joker.initialize();
        joker
    }
}
