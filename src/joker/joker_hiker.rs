use godot::obj::Gd;

use crate::{
    card::card::ICard,
    poker::{
        category::{Category, ScoringInfo},
        poker::PokerSprite,
    },
};

use super::joker::{IJoker, IJokerCard, IJokerSpritePath, JokerDisplayInfo, JokerRarity};

/// 徒步者
#[derive(Default, Clone)]
pub struct Hiker {
    pub price: i32,
    pub mult: i64,
}

impl ICard for Hiker {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for Hiker {
    fn initialize(&mut self) {
        self.price = 4;
    }

    fn on_card_played(
        &mut self,
        _: &mut ScoringInfo,
        hands: &mut Vec<Gd<PokerSprite>>,
        _: Category,
    ) {
        hands
            .iter_mut()
            .for_each(|x| x.bind_mut().poker.extra_chip += Self::POKER_CHIPS_ADD);
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

impl IJokerSpritePath for Hiker {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for Hiker {}

impl Hiker {
    #[allow(dead_code)]
    const ID: i32 = 6;

    const NAME_ZH: &'static str = "徒步者";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Hiker";

    const RARITY: JokerRarity = JokerRarity::Uncommon;

    const DESCRIPTION: &'static str = "打出的每一张牌\n在计分时\n会永久获得+5筹码";

    const POKER_CHIPS_ADD: i64 = 5;

    // TODO
    const SPRITE_PATH: &'static str = "res://images/jokers/hiker.png";

    pub fn new() -> Self {
        let mut joker = Hiker::default();
        joker.initialize();
        joker
    }
}
