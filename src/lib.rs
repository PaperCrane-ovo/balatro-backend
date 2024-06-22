use core::gamecore::GameCore;

use godot::{engine::Engine, prelude::*};

mod core;
mod joker;
mod poker;
mod player;
mod card;
mod blind;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // Engine::singleton()
            //     .register_singleton(
            //         StringName::from("GameCore"),
            //          GameCore::new_alloc().upcast());
            let mut engine = Engine::singleton();
            godot_print!("engine");
            let singleton_name = StringName::from("GameCore");
            godot_print!("singleton_name");
            let singleton = GameCore::new_alloc();
            godot_print!("singleton alloc");
            let singleton = singleton.upcast();
            godot_print!("singleton upcast");
            engine.register_singleton(singleton_name, singleton);
            godot_print!("GameCore singleton registered");
        }
    }
    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let singleton_name = StringName::from("GameCore");
            let singleton = engine
                .get_singleton(singleton_name.clone())
                .expect("Singleton not found");
            engine.unregister_singleton(singleton_name);
            singleton.free();
        }
    }
}
