use crate::sfl::GameType::{EXTRA, GENERAL, MID, VAN};
use crate::sfl::SflStage::{JP2024DivisionF, JP2024DivisionS};
use crate::sfl::SflTeam::*;
use rand::prelude::*;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct SflRecord {
    pub sfl_match: SflMatch,
    pub set_number: u32,
    pub win_flag: bool,
    pub point: u32,
    pub game_type: GameType,
    // 有効なレコード（延長戦が行われなかったり大将戦が3本で終わると4本目、5本目はfalseになる）
    // TODO
    // is_disabled の方がわかりやすいのでできれば直す
    pub is_valid: bool,
    // 予想か実際かを区別する。予想ならtrue
    // これから予想する場合にもtrue。実績ならfalse
    pub is_prediction: bool,
}

impl SflRecord {
    // pub fn random_result(sfl_match: &SflMatch, rng: &mut StdRng) -> SflRecord {
    //     let required_battle = sfl_match.match_type.get_required_battle();
    //     let mut win_count = 0_u32;
    //     let mut lose_count = 0_u32;
    //     while win_count < required_battle && lose_count < required_battle {
    //         if rng.random() {
    //             win_count = win_count + 1;
    //         } else {
    //             lose_count = lose_count + 1;
    //         }
    //     }
    //     SflRecord {
    //         sfl_match: sfl_match.to_owned(),
    //         set: 0,
    //         win_count,
    //         lose_count,
    //         win_flag: win_count > lose_count,
    //         game_type: sfl_match.match_type.to_owned(),
    //         point: sfl_match.match_type.get_point(),
    //         is_valid: false,
    //         is_prediction: false,
    //     }
    // }
    // pub fn random_extra_result(sfl_match: &SflMatch, rng: &mut StdRng) -> SflRecord {
    //     let win_flag: bool = rng.random();
    //     SflRecord {
    //         sfl_match: SflMatch {
    //             section: sfl_match.section,
    //             branch: sfl_match.branch,
    //             sfl_stage: sfl_match.sfl_stage.to_owned(),
    //             team: sfl_match.team.to_owned(),
    //             opponent_team: sfl_match.opponent_team.to_owned(),
    //             is_home: sfl_match.is_home,
    //             match_type: EXTRA,
    //         },
    //         set: 0,
    //         win_count: if win_flag { 1 } else { 0 },
    //         lose_count: if win_flag { 0 } else { 1 },
    //         win_flag,
    //         point: EXTRA.get_point(),
    //         game_type: EXTRA,
    //         is_valid: false,
    //         is_prediction: false,
    //     }
    // }
    // pub fn update_record_by_simple_result(&self, win_flag: bool, is_valid: bool) -> SflRecord {
    //     let SflRecord { sfl_match, set, game_type, .. } = self.to_owned();
    //     SflRecord {
    //         sfl_match,
    //         set,
    //         win_count: if win_flag { 1 } else { 0 },
    //         lose_count: if win_flag { 0 } else { 1 },
    //         win_flag,
    //         point: 0,
    //         game_type,
    //         is_valid,
    //         is_prediction: false,
    //     }
    // }
}

#[derive(Clone, Debug)]
pub enum GameType {
    VAN,
    MID,
    GENERAL,
    EXTRA,
}

impl GameType {
    fn get_point(&self) -> u32 {
        match self {
            VAN => 10,
            MID => 10,
            GENERAL => 20,
            EXTRA => 5,
        }
    }
    fn is_leader(&self) -> bool {
        match self {
            VAN | MID => false,
            GENERAL | EXTRA => true,
        }
    }
    fn get_games_by_stage(sfl_stage: &SflStage) -> Vec<(u32, GameType)> {
        match sfl_stage {
            JP2024DivisionS | JP2024DivisionF => {
                vec![
                    (1, VAN),
                    (2, VAN),
                    (3, VAN),
                    (1, MID),
                    (2, MID),
                    (3, MID),
                    (1, GENERAL),
                    (2, GENERAL),
                    (3, GENERAL),
                    (4, GENERAL),
                    (5, GENERAL),
                    (1, EXTRA),
                ]
            }
            _ => vec![],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SflStage {
    JP2024DivisionS,
    JP2024DivisionF,
    JP2024Playoff,
    JP2024GrandFinal,
}

impl SflStage {
    pub fn get_teams(&self) -> Vec<SflTeam> {
        match self {
            JP2024DivisionS => vec![G8S, DFM, SOL, IBS, OJA, SNB],
            JP2024DivisionF => vec![CR, CAG, IXA, RC, VAR, FAV],
            _ => vec![],
        }
    }
    pub fn get_initial_records(&self) -> Vec<Vec<SflRecord>> {
        self.get_matches()
            .iter()
            .map(|sfl_match| self.match_to_records(sfl_match))
            .collect()
    }
    pub fn get_matches(&self) -> Vec<SflMatch> {
        match self {
            JP2024DivisionS => {
                vec![
                    (1, 1, DFM, OJA),
                    (1, 2, G8S, SNB),
                    (1, 3, SOL, IBS),
                    (2, 1, SNB, DFM),
                    (2, 2, IBS, OJA),
                    (2, 3, SOL, G8S),
                    (3, 1, OJA, SOL),
                    (3, 2, G8S, DFM),
                    (3, 3, SNB, IBS),
                    (4, 1, G8S, OJA),
                    (4, 2, SNB, SOL),
                    (4, 3, IBS, DFM),
                    (5, 1, IBS, G8S),
                    (5, 2, DFM, SOL),
                    (5, 3, OJA, SNB),
                    (6, 1, IBS, SOL),
                    (6, 2, SNB, G8S),
                    (6, 3, OJA, DFM),
                    (7, 1, G8S, SOL),
                    (7, 2, DFM, SNB),
                    (7, 3, OJA, IBS),
                    (8, 1, IBS, SNB),
                    (8, 2, SOL, OJA),
                    (8, 3, DFM, G8S),
                    (9, 1, OJA, G8S),
                    (9, 2, DFM, IBS),
                    (9, 3, SOL, SNB),
                    (10, 1, SNB, OJA),
                    (10, 2, SOL, DFM),
                    (10, 3, G8S, IBS),
                ]
            }
            JP2024DivisionF => {
                vec![
                    (1, 1, RC, IXA),
                    (1, 2, CAG, VAR),
                    (1, 3, CR, FAV),
                    (2, 1, VAR, RC),
                    (2, 2, FAV, IXA),
                    (2, 3, CR, CAG),
                    (3, 1, IXA, CR),
                    (3, 2, CAG, RC),
                    (3, 3, VAR, FAV),
                    (4, 1, CAG, IXA),
                    (4, 2, VAR, CR),
                    (4, 3, FAV, RC),
                    (5, 1, FAV, CAG),
                    (5, 2, RC, CR),
                    (5, 3, IXA, VAR),
                    (6, 1, FAV, CR),
                    (6, 2, VAR, CAG),
                    (6, 3, IXA, RC),
                    (7, 1, CAG, CR),
                    (7, 2, RC, VAR),
                    (7, 3, IXA, FAV),
                    (8, 1, FAV, VAR),
                    (8, 2, CR, IXA),
                    (8, 3, RC, CAG),
                    (9, 1, IXA, CAG),
                    (9, 2, RC, FAV),
                    (9, 3, CR, VAR),
                    (10, 1, VAR, IXA),
                    (10, 2, CR, RC),
                    (10, 3, CAG, FAV),
                ]
            }
            _ => {
                vec![]
            }
        }
        .iter()
        .map(|tup| {
            let (section, branch, team, opponent_team) = tup.to_owned();
            SflMatch {
                section,
                branch,
                sfl_stage: self.to_owned(),
                team,
                opponent_team,
                is_home: false,
            }
        })
        .collect()
    }
    fn match_to_records(&self, sfl_match: &SflMatch) -> Vec<SflRecord> {
        match sfl_match.sfl_stage {
            JP2024DivisionS | JP2024DivisionF => {
                GameType::get_games_by_stage(&sfl_match.sfl_stage)
                    .iter()
                    .map(|(set_number, game_type)| {
                        SflRecord {
                            sfl_match: sfl_match.to_owned(),
                            set_number: *set_number,
                            win_flag: false,
                            game_type: game_type.to_owned(),
                            // pointはcorrect_recordでセットする
                            point: 0,
                            is_valid: false,
                            is_prediction: true,
                        }
                    })
                    .collect()
            }
            _ => vec![],
        }
    }
    // パフォーマンスの問題もあるから前後の関連だけ見て修正する
    // is_valid = true フラグが立っているレコードについて見直して一部 is_valid = false に変える
    // ポイントを決着セットに書き加える
    // 決着していない場合はもちろんポイントを書かない
    // ランダム結果と実際結果が混じることがある
    pub fn correct_records(&self, records: &mut Vec<SflRecord>) {
        match self {
            JP2024DivisionS | JP2024DivisionF => {
                let van1 = records.get(0).unwrap().to_owned();
                let van2 = records.get(1).unwrap().to_owned();
                let van3 = records.get(2).unwrap().to_owned();
                let mid1 = records.get(3).unwrap().to_owned();
                let mid2 = records.get(4).unwrap().to_owned();
                let mid3 = records.get(5).unwrap().to_owned();
                let general1 = records.get(6).unwrap().to_owned();
                let general2 = records.get(7).unwrap().to_owned();
                let general3 = records.get(8).unwrap().to_owned();
                let general4 = records.get(9).unwrap().to_owned();
                let general5 = records.get(10).unwrap().to_owned();

                let mut team_point: u32 = 0;
                let mut opponent_team_point: u32 = 0;

                // 先鋒戦のポイント決定
                let van_point = VAN.get_point();
                if van1.is_valid && van2.is_valid {
                    if van1.win_flag == van2.win_flag {
                        let mut_van2 = records.get_mut(1).unwrap();
                        mut_van2.point = van_point;
                        let mut_van3 = records.get_mut(2).unwrap();
                        mut_van3.is_valid = false;
                        // ポイントのリセットはここではしない
                        // mut_van3.point = 0;
                        if van1.win_flag {
                            team_point += van_point;
                        } else {
                            opponent_team_point += van_point;
                        }
                    } else if van3.is_valid {
                        let mut_van3 = records.get_mut(2).unwrap();
                        mut_van3.is_valid = true;
                        mut_van3.point = van_point;
                        // ポイントのリセットはここではしない
                        let mut_van2 = records.get_mut(1).unwrap();
                        // mut_van2.point = 0;
                        if van3.win_flag {
                            team_point += van_point;
                        } else {
                            opponent_team_point += van_point;
                        }
                    }
                }

                // 中堅戦のポイント決定
                let mid_point = MID.get_point();
                if mid1.is_valid && mid2.is_valid {
                    if mid1.win_flag == mid2.win_flag {
                        let mut_mid2 = records.get_mut(4).unwrap();
                        mut_mid2.point = mid_point;
                        let mut_mid3 = records.get_mut(5).unwrap();
                        mut_mid3.is_valid = false;
                        // ポイントのリセットはここではしない
                        // mut_mid3.point = 0;
                        if mid1.win_flag {
                            team_point += mid_point;
                        } else {
                            opponent_team_point += mid_point;
                        }
                    } else if mid3.is_valid {
                        let mut_mid3 = records.get_mut(5).unwrap();
                        mut_mid3.is_valid = true;
                        mut_mid3.point = mid_point;
                        // ポイントのリセットはここではしない
                        let mut_mid2 = records.get_mut(4).unwrap();
                        // mut_mid2.point = 0;
                        if mid3.win_flag {
                            team_point += mid_point;
                        } else {
                            opponent_team_point += mid_point;
                        }
                    }
                }

                // 大将戦
                let general_point = GENERAL.get_point();
                if general1.is_valid && general2.is_valid && general3.is_valid {
                    if general1.win_flag == general2.win_flag
                        && general1.win_flag == general3.win_flag
                    {
                        let mut_general3 = records.get_mut(8).unwrap();
                        mut_general3.point = general_point;
                        let mut_general4 = records.get_mut(9).unwrap();
                        mut_general4.is_valid = false;
                        // ポイントのリセットはここではしない
                        // mut_general4.point = 0;
                        let mut_general5 = records.get_mut(10).unwrap();
                        mut_general5.is_valid = false;
                        // ポイントのリセットはここではしない
                        // mut_general5.point = 0;
                        if general1.win_flag {
                            team_point += general_point;
                        } else {
                            opponent_team_point += general_point;
                        }
                    } else if general4.is_valid {
                        let decide_flag = (general4.win_flag == general1.win_flag
                            && general4.win_flag == general2.win_flag
                            && general4.win_flag != general3.win_flag)
                            || (general4.win_flag == general1.win_flag
                                && general4.win_flag != general2.win_flag
                                && general4.win_flag == general3.win_flag)
                            || (general4.win_flag != general1.win_flag
                                && general4.win_flag == general2.win_flag
                                && general4.win_flag == general3.win_flag);
                        if decide_flag {
                            // ポイントのリセットはここではしない
                            let mut_general5 = records.get_mut(10).unwrap();
                            mut_general5.is_valid = false;
                            let mut_general4 = records.get_mut(9).unwrap();
                            mut_general4.is_valid = true;
                            mut_general4.point = general_point;
                            // mut_general5.point = 0;
                            if mut_general4.win_flag {
                                team_point += general_point;
                            } else {
                                opponent_team_point += general_point;
                            }
                        } else {
                            if general5.is_valid {
                                let mut_general5 = records.get_mut(10).unwrap();
                                mut_general5.is_valid = true;
                                mut_general5.point = general_point;
                                // ポイントのリセットはここではしない
                                // let mut_general4 = records.get_mut(9).unwrap();
                                // mut_general4.point = 0;
                                if mut_general5.win_flag {
                                    team_point += general_point;
                                } else {
                                    opponent_team_point += general_point;
                                }
                            }
                        }
                    }
                }

                // 延長戦
                let mut_extra1 = records.get_mut(11).unwrap();
                mut_extra1.is_valid = if team_point == van_point + mid_point
                    && opponent_team_point == general_point
                {
                    mut_extra1.point = EXTRA.get_point();
                    true
                } else {
                    false
                };
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct SflMatch {
    // 節
    pub section: u32,
    // 節内の順序
    pub branch: u32,
    pub sfl_stage: SflStage,
    pub team: SflTeam,
    pub opponent_team: SflTeam,
    is_home: bool,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum SflTeam {
    G8S,
    DFM,
    SOL,
    IBS,
    OJA,
    SNB,
    CR,
    CAG,
    IXA,
    RC,
    VAR,
    FAV,
}

impl fmt::Display for SflTeam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum SflRatingSetting {
    TeamOnly,
    HomeAway,
    GameType,
    HomeAwayGameType,
}

pub fn create_key_function_and_init_rating_map(
    setting: SflRatingSetting,
    teams: Vec<SflTeam>,
) -> (
    fn(&SflRecord) -> ((SflTeam, u8), (SflTeam, u8)),
    HashMap<(SflTeam, u8), f64>,
) {
    let default_rating = 1500_f64;
    let mut rating_map: HashMap<(SflTeam, u8), f64> = HashMap::new();
    match setting {
        SflRatingSetting::TeamOnly => {
            for team in teams.iter() {
                rating_map.insert((team.to_owned(), 000_u8), default_rating);
            }
            fn team_only_function(record: &SflRecord) -> ((SflTeam, u8), (SflTeam, u8)) {
                (
                    (record.sfl_match.team.to_owned(), 000_u8),
                    (record.sfl_match.opponent_team.to_owned(), 000_u8),
                )
            }
            (team_only_function, rating_map)
        }
        SflRatingSetting::HomeAway => {
            for team in teams.iter() {
                rating_map.insert((team.to_owned(), 120_u8), default_rating);
                rating_map.insert((team.to_owned(), 121_u8), default_rating);
            }
            fn home_away_function(record: &SflRecord) -> ((SflTeam, u8), (SflTeam, u8)) {
                if record.sfl_match.is_home {
                    (
                        (record.sfl_match.team.to_owned(), 121_u8),
                        (record.sfl_match.opponent_team.to_owned(), 120_u8),
                    )
                } else {
                    (
                        (record.sfl_match.team.to_owned(), 120_u8),
                        (record.sfl_match.opponent_team.to_owned(), 121_u8),
                    )
                }
            }
            (home_away_function, rating_map)
        }
        SflRatingSetting::GameType => {
            for team in teams.iter() {
                rating_map.insert((team.to_owned(), 102_u8), default_rating);
                rating_map.insert((team.to_owned(), 112_u8), default_rating);
            }
            fn game_type_function(record: &SflRecord) -> ((SflTeam, u8), (SflTeam, u8)) {
                if record.game_type.is_leader() {
                    (
                        (record.sfl_match.team.to_owned(), 112_u8),
                        (record.sfl_match.opponent_team.to_owned(), 112_u8),
                    )
                } else {
                    (
                        (record.sfl_match.team.to_owned(), 102_u8),
                        (record.sfl_match.opponent_team.to_owned(), 102_u8),
                    )
                }
            }
            (game_type_function, rating_map)
        }
        SflRatingSetting::HomeAwayGameType => {
            for team in teams.iter() {
                for n in [100_u8, 101_u8, 110_u8, 111_u8] {
                    rating_map.insert((team.to_owned(), n), default_rating);
                }
            }
            fn home_away_game_type_function(record: &SflRecord) -> ((SflTeam, u8), (SflTeam, u8)) {
                if record.sfl_match.is_home {
                    if record.game_type.is_leader() {
                        (
                            (record.sfl_match.team.to_owned(), 111_u8),
                            (record.sfl_match.opponent_team.to_owned(), 110_u8),
                        )
                    } else {
                        (
                            (record.sfl_match.team.to_owned(), 101_u8),
                            (record.sfl_match.opponent_team.to_owned(), 100_u8),
                        )
                    }
                } else {
                    if record.game_type.is_leader() {
                        (
                            (record.sfl_match.team.to_owned(), 110_u8),
                            (record.sfl_match.opponent_team.to_owned(), 111_u8),
                        )
                    } else {
                        (
                            (record.sfl_match.team.to_owned(), 100_u8),
                            (record.sfl_match.opponent_team.to_owned(), 101_u8),
                        )
                    }
                }
            }
            (home_away_game_type_function, rating_map)
        }
    }
}

fn init_rating_map(teams: Vec<SflTeam>) {}

pub fn get_place_sim_count(
    sfl_stage: SflStage,
) -> HashMap<SflTeam, (Vec<u32>, (u32, u32, i32, i32))> {
    let mut count: HashMap<SflTeam, (Vec<u32>, (u32, u32, i32, i32))> = HashMap::new();
    for team in sfl_stage.get_teams().into_iter() {
        count.insert(team, (vec![0; 6], (0, 0, 0, 0)));
    }
    count
}
