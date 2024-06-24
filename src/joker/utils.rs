use godot::engine::Engine;

use crate::core::gamecore::GameCore;

/// 获得当前钱数
pub fn get_gold_display_info() -> i32 {
    let binding = Engine::singleton()
        .get_singleton("GameCore".into())
        .unwrap()
        .cast::<GameCore>();
    let gamecore = binding.bind();
    gamecore.gold
}
