use crate::{
    card::card::{ICard, IUseableCard},
    poker::{
        category::{Category, ScoringInfo},
        poker::{Poker, PokerSprite},
    },
};
use godot::{
    engine::{ISprite2D, Sprite2D},
    prelude::*,
};

// 稀有度
#[derive(Default, Clone)]
pub enum JokerRarity {
    #[default]
    Common,
    Uncommon,
    Rare,
    Legendary,
}

pub trait IJoker {
    /// 初始化
    fn initialize(&mut self);

    fn on_equip(&mut self) {}
    fn on_another_card_bought(&mut self, _card: &dyn ICard) {}
    fn on_destroyed(&mut self) {}
    fn on_another_card_destroyed(&mut self, _card: &dyn ICard) {}
    fn on_sold(&mut self) {}
    fn on_another_card_sold(&mut self, _card: &dyn ICard) {}

    /// 打出手牌时触发, 如裤子
    fn on_play_card(&mut self, _cards: &Vec<Gd<PokerSprite>>, pokerhand: Category) {}

    /// 弃牌时触发, 如城堡
    fn on_discard_card(&mut self, _cards: &Vec<Gd<PokerSprite>>, pokerhand: Category) {}
    fn can_be_duplicated(&self) -> bool {
        true
    }
    fn can_be_sold(&self) -> bool {
        true
    }
    fn on_enter_room(&mut self) {}
    fn on_battle_win(&mut self) {}
    fn on_card_used(&mut self, _card: &dyn IUseableCard) {}

    /// 牌对小丑牌的作用, 如“每打出一个红桃+3倍率”
    fn cal_card_chip_mag(
        &mut self,
        _score: &mut ScoringInfo,
        _cards: &mut Vec<Gd<PokerSprite>>,
        _category: Category,
    ) {
    }

    /// 小丑牌对打出牌组的作用, 如“打出牌数少于3时+20倍率”
    fn cal_final_chip_mag(
        &mut self,
        _score: &mut ScoringInfo,
        _cards: &mut Vec<Gd<PokerSprite>>,
        _category: Category,
    ) {
    }
    fn on_skip_blind(&mut self) {}
}

pub trait IJokerCard: ICard + IJoker {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct JokerSprite {
    pub base: Base<Sprite2D>,
    pub joker: Option<Box<dyn IJokerCard>>,
}

#[godot_api]
impl ISprite2D for JokerSprite {
    fn init(base: Base<Sprite2D>) -> Self {
        JokerSprite { base, joker: None }
    }
}
