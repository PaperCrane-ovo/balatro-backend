use godot::{
    builtin::GString,
    register::{Export, GodotConvert, Var},
};
pub mod blind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum BlindType {
    SmallBlind,
    BigBlind,
    BossBlind,
}

impl Default for BlindType {
    fn default() -> Self {
        BlindType::SmallBlind
    }
}
#[derive(Debug, GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum BlindState {
    NotChoose,
    Choose,
    Skip,
    Killed,
}

impl Default for BlindState {
    fn default() -> Self {
        BlindState::NotChoose
    }
}
