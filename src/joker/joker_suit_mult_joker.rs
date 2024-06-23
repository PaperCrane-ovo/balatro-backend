/// 四牌组小丑
use godot::obj::Gd;

use crate::{
    card::card::ICard,
    poker::{
        category::{Category, ScoringInfo},
        poker::{PokerSprite, Suit},
    },
};

use super::joker::{IJoker, IJokerCard, IJokerSpritePath, JokerDisplayInfo, JokerRarity};

/// 贪婪小丑
#[derive(Default, Clone)]
pub struct DiamondsMultJoker {
    pub price: i32,
}

impl ICard for DiamondsMultJoker {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for DiamondsMultJoker {
    fn initialize(&mut self) {
        self.price = 4;
    }

    fn on_card_played(
        &mut self,
        score_info: &mut ScoringInfo,
        hands: &mut Vec<Gd<PokerSprite>>,
        _: Category,
    ) {
        for i in hands.iter_mut().map(|x| x.bind().poker) {
            if i.suit == Self::BONUS_SUIT {
                score_info.mult += Self::MULT_ADD;
            }
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

impl IJokerSpritePath for DiamondsMultJoker {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for DiamondsMultJoker {}

impl DiamondsMultJoker {
    #[allow(dead_code)]
    const ID: i32 = 1;

    const NAME_ZH: &'static str = "贪婪小丑";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Greedy Joker";

    const RARITY: JokerRarity = JokerRarity::Common;

    const DESCRIPTION: &'static str = "打出的方片花色牌\n在计分时给予+3倍率";

    const BONUS_SUIT: Suit = Suit::Diamonds;

    const MULT_ADD: f64 = 3_f64;

    const SPRITE_PATH: &'static str = "res://images/jokers/greedy_joker.png";

    pub fn new() -> Self {
        let mut joker = DiamondsMultJoker::default();
        joker.initialize();
        joker
    }
}

/// 色欲小丑
#[derive(Default, Clone)]
pub struct HeartsMultJoker {
    pub price: i32,
}

impl ICard for HeartsMultJoker {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for HeartsMultJoker {
    fn initialize(&mut self) {
        self.price = 4;
    }

    fn on_card_played(
        &mut self,
        score_info: &mut ScoringInfo,
        hands: &mut Vec<Gd<PokerSprite>>,
        _: Category,
    ) {
        for i in hands.iter_mut().map(|x| x.bind().poker) {
            if i.suit == Self::BONUS_SUIT {
                score_info.mult += Self::MULT_ADD;
            }
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

impl IJokerSpritePath for HeartsMultJoker {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for HeartsMultJoker {}

impl HeartsMultJoker {
    #[allow(dead_code)]
    const ID: i32 = 2;

    const NAME_ZH: &'static str = "色欲小丑";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Lusty Joker";

    const RARITY: JokerRarity = JokerRarity::Common;

    const DESCRIPTION: &'static str = "打出的红桃花色牌\n在计分时给予+3倍率";

    const BONUS_SUIT: Suit = Suit::Hearts;

    const MULT_ADD: f64 = 3_f64;

    // TODO
    const SPRITE_PATH: &'static str = "res://images/jokers/lusty_joker.png";

    pub fn new() -> Self {
        let mut joker = HeartsMultJoker::default();
        joker.initialize();
        joker
    }
}

/// 愤怒小丑
#[derive(Default, Clone)]
pub struct SpadesMultJoker {
    pub price: i32,
}

impl ICard for SpadesMultJoker {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for SpadesMultJoker {
    fn initialize(&mut self) {
        self.price = 4;
    }

    fn on_card_played(
        &mut self,
        score_info: &mut ScoringInfo,
        hands: &mut Vec<Gd<PokerSprite>>,
        _: Category,
    ) {
        for i in hands.iter_mut().map(|x| x.bind().poker) {
            if i.suit == Self::BONUS_SUIT {
                score_info.mult += Self::MULT_ADD;
            }
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

impl IJokerSpritePath for SpadesMultJoker {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for SpadesMultJoker {}

impl SpadesMultJoker {
    #[allow(dead_code)]
    const ID: i32 = 3;

    const NAME_ZH: &'static str = "愤怒小丑";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Wrathful Joker";

    const RARITY: JokerRarity = JokerRarity::Common;

    const DESCRIPTION: &'static str = "打出的黑桃花色牌\n在计分时给予+3倍率";

    const BONUS_SUIT: Suit = Suit::Spades;

    const MULT_ADD: f64 = 3_f64;

    // TODO
    const SPRITE_PATH: &'static str = "res://images/jokers/wrathful_joker.png";

    pub fn new() -> Self {
        let mut joker = SpadesMultJoker::default();
        joker.initialize();
        joker
    }
}

/// 暴食小丑
#[derive(Default, Clone)]
pub struct ClubsMultJoker {
    pub price: i32,
}

impl ICard for ClubsMultJoker {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for ClubsMultJoker {
    fn initialize(&mut self) {
        self.price = 4;
    }

    fn on_card_played(
        &mut self,
        score_info: &mut ScoringInfo,
        hands: &mut Vec<Gd<PokerSprite>>,
        _: Category,
    ) {
        for i in hands.iter_mut().map(|x| x.bind().poker) {
            if i.suit == Self::BONUS_SUIT {
                score_info.mult += Self::MULT_ADD;
            }
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

impl IJokerSpritePath for ClubsMultJoker {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for ClubsMultJoker {}

impl ClubsMultJoker {
    #[allow(dead_code)]
    const ID: i32 = 4;

    const NAME_ZH: &'static str = "暴食小丑";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Gluttonous Joker";

    const RARITY: JokerRarity = JokerRarity::Common;

    const DESCRIPTION: &'static str = "打出的梅花花色牌\n在计分时给予+3倍率";

    const BONUS_SUIT: Suit = Suit::Clubs;

    const MULT_ADD: f64 = 3_f64;

    // TODO
    const SPRITE_PATH: &'static str = "res://images/jokers/gluttonous_joker.png";

    pub fn new() -> Self {
        let mut joker = ClubsMultJoker::default();
        joker.initialize();
        joker
    }
}
