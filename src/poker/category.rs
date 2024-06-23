use godot::obj::Gd;

use super::poker::PokerSprite;

// pub trait Category {

//     fn get_priority(&self) -> i32;
//     fn match_category(cards:&Vec<PokerSprite>) -> bool;
// }

// pub struct HighCard;
// impl Category for HighCard{
//     fn get_priority(&self) -> i32{
//         1
//     }
//     fn match_category(_cards:&Vec<PokerSprite>) -> bool{
//         true
//     }
// }

// pub struct OnePair;
// impl Category for OnePair{
//     fn get_priority(&self) -> i32{
//         2
//     }
//     fn match_category(cards:&Vec<PokerSprite>) -> bool{
//         let mut values = vec![];
//         for card in cards.iter(){
//             values.push(card.poker.get_value());
//         }
//         values.sort();
//         let mut past = values[0];
//         for i in 1..values.len(){
//             if past == values[i]{
//                 return true;
//             }
//             past = values[i];
//         }
//         false
//     }
// }
// pub struct TwoPair;
// impl Category for TwoPair{
//     fn get_priority(&self) -> i32{
//         3
//     }
//     fn match_category(cards:&Vec<PokerSprite>) -> bool{
//         let mut values = vec![];
//         for card in cards.iter(){
//             values.push(card.poker.get_value());
//         }
//         values.sort();
//         let mut past = values[0];
//         let mut count = 0;
//         for i in 1..values.len(){
//             if past == values[i]{
//                 count += 1;
//             }
//             past = values[i];
//         }
//         count == 2
//     }
// }

// pub struct ThreeOfAKind;
// impl Category for ThreeOfAKind{
//     fn get_priority(&self) -> i32{
//         4
//     }
//     fn match_category(cards:&Vec<PokerSprite>) -> bool{
//         let mut values = vec![];
//         for card in cards.iter(){
//             values.push(card.poker.get_value());
//         }
//         values.sort();
//         for i in 0..values.len()-2{
//             if values[i] == values[i+1] && values[i] == values[i+2]{
//                 return true;
//             }
//         }
//         false
//     }
// }

// pub struct Straight;
// impl Category for Straight{
//     fn get_priority(&self) -> i32{
//         5
//     }
//     // TODO
//     fn match_category(cards:&Vec<PokerSprite>) -> bool{
//         let mut values = vec![];
//         for card in cards.iter(){
//             values.push(card.poker.get_value());
//         }
//         values.sort();
//         // 处理特殊情况
//         if values[0] == 1 && values[1] == 10 && values[2] == 11 && values[3] == 12 && values[4] == 13{
//             return true;
//         }
//         let mut past = values[0];
//         for i in 1..values.len(){
//             if past + 1 != values[i]{
//                 return false;
//             }
//             past = values[i];
//         }
//         true
//     }
// }

// pub struct Flush;
// impl Category for Flush{
//     fn get_priority(&self) -> i32{
//         6
//     }
//     fn match_category(cards:&Vec<PokerSprite>) -> bool{
//         let suit = cards[0].poker.get_suit();
//         for card in cards.iter(){
//             if card.poker.get_suit() != suit{
//                 return false;
//             }
//         }
//         true
//     }
// }

// pub struct FullHouse;
// impl Category for FullHouse{
//     fn get_priority(&self) -> i32{
//         7
//     }
//     fn match_category(cards:&Vec<PokerSprite>) -> bool{
//         let mut values = vec![];
//         for card in cards.iter(){
//             values.push(card.poker.get_value());
//         }
//         values.sort();
//         if values[0] == values[1] && values[2] == values[3] && values[2] == values[4]{
//             return true;
//         }
//         if values[0] == values[1] && values[0] == values[2] && values[3] == values[4]{
//             return true;
//         }
//         false
//     }
// }

// pub struct FourOfAKind;
// impl Category for FourOfAKind{
//     fn get_priority(&self) -> i32{
//         8
//     }
//     fn match_category(cards:&Vec<PokerSprite>) -> bool{
//         let mut values = vec![];
//         for card in cards.iter(){
//             values.push(card.poker.get_value());
//         }
//         values.sort();
//         if values[0] == values[1] && values[0] == values[2] && values[0] == values[3]{
//             return true;
//         }
//         if values[1] == values[2] && values[1] == values[3] && values[1] == values[4]{
//             return true;
//         }
//         false
//     }
// }

// pub struct StraightFlush;
// impl Category for StraightFlush{
//     fn get_priority(&self) -> i32{
//         9
//     }
//     fn match_category(cards:&Vec<PokerSprite>) -> bool{
//         Straight::match_category(cards) && Flush::match_category(cards)
//     }
// }

// pub struct RoyalFlush;
// impl Category for RoyalFlush{
//     fn get_priority(&self) -> i32{
//         10
//     }
//     fn match_category(cards:&Vec<PokerSprite>) -> bool{
//         let mut values = vec![];
//         for card in cards.iter(){
//             values.push(card.poker.get_value());
//         }
//         values.sort();
//         if values[0] == 1 && values[1] == 10 && values[2] == 11 && values[3] == 12 && values[4] == 13{
//             return Flush::match_category(cards);
//         }
//         false
//     }
// }

/// 存储筹码和倍率信息, 可用于一次出牌的计算
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScoringInfo {
    /// 筹码
    pub chips: i64,
    /// 倍率
    pub mult: f64,
}

impl ScoringInfo {
    /// 计算筹码乘倍率
    pub fn get_score(&self) -> i64 {
        (self.chips as f64 * self.mult) as i64
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Null,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl Category {
    pub fn get_priority(&self) -> i32 {
        match self {
            Category::Null => 11,
            Category::HighCard => 1,
            Category::OnePair => 2,
            Category::TwoPair => 3,
            Category::ThreeOfAKind => 4,
            Category::Straight => 5,
            Category::Flush => 6,
            Category::FullHouse => 7,
            Category::FourOfAKind => 8,
            Category::StraightFlush => 9,
            Category::RoyalFlush => 10,
        }
    }

    /// 计算牌型
    pub fn match_category(&self, cards: &Vec<Gd<PokerSprite>>) -> bool {
        match self {
            Category::Null => cards.len() == 0,
            Category::HighCard => true,
            Category::OnePair => {
                let mut values = [0; 14];
                for card in cards.iter() {
                    values[card.bind().poker.get_value() as usize] += 1;
                }
                for i in 1..values.len() {
                    if values[i] == 2 {
                        return true;
                    }
                }
                false
            }
            Category::TwoPair => {
                let mut values = [0; 14];
                for card in cards.iter() {
                    values[card.bind().poker.get_value() as usize] += 1;
                }
                let mut count = 0;
                for i in 1..values.len() {
                    if values[i] >= 2 {
                        count += 1;
                    }
                }
                count == 2
            }
            Category::ThreeOfAKind => {
                let mut values = [0; 14];
                for card in cards.iter() {
                    values[card.bind().poker.get_value() as usize] += 1;
                }
                for i in 1..values.len() {
                    if values[i] == 3 {
                        return true;
                    }
                }
                false
            }
            Category::Straight => {
                let mut values = vec![];
                for card in cards.iter() {
                    values.push(card.bind().poker.get_value());
                }
                if values.len() != 5 {
                    return false;
                }
                values.sort();
                // 处理特殊情况
                if values[0] == 1
                    && values[1] == 10
                    && values[2] == 11
                    && values[3] == 12
                    && values[4] == 13
                {
                    return true;
                }
                let mut past = values[0];
                for i in 1..values.len() {
                    if past + 1 != values[i] {
                        return false;
                    }
                    past = values[i];
                }
                true
            }
            Category::Flush => {
                // TODO: 4张牌
                if cards.len() != 5 {
                    return false;
                }
                let suit = cards[0].bind().poker.get_suit();
                for card in cards.iter() {
                    if card.bind().poker.get_suit() != suit {
                        return false;
                    }
                }
                true
            }
            Category::FullHouse => {
                let mut values = [0; 14];
                for card in cards.iter() {
                    values[card.bind().poker.get_value() as usize] += 1;
                }
                let mut three = false;
                let mut two = false;
                for i in 1..values.len() {
                    if values[i] == 3 {
                        three = true;
                    }
                    if values[i] == 2 {
                        two = true;
                    }
                }
                three && two
            }
            Category::FourOfAKind => {
                let mut values = [0; 14];
                for card in cards.iter() {
                    values[card.bind().poker.get_value() as usize] += 1;
                }
                for i in 1..values.len() {
                    if values[i] == 4 {
                        return true;
                    }
                }
                false
            }
            Category::StraightFlush => {
                Category::Straight.match_category(cards) && Category::Flush.match_category(cards)
            }
            Category::RoyalFlush => {
                let mut values = vec![];
                for card in cards.iter() {
                    values.push(card.bind().poker.get_value());
                }
                if values.len() != 5 {
                    return false;
                }
                values.sort();
                if values[0] == 1
                    && values[1] == 10
                    && values[2] == 11
                    && values[3] == 12
                    && values[4] == 13
                {
                    return Category::Flush.match_category(cards);
                }
                false
            }
        }
    }

    /// 计算牌型对应的默认筹码和倍率
    pub fn get_chip_mag(&self) -> ScoringInfo {
        let (chips, mult) = match self {
            Category::Null => (0, 0.),
            Category::HighCard => (5, 1.),
            Category::OnePair => (10, 2.),
            Category::TwoPair => (20, 2.),
            Category::ThreeOfAKind => (30, 3.),
            Category::Straight => (30, 4.),
            Category::Flush => (35, 4.),
            Category::FullHouse => (40, 4.),
            Category::FourOfAKind => (60, 7.),
            Category::StraightFlush => (100, 8.),
            Category::RoyalFlush => (100, 8.),
        };
        ScoringInfo { chips, mult }
    }
}
