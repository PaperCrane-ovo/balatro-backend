use crate::{
    card::card::{ICard, IUseableCard},
    poker::{
        category::{Category, ScoringInfo},
        poker::PokerSprite,
    },
};
use dyn_clone::DynClone;
use godot::{
    engine::{ISprite2D, Sprite2D, Texture2D},
    prelude::*,
};

// 稀有度
#[derive(GodotConvert, Var, Export, Default, Clone, Copy)]
#[godot(via=GString)]
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

    fn get_display_info(&self) -> Gd<JokerDisplayInfo>;

    #[allow(unused_variables, dead_code)]
    fn on_equip(&mut self) {}

    #[allow(unused_variables, dead_code)]
    fn on_another_card_bought(&mut self, _card: &dyn ICard) {}

    #[allow(unused_variables, dead_code)]
    fn on_destroyed(&mut self) {}

    #[allow(unused_variables, dead_code)]
    fn on_another_card_destroyed(&mut self, _card: &dyn ICard) {}

    #[allow(unused_variables, dead_code)]
    fn on_sold(&mut self) {}

    #[allow(unused_variables, dead_code)]
    fn on_another_card_sold(&mut self, _card: &dyn ICard) {}

    /// 打出手牌时触发, 算分前, 如裤子
    #[allow(unused_variables)]
    fn on_play_hand(&mut self, hands: &Vec<Gd<PokerSprite>>, pokerhand: Category) {}

    /// 弃牌时触发, 如城堡
    #[allow(unused_variables)]
    fn on_discard_poker(&mut self, hands: &Vec<Gd<PokerSprite>>, pokerhand: Category) {}

    #[allow(unused_variables, dead_code)]
    fn can_be_duplicated(&self) -> bool {
        true
    }

    #[allow(unused_variables, dead_code)]
    fn can_be_sold(&self) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn on_enter_room(&mut self) {}

    #[allow(unused_variables, dead_code)]
    fn on_battle_win(&mut self) {}

    #[allow(unused_variables, dead_code)]
    fn on_card_used(&mut self, _card: &dyn IUseableCard) {}

    /// 计算单卡得分时调用, 如“每打出一个红桃+3倍率”
    #[allow(unused_variables)]
    fn on_calculate_poker_score(
        &mut self,
        score_info: &mut ScoringInfo,
        hands: &mut Vec<Gd<PokerSprite>>,
        pokerhand: Category,
    ) {
    }

    /// 最后计算整手牌得分时调用, 如“打出牌数少于3时+20倍率”
    #[allow(unused_variables)]
    fn post_calculate_poker_score(
        &mut self,
        score_info: &mut ScoringInfo,
        hands: &mut Vec<Gd<PokerSprite>>,
        pokerhand: Category,
    ) {
    }

    #[allow(dead_code)]
    fn on_skip_blind(&mut self) {}
}

pub trait IJokerSpritePath {
    fn get_sprite_path(&self) -> String;
}

pub trait IJokerCard: ICard + IJoker + IJokerSpritePath + DynClone {}

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

#[godot_api]
impl JokerSprite {
    pub fn set_texture(&mut self) {
        if let Some(joker) = &self.joker {
            let texture = load::<Texture2D>(joker.get_sprite_path());
            self.base_mut().set_texture(texture);
        }
    }
    #[func]
    pub fn get_display_info(&self) -> Gd<JokerDisplayInfo> {
        self.joker.as_ref().unwrap().get_display_info()
    }
}

#[derive(GodotClass)]
#[class(init)]
pub struct JokerDisplayInfo {
    #[var]
    pub name: GString,
    #[var]
    pub description: GString,
    #[var]
    pub rarity: JokerRarity,
    #[var]
    pub price: i32,
}
