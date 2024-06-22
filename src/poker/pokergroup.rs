use std::borrow::BorrowMut;

use super::poker::PokerSprite;
use godot::{
    engine::{ColorRect, IColorRect, Label}, obj::WithBaseField, prelude::*
};

#[derive(GodotClass)]
#[class(base=ColorRect)]
pub struct PokerGroup {
    base: Base<ColorRect>,
    pub cards: Vec<Gd<PokerSprite>>,
    pub capacity: i32,
    pub z_buffer:Vec<i32>,
}
#[godot_api]
impl IColorRect for PokerGroup {
    fn init(base: Base<ColorRect>) -> Self {
        PokerGroup {
            base,
            cards: Vec::new(),
            capacity: 0,
            z_buffer:Vec::new(),
        }
    }
    fn ready(&mut self){
    }
    fn process(&mut self, _delta: f64) {
        // 需要处理不同数量的扑克牌在手中的渲染
        let count = self.cards.len();
        let rect = self.base().get_global_rect();
        let (position,center, width, _height) = (rect.position, rect.center(),rect.size.x, rect.size.y);
        
        if self.z_buffer.len() > 0{
            godot_print!("process z_buffer: {:?}",self.z_buffer);
            let max = self.z_buffer.iter().max().unwrap();
            for i in 0..self.cards.len(){
                if self.cards[i].bind().base().get_z_index() == *max{
                    self.cards[i].bind_mut().clicked();
                    break;
                }
            }
            self.z_buffer.clear();
        }        
        
        for i in 0..count {
            // 需要计算每张牌的位置
            let x = position.x + 20.0 + i as f32 * width / count as f32;
            let y = center.y; // TODO: 弧形排列更美观
            let mut bind = self.cards[i].borrow_mut().bind_mut();
            let y = y + if bind.is_selected { -15.0 } else { 0.0 };
            let mut card = bind.base_mut();

            card.set_position(Vector2::new(x, y) - position);
            card.set_z_index(i as i32);

        }

        // 标签内容修改
        let capacity = self.capacity;
        self.base_mut()
            .get_node_as::<Label>("./Label")
            .set_text(format!("{} / {}", count, capacity).into());



        self.base_mut().set_process(false);
    }
}

#[godot_api]
impl PokerGroup {}
