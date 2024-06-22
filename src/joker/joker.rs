use crate::{card::card::{ICard, IUseableCard}, poker::{category::Category, poker::{Poker, PokerSprite}}};
use godot::{engine::{ISprite2D, Sprite2D}, prelude::*};

// 稀有度
#[derive(Clone)]
pub enum JokerRarity{
    Common,
    Uncommon,
    Rare,
    Legendary,
}

pub trait IJoker{
    fn initialize(&mut self);
    fn on_equip(&mut self){

    }
    fn on_another_card_bought(&mut self,_card:&dyn ICard){

    }
    fn on_destroyed(&mut self){

    }
    fn on_another_card_destroyed(&mut self,_card:&dyn ICard){

    }
    fn on_sold(&mut self){

    }
    fn on_another_card_sold(&mut self,_card:&dyn ICard){

    }
    fn on_play_card(&mut self,_cards:&Vec<Gd<PokerSprite>>,_category:Category){
        
    }
    fn on_discard_card(&mut self,_cards:&Vec<Gd<PokerSprite>>,_category:Category){
        
    }
    fn can_be_duplicated(&self) -> bool{
        true
    }
    fn can_be_sold(&self) -> bool{
        true
    }
    fn on_enter_room(&mut self){

    }
    fn on_battle_win(&mut self){

    }
    fn on_card_used(&mut self,_card:&dyn IUseableCard){

    }
    fn cal_card_chip_mag(&mut self,_score:&mut Vec<i32>,_cards:&mut Vec<Gd<PokerSprite>>,_category:Category){

    }
    fn cal_final_chip_mag(&mut self,_score:&mut Vec<i32>,_cards:&mut Vec<Gd<PokerSprite>>,_category:Category){

    }
    fn on_skip_blind(&mut self){

    }

}

pub trait IJokerCard:ICard+IJoker{}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct JokerSprite{
    pub base: Base<Sprite2D>,
    pub joker: Option<Box<dyn IJokerCard>>,
}

#[godot_api]
impl ISprite2D for JokerSprite{
    fn init(base: Base<Sprite2D>) -> Self{
        JokerSprite{
            base,
            joker: None,
        }
    }

}