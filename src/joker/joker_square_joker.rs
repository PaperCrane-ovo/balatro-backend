use godot::obj::Gd;

use crate::{
    card::card::ICard,
    poker::{
        category::{Category, ScoringInfo},
        poker::PokerSprite,
    },
};

use super::joker::{IJoker, IJokerCard, IJokerSpritePath, JokerDisplayInfo, JokerRarity};

/// 方块小丑
#[derive(Default, Clone)]
pub struct SquareJoker {
    pub description: String,
    pub price: i32,
    pub chips: i64,
}

impl ICard for SquareJoker {
    fn get_price(&self) -> i32 {
        self.price
    }
    fn set_price(&mut self, price: i32) {
        self.price = price;
    }
}

impl IJoker for SquareJoker {
    fn initialize(&mut self) {
        self.price = 4;
        self.chips = Self::BASE_CHIPS;
        self.description = Self::DESCRIPTION_FORMAT.replace("{}", self.chips.to_string().as_str());
    }

    fn on_play_hand(&mut self, hands: &Vec<Gd<PokerSprite>>, _: Category) {
        if hands.len() == 4 {
            self.chips += Self::CHIPS_ADD;
            self.description =
                Self::DESCRIPTION_FORMAT.replace("{}", self.chips.to_string().as_str());
        }
    }

    fn post_calculate_poker_score(
        &mut self,
        score_info: &mut ScoringInfo,
        _: &mut Vec<Gd<PokerSprite>>,
        _: Category,
    ) {
        score_info.chips += self.chips;
    }

    fn get_display_info(&self) -> Gd<super::joker::JokerDisplayInfo> {
        let display_info = JokerDisplayInfo {
            name: Self::NAME_ZH.into(),
            description: self.description.clone().into(),
            rarity: Self::RARITY,
            price: self.price,
        };
        Gd::from_object(display_info)
    }
}

impl IJokerSpritePath for SquareJoker {
    fn get_sprite_path(&self) -> String {
        Self::SPRITE_PATH.to_string()
    }
}

impl IJokerCard for SquareJoker {}

impl SquareJoker {
    #[allow(dead_code)]
    const ID: i32 = 7;

    const NAME_ZH: &'static str = "方块小丑";

    #[allow(dead_code)]
    const NAME_EN: &'static str = "Square Joker";

    const RARITY: JokerRarity = JokerRarity::Common;

    const DESCRIPTION_FORMAT: &'static str =
        "如果打出的牌正好是4张牌\n这张小丑牌获得+4筹码\n（当前为+{}筹码）";

    const BASE_CHIPS: i64 = 0;

    const CHIPS_ADD: i64 = 4;

    // TODO
    const SPRITE_PATH: &'static str = "res://images/jokers/common_joker.jpg";

    pub fn new() -> Self {
        let mut joker = SquareJoker::default();
        joker.initialize();
        joker
    }
}
