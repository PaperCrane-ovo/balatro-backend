use godot::{
    engine::{Engine},
    prelude::*,
};

use crate::{core::gamecore::GameCore};

#[derive(GodotClass)]
#[class(init,base=Node2D)]
pub struct Player {
    base: Base<Node2D>,
}

// #[godot_api]
// impl INode2D for Player{
//     fn init(base: Base<Node2D>) -> Self{
//         Player{
//             base,

//         }
//     }

// }

#[godot_api]
impl Player {
    pub fn initialize_message(
        &mut self,
        _gold: i32,
        _round: i32,
        _play_hand_count: i32,
        _discard_count: i32,
        _cur_ante: i32,
        _max_ante: i32,
    ) {
        let mut binding = Engine::singleton()
            .get_singleton("GameCore".into())
            .unwrap()
            .cast::<GameCore>();
        let mut gamecore = binding.bind_mut();
        gamecore.gold = _gold;
        gamecore.round = _round;
        gamecore.play_hand_count = _play_hand_count;
        gamecore.discard_count = _discard_count;
        gamecore.cur_ante = _cur_ante;
        gamecore.max_ante = _max_ante;
    }

    #[func]
    pub fn get_message(&mut self) -> Dictionary {
        let binding = Engine::singleton()
            .get_singleton("GameCore".into())
            .unwrap()
            .cast::<GameCore>();
        let gamecore = binding.bind();
        let mut dict = Dictionary::new();
        dict.insert("gold", gamecore.gold);
        dict.insert("round", gamecore.round);
        dict.insert("play_hand_count", gamecore.play_hand_count);
        dict.insert("discard_count", gamecore.discard_count);
        dict.insert("cur_ante", gamecore.cur_ante);
        dict.insert("max_ante", gamecore.max_ante);
        dict
    }
}
