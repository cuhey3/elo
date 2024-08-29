use std::cmp;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Tournament {
    pub id: u32,
    pub name: String,
}

#[derive(Clone)]
pub struct TournamentSub {
    pub id: u32,
    pub name: String,
}

#[derive(Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
}

#[derive(Clone)]
pub struct Character {
    pub id: u32,
    pub name: String,
}

pub struct EloSetting {
    player_list: Vec<Player>,
    tournament_list: Vec<Tournament>,
    tournament_sub_list: Vec<TournamentSub>,
    character_list: Vec<Character>,
}

impl EloSetting {
    pub fn new(
        player_list: Vec<Player>,
        tournament_list: Vec<Tournament>,
        tournament_sub_list: Vec<TournamentSub>,
        character_list: Vec<Character>,
    ) -> EloSetting {
        EloSetting {
            player_list,
            tournament_list,
            tournament_sub_list,
            character_list,
        }
    }
    pub fn find_player(&self, id: u32) -> Option<&Player> {
        self.player_list.iter().find(|p| p.id == id)
    }
    fn find_tournament(&self, id: u32) -> Option<&Tournament> {
        self.tournament_list.iter().find(|p| p.id == id)
    }
    fn find_tournament_sub(&self, id: u32) -> Option<&TournamentSub> {
        self.tournament_sub_list.iter().find(|p| p.id == id)
    }
    fn find_character(&self, id: u32) -> Option<&Character> {
        self.character_list.iter().find(|p| p.id == id)
    }
    fn create_rating_map(&self) -> HashMap<u32, f64> {
        let mut rating_map: HashMap<u32, f64> = HashMap::new();
        for p in self.player_list.iter() {
            rating_map.insert(p.id, 1500_f64);
        }
        rating_map
    }
}

pub struct EloRecord {
    date: u64,
    tournament_id: u32,
    tournament_sub_id: u32,
    player_id: u32,
    character_id: u32,
    opponent_player_id: u32,
    opponent_character_id: u32,
    win_count: u32,
    lose_count: u32,
}

impl EloRecord {
    pub fn new(
        tournament_id: u32,
        tournament_sub_id: u32,
        date: u64,
        player_id: u32,
        character_id: u32,
        opponent_player_id: u32,
        opponent_character_id: u32,
        win_count: u32,
        lose_count: u32,
    ) -> EloRecord {
        EloRecord {
            date,
            tournament_id,
            tournament_sub_id,
            player_id,
            character_id,
            opponent_player_id,
            opponent_character_id,
            win_count,
            lose_count,
        }
    }
}

const K: f64 = 16_f64;

pub fn get_player_rating(
    elo_setting: EloSetting,
    record_list: Vec<EloRecord>,
) -> HashMap<u32, f64> {
    let mut rating_map = elo_setting.create_rating_map();
    for r in record_list.iter() {
        let player_rating = rating_map.get(&r.player_id);
        if player_rating.is_none() {
            continue;
        }
        let opponent_player_rating = rating_map.get(&r.opponent_player_id);
        if opponent_player_rating.is_none() {
            continue;
        }
        let mut player_rating = player_rating.unwrap().to_owned();
        let mut opponent_player_rating = opponent_player_rating.unwrap().to_owned();
        let win_lose_pair_count = cmp::min(r.win_count, r.lose_count);
        let win_lose_over_count = cmp::max(r.win_count, r.lose_count) - win_lose_pair_count;
        let win_flag = r.win_count >= r.lose_count;

        for _ in 0..win_lose_pair_count {
            if win_flag {
                // ○の処理
                (player_rating, opponent_player_rating) =
                    update_rating(&player_rating, &opponent_player_rating, &win_flag);
                // ×の処理
                (player_rating, opponent_player_rating) =
                    update_rating(&player_rating, &opponent_player_rating, &!win_flag);
            } else {
                // ×の処理
                (player_rating, opponent_player_rating) =
                    update_rating(&player_rating, &opponent_player_rating, &!win_flag);
                // ○の処理
                (player_rating, opponent_player_rating) =
                    update_rating(&player_rating, &opponent_player_rating, &win_flag);
            }
        }
        for _ in 0..win_lose_over_count {
            (player_rating, opponent_player_rating) =
                update_rating(&player_rating, &opponent_player_rating, &win_flag);
        }
        rating_map.insert(r.player_id, player_rating);
        rating_map.insert(r.opponent_player_id, opponent_player_rating);
    }
    rating_map
}

pub fn update_rating(a_rate: &f64, b_rate: &f64, a_win: &bool) -> (f64, f64) {
    let a_win_percentage = 1_f64 / (10_f64.powf((b_rate - a_rate) / 400_f64) + 1_f64);
    if *a_win {
        let b_win_percentage = 1_f64 - a_win_percentage;
        let a_win_increment = b_win_percentage * K;
        (a_rate + a_win_increment, b_rate - a_win_increment)
    } else {
        let b_win_increment = a_win_percentage * K;
        (a_rate - b_win_increment, b_rate + b_win_increment)
    }
}

pub fn get_win_percentage(a_rate: f64, b_rate: f64) -> (f64, f64) {
    let a_win_percentage = 1_f64 / (10_f64.powf((b_rate - a_rate) / 400_f64) + 1_f64);
    (a_win_percentage, 1_f64 - a_win_percentage)
}
