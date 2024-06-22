/// 消耗牌和小丑牌的价格性
pub trait ICard {
    fn get_price(&self) -> i32;
    fn set_price(&mut self, price: i32);
}
pub trait IUseableCard {
    fn use_card(&self);
    fn can_use_without_hand(&self) -> bool;
}
