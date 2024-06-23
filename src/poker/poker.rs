use godot::engine::utilities::randf_range;

use godot::engine::ISprite2D;
use godot::engine::Sprite2D;
use godot::engine::Texture2D;
use godot::obj::WithBaseField;
use godot::prelude::*;

/// 花色
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Suit {
    /// 梅花
    Clubs,
    /// 方片
    Diamonds,
    /// 黑桃
    Spades,
    /// 红桃
    Hearts,
    NotUsed,
}

impl Suit {
    #[allow(dead_code)]
    /// 返回花色名称：方片、红桃、黑桃、梅花
    pub fn get_display_suit_name(&self) -> String {
        match self {
            Self::Clubs => "梅花".to_string(),
            Self::Diamonds => "方片".to_string(),
            Self::Spades => "黑桃".to_string(),
            Self::Hearts => "红桃".to_string(),
            _ => "ERROR".to_string(),
        }
    }
}
#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct PokerSprite {
    pub base: Base<Sprite2D>,
    pub poker: Poker,
    #[var]
    pub is_selected: bool,
    pub object_id: Option<InstanceId>,
}

/// 扑克牌
#[derive(Copy, Clone, Debug)]
pub struct Poker {
    /// 花色
    pub suit: Suit,
    pub value: i64,
    /// 基础筹码
    pub base_chip: i64,
    /// 额外筹码
    pub extra_chip: i64,
    pub is_valid: bool,
}

impl Poker {
    pub fn new() -> Self {
        Self {
            suit: Suit::NotUsed,
            value: 0,
            base_chip: 0,
            extra_chip: 0,
            is_valid: false,
        }
    }
}

#[godot_api]
impl ISprite2D for PokerSprite {
    fn init(base: Base<Sprite2D>) -> Self {
        PokerSprite {
            base,
            poker: Poker::new(),
            is_selected: false,
            object_id: None,
        }
    }
    fn ready(&mut self) {
        self.base_mut()
            .set_rotation_degrees(randf_range(-5.0, 5.0) as f32);
    }
}
#[godot_api]
impl PokerSprite {
    pub fn set_texture(&mut self) {
        let num = match self.poker.get_suit() {
            Suit::Spades => 0,
            Suit::Hearts => 13,
            Suit::Clubs => 26,
            Suit::Diamonds => 39,
            _ => 4,
        } + self.poker.get_value();
        let texture = load::<Texture2D>(format!("res://images/pokers/{}.jpg", num));
        self.base_mut().set_texture(texture);
    }
    pub fn clicked(&mut self) {
        // godot_print!("PokerSprite clicked");
        // let gamecore = godot::engine::Engine::singleton().get_singleton("GameCore".into());
        // if let Some(node) = gamecore {
        //     let node = node.try_cast::<GameCore>();
        //     if let Ok(mut node) = node {
        //         let mut gamecore = node.bind_mut();
        //         let selected_pokers = &mut gamecore.selected_pokers;
        //         if !self.is_selected {
        //             if selected_pokers.len() < 5 {
        //                 let object = Gd::<PokerSprite>::try_from_instance_id(self.object_id.unwrap());
        //                 if let Ok(object) = object {
        //                     selected_pokers.push(object);
        //                     self.selected();
        //                 }
        //             }
        //         } else {
        //             selected_pokers.retain(|x| x != &Gd::from_instance_id(self.object_id.unwrap()));
        //             self.unselected();
        //         }
        //     }
        // }
    }
    #[func]
    pub fn selected(&mut self) {
        self.is_selected = true;
    }
    #[func]
    pub fn unselected(&mut self) {
        self.is_selected = false;
    }

    pub fn set_object_id(&mut self, object_id: InstanceId) {
        self.object_id = Some(object_id);
    }
    #[func]
    pub fn get_value_to_sort(&self) -> i64 {
        let ret = self.poker.get_value();
        if ret == 1 {
            14
        } else {
            ret
        }
    }
}

impl Poker {
    pub fn initialize(&mut self, suit: i32, value: i64) {
        self.suit = match suit {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Clubs,
            3 => Suit::Diamonds,
            _ => Suit::NotUsed,
        };
        self.value = value;
        // self.value_str = match value{
        //     1 => "A".to_string(),
        //     11 => "J".to_string(),
        //     12 => "Q".to_string(),
        //     13 => "K".to_string(),
        //     _ => value.to_string(),
        // };
        self.base_chip = match value {
            1 => 11,
            11 | 12 | 13 => 10,
            _ => value,
        };
        self.extra_chip = 0;
        self.is_valid = true;
    }
    pub fn get_chip(&self) -> i64 {
        if self.get_valid() {
            self.base_chip + self.extra_chip
        } else {
            0
        }
    }
    pub fn set_extra_chip(&mut self, chip: i64) {
        self.extra_chip = chip;
    }
    pub fn set_valid(&mut self, is_valid: bool) {
        self.is_valid = is_valid;
    }
    pub fn get_valid(&self) -> bool {
        self.is_valid
    }
    pub fn get_suit(&self) -> Suit {
        self.suit
    }
    pub fn get_value(&self) -> i64 {
        self.value
    }
}
