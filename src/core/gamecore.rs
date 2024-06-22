use godot::{
    engine::Time,
    prelude::*,
};

use crate::{
    blind::{blind::Blind, BlindState, BlindType},
    joker::{commonjoker::CommonJoker, joker::JokerSprite},
    poker::{category::Category, poker::{Poker, PokerSprite}},
};

use super::random::LinearCongruentialGenerator;

#[derive(GodotConvert,Var,Export)]
#[godot(via = GString)]
pub enum GameState{
    Win,
    Lose,
    StillPlaying,
    FinalWin,
}
impl Default for GameState{
    fn default() -> Self{
        GameState::StillPlaying
    }
}


#[derive(GodotClass)]
#[class(init,base=Object)]
pub struct GameCore {
    base: Base<Object>,

    pub poker_pool: Vec<Gd<PokerSprite>>,
    pub joker_pool: Vec<Gd<JokerSprite>>,

    pub boss_rng: Gd<LinearCongruentialGenerator>,
    pub joker_rng: Gd<LinearCongruentialGenerator>,
    pub shop_rng: Gd<LinearCongruentialGenerator>,
    pub shuffle_rng: Gd<LinearCongruentialGenerator>,
    #[export]
    pub game_seed: i64,

    pub selected_pokers: Vec<Gd<PokerSprite>>,

    #[export]
    pub cur_ante: i32,
    #[export]
    pub max_ante: i32,

    pub draw_pile: Vec<Gd<PokerSprite>>,
    pub discard_pile: Vec<Gd<PokerSprite>>,
    pub hand_pile: Vec<Gd<PokerSprite>>,

    #[export]
    pub gold: i32,
    #[export]
    pub round: i32,
    #[export]
    pub play_hand_count: i32,
    #[export]
    pub discard_count: i32,

    #[var]
    pub hand_limit: i32,

    
    pub blinds: Vec<Gd<Blind>>,
    #[var]
    pub cur_blind_index: i32,

    pub joker_list: Vec<Gd<JokerSprite>>,
    pub poker_deck: Vec<Gd<PokerSprite>>,

    pub categories: Vec<Category>,
    #[var]
    pub cur_score:i64,
    #[var]
    pub this_round_score: Array<i64>,
    #[var]
    pub game_state:GameState,
}

#[godot_api]
impl GameCore {
    /// 游戏核心初始化
    #[func]
    pub fn initialize(&mut self) {
        godot_print!("GameCore initialize");
        self.generate_random_seed();
        godot_print!("GameCore generate_random_seed:{}", self.game_seed);
        self.init_rng(self.game_seed);
        godot_print!("GameCore init_rng");
        self.generate_poker_pool();
        godot_print!("GameCore generate_poker_pool");
        self.generate_joker_pool();
        godot_print!("GameCore generate_joker_pool");
        self.initialize_player_message();
        godot_print!("GameCore initialize_player_message");
        self.initialize_blind();
        godot_print!("GameCore initialize_blind");
        self.initialize_categories();
        godot_print!("GameCore initialize_poker_deck");
        self.initialize_poker_deck();

        self.this_round_score.resize(2, &0);
        self.game_state = GameState::StillPlaying;
    }

    #[func]
    pub fn get_blinds(&self) -> Array<Gd<Blind>>{
        let mut blinds = Array::new();
        for i in self.blinds.iter(){
            blinds.push(i.clone());
        }
        blinds
    }



    #[func]
    #[deprecated(note = "use get_blinds instead")]

    pub fn get_blind_state(&self) -> Array<i32> {
        godot_print!("get_blinds");
        let mut state = Array::new();
        for i in 0..3 {
            state.insert(i, {
                match self.blinds[i].bind().state {
                    BlindState::NotChoose => 0,
                    BlindState::Choose => 1,
                    BlindState::Skip => 2,
                    BlindState::Killed => 3,
                }
            })
        }
        state
    }
    #[func]
    pub fn set_blinds(&mut self, state:Array<i32>){
        for i in 0..3{
            self.blinds[i].bind_mut().state = match state.get(i){
                0 => BlindState::NotChoose,
                1 => BlindState::Choose,
                2 => BlindState::Skip,
                3 => BlindState::Killed,
                _ => BlindState::NotChoose,
            }
        }
    }

    #[func]
    pub fn get_specific_pile_count(&self,pile:StringName) -> i32{
        match pile.to_string().as_str(){
            "draw_pile" => self.draw_pile.len() as i32,
            "discard_pile" => self.discard_pile.len() as i32,
            "hand_pile" => self.hand_pile.len() as i32,
            _ => 0,
        }
    }



    #[func]
    pub fn choose_current_blind(&mut self){
        godot_print!("choose_current_blind");
        // 给joker钩子
        for joker in self.joker_list.iter_mut(){
            if let Some(joker_card) = joker.bind_mut().joker.as_mut() {
                joker_card.on_enter_room();
            }
        }
        // 选择盲注后做初始化工作
        // 1. 重置玩家抽牌堆
        for poker in self.poker_deck.iter(){
            self.draw_pile.push(poker.clone());
        }
        // 2. 洗牌
        let mut draw_pile = &mut self.draw_pile;
        let mut shuffle_rng = self.shuffle_rng.bind_mut();

        GameCore::shuffle(&mut draw_pile, &mut shuffle_rng);
        // 3. 重置得分
        self.cur_score = 0;
    }


    // #[func]
    // pub fn skip_current_blind(&mut self){
    //     godot_print!("skip_current_blind");
    //     if let BlindType::BossBlind = self.cur_blind{
    //         godot_error!("can't skip boss blind");
    //         return;
    //     }
    //     self.blind_state[self.cur_blind as usize] = BlindState::Skip;
    //     self.cur_blind = match self.cur_blind{
    //         BlindType::SmallBlind => BlindType::BigBlind,
    //         BlindType::BigBlind => BlindType::BossBlind,
    //         BlindType::BossBlind => unreachable!("can't skip boss blind"),
    //     };
    //     self.blind_state[self.cur_blind as usize] = BlindState::Choose;
    //     for joker in self.joker_list.iter_mut(){
    //         if let Some(joker_card) = joker.bind_mut().joker.as_mut() {
    //             joker_card.on_skip_blind();
    //         }
    //     }
    // }


    #[func]
    pub fn on_play_card(&mut self){
        let played_category = {
            let mut _category = Category::HighCard;
            for category in self.categories.iter(){
                if category.match_category(&self.selected_pokers){
                    _category = *category;
                    break;
                }
            }
            _category
        };
        // 打出某种牌型时，触发joker钩子
        for joker in self.joker_list.iter_mut(){
            if let Some(joker_card) = joker.bind_mut().joker.as_mut() {
                joker_card.on_play_card(&self.selected_pokers,played_category);
            }
        }
        // 3. 处理算分
        let score = played_category.get_chip_mag();
        let mut score = vec![score.0 as i32,score.1 as i32]; // 牌型带来的筹码和倍率
        for i in self.selected_pokers.iter(){ // 扑克牌带来的筹码
            score[0] += i.bind().poker.get_chip();
        }
        for i in self.joker_list.iter_mut(){ // 小丑牌带来的筹码和倍率加
            if let Some(joker_card) = i.bind_mut().joker.as_mut() {
                joker_card.cal_card_chip_mag(&mut score, &mut self.selected_pokers,played_category);
            }
        }
        for i in self.joker_list.iter_mut(){ // 小丑牌带来的倍率乘
            if let Some(joker_card) = i.bind_mut().joker.as_mut(){
                joker_card.cal_final_chip_mag(&mut score, &mut self.selected_pokers, played_category);
            }
        }

        self.this_round_score.set(0, score[0] as i64);
        self.this_round_score.set(1, score[1] as i64);

        self.cur_score += score[0] as i64 * score[1] as i64;

        self.play_hand_count -= 1;
    }

    #[func]
    pub fn on_discard_card(&mut self){
        let played_category = {
            let mut _category = Category::HighCard;
            for category in self.categories.iter(){
                if category.match_category(&self.selected_pokers){
                    _category = *category;
                    break;
                }
            }
            _category
        };
        // 弃掉某种牌型时触发joker钩子
        for joker in self.joker_list.iter_mut(){
            if let Some(joker_card) = joker.bind_mut().joker.as_mut() {
                joker_card.on_discard_card(&self.selected_pokers,played_category);
            }
        }
        // TODO: 弃牌触发蜡封
        self.discard_count -= 1;

    }

    #[func]
    pub fn move_selected_pokers_to_discard_pile(&mut self){
        for i in self.selected_pokers.iter(){
            self.discard_pile.push(i.clone());
        }
        self.selected_pokers.clear();
        self.hand_pile.retain(|x| x.bind().is_selected == false);

    }



    #[func]
    pub fn draw_pokers(&mut self,count: i32){
        for _ in 0..count{
            if let Some(poker) = self.draw_pile.pop(){
                self.hand_pile.push(poker);
            }
        }
    }

    #[func]
    pub fn get_hand_pile(&self) -> Array<Gd<PokerSprite>>{
        let mut hand_pile = Array::new();
        for i in self.hand_pile.iter(){
            hand_pile.push(i.clone());
        }
        hand_pile
    }

    

    #[func]
    pub fn get_category(&mut self) -> StringName{
        let mut category = Category::Null;
        for i in 0..self.categories.len(){
            if self.categories[i].match_category(&self.selected_pokers){
                category = self.categories[i];
                break;
            }
        }
        self.this_round_score.set(0,category.get_chip_mag().0 as i64);
        self.this_round_score.set(1,category.get_chip_mag().1 as i64);
        match category {
            // Category::HighCard => "HighCard".into(),
            // Category::OnePair => "OnePair".into(),
            // Category::TwoPair => "TwoPair".into(),
            // Category::ThreeOfAKind => "ThreeOfAKind".into(),
            // Category::Straight => "Straight".into(),
            // Category::Flush => "Flush".into(),
            // Category::FullHouse => "FullHouse".into(),
            // Category::FourOfAKind => "FourOfAKind".into(),
            // Category::StraightFlush => "StraightFlush".into(),
            // Category::RoyalFlush => "RoyalFlush".into(),
            Category::Null => "".into(),
            Category::HighCard => "高牌".into(),
            Category::OnePair => "对子".into(),
            Category::TwoPair => "两对".into(),
            Category::ThreeOfAKind => "三条".into(),
            Category::Straight => "顺子".into(),
            Category::Flush => "同花".into(),
            Category::FullHouse => "葫芦".into(),
            Category::FourOfAKind => "四条".into(),
            Category::StraightFlush => "同花顺".into(),
            Category::RoyalFlush => "皇家同花顺".into(),
        }

    }

    #[func]
    pub fn has_selected_pokers(&self) -> bool{
        !self.selected_pokers.is_empty()
    }

    #[func]
    pub fn add_to_selected_cards(&mut self,poker:Gd<PokerSprite>) -> bool{
        let size = self.selected_pokers.len();
        if size >=5{
            return false;
        }
        self.selected_pokers.push(poker);
        return true;
    }
    #[func]
    pub fn remove_from_selected_cards(&mut self,poker:Gd<PokerSprite>){
        self.selected_pokers.retain(|x| x != &poker);
    }
    



    pub fn initialize_blind(&mut self){
        // TODO: BossBlind
        self.blinds.clear();
        let mut small_blind = Blind::new(BlindType::SmallBlind);
        let mut big_blind = Blind::new(BlindType::BigBlind);
        let mut boss_blind = Blind::new(BlindType::BossBlind);
        small_blind.state = BlindState::Choose;
        small_blind.init_hp(self.cur_ante);
        big_blind.init_hp(self.cur_ante);
        boss_blind.init_hp(self.cur_ante);
        small_blind.init_award();
        big_blind.init_award();
        boss_blind.init_award();
        self.blinds.push(Gd::from_object(small_blind));
        self.blinds.push(Gd::from_object(big_blind));
        self.blinds.push(Gd::from_object(boss_blind));

        self.cur_blind_index = 0;

    }

    pub fn initialize_player_message(&mut self) {
        self.cur_ante = 1;
        self.max_ante = 8;
        self.round = 0;
        self.gold = 4;
        self.play_hand_count = 4;
        self.discard_count = 3;
        self.hand_limit = 12;

        self.cur_score = 0;
    }

    pub fn init_rng(&mut self, seed: i64) {
        self.game_seed = seed;
        self.boss_rng.bind_mut().set_seed(seed);
        self.joker_rng.bind_mut().set_seed(seed);
        self.shop_rng.bind_mut().set_seed(seed);
        self.shuffle_rng.bind_mut().set_seed(seed);
    }

    pub fn initialize_categories(&mut self){
        self.categories.push(Category::HighCard);
        self.categories.push(Category::OnePair);
        self.categories.push(Category::TwoPair);
        self.categories.push(Category::ThreeOfAKind);
        self.categories.push(Category::Straight);
        self.categories.push(Category::Flush);
        self.categories.push(Category::FullHouse);
        self.categories.push(Category::FourOfAKind);
        self.categories.push(Category::StraightFlush);
        self.categories.push(Category::RoyalFlush);
        self.categories.push(Category::Null);
        self.categories.sort_by_key(|x| -x.get_priority());
    }

    pub fn generate_random_seed(&mut self) -> i64 {
        let mut time = Time::singleton().get_datetime_dict_from_system();
        let mut seeds: Vec<u64> = Vec::new();

        time.remove("dst");
        for i in time.values_array().iter_shared() {
            seeds.push(i.to::<u64>());
        }
        seeds.push(Time::singleton().get_unix_time_from_system() as u64);
        godot_print!("time:{:?}", seeds);
        let v: Vec<u64> = vec![2024, 05, 06, 2003, 01, 13, 957, 5755];
        let mut seed = 0;
        for i in 0..seeds.len() {
            seed = u64::wrapping_add(
                seed,
                u64::wrapping_mul(
                    u64::wrapping_mul(seeds[i] as u64, v[i] as u64),
                    (i * 20030113793380) as u64,
                ),
            );
        }

        godot_print!("game seed:{}", seed);
        self.game_seed = seed as i64;
        seed as i64
    }

    pub fn generate_poker_pool(&mut self) {
        self.poker_pool.clear();
        for i in 0..4 {
            for j in 1..14 {
                let mut poker = Poker::new();
                poker.initialize(i, j);
                let mut poker_sprite = Gd::from_init_fn(|base| {
                    PokerSprite{
                        base,
                        poker,
                        is_selected: false,
                        object_id: None,
                    }
                });
                let id = poker_sprite.instance_id();
                poker_sprite.bind_mut().set_object_id(id);
                poker_sprite.bind_mut().set_texture();
                
                self.poker_pool.push(poker_sprite);
            }
        }
    }

    pub fn generate_joker_pool(&mut self) {
        // TODO
        self.joker_pool.clear();
        self.joker_pool.push(Gd::from_init_fn(|base| {JokerSprite{base, joker: Some(Box::new(CommonJoker::new()))}}));


    }

    pub fn initialize_poker_deck(&mut self){
        for i in self.poker_pool.iter(){
            self.poker_deck.push(i.clone());
        }
    }

    fn shuffle<T>(list: &mut Vec<T>,rng:&mut GdMut<LinearCongruentialGenerator>) {
        let mut n = list.len();
        godot_print!("shuffle list len:{}",n);
        while n > 1 {
            let k = rng.gen() as usize % n;
            n -= 1;
            list.swap(n, k);
        }
    }
}
