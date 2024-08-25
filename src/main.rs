mod elo;
mod sfl;

use crate::elo::{get_win_percentage, update_rating};
use crate::sfl::{
    create_key_function_and_init_rating_map, SflRatingSetting, SflRecord, SflStage, SflTeam,
};
use rand::prelude::*;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let seed: [u8; 32] = [5; 32];
    let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
    //    let mut rng= StdRng::from_os_rng();
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            println!("{}", n.as_secs())
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    let sfl_stage = SflStage::JP2024DivisionS;
    let sfl_rate_setting = SflRatingSetting::HomeAwayGameType;
    let (rate_key_function, mut rating_map) =
        create_key_function_and_init_rating_map(sfl_rate_setting, sfl_stage.get_teams());
    // ステージに応じた初期状態のレコードを生成
    let mut initial_match_records: Vec<Vec<SflRecord>> = sfl_stage.get_initial_records();
    // すでに行われた結果を入力
    let played_game_results = vec![
        // match 1
        vec![
            true, true, false, false, true, true, false, true, false, true, false, true,
        ],
        // match 2
        vec![
            true, true, false, true, false, true, true, true, true, false,
        ],
        // match 3
        vec![
            true, false, true, true, false, true, false, false, true, true, false, true,
        ],
    ];
    // すでに行われた結果を初期状態のレコードに記入
    for tup in played_game_results.iter().enumerate() {
        let (index, match_results) = tup;
        let records = initial_match_records.get_mut(index);
        if records.is_none() {
            break;
        }
        let records = records.unwrap();
        for tup in match_results.iter().enumerate() {
            let (index, win_flag) = tup;
            let record = records.get_mut(index);
            if record.is_none() {
                break;
            }
            let record = record.unwrap();
            record.win_flag = *win_flag;
            record.is_valid = true;
            record.is_prediction = false;
        }
    }
    // すでに行われた分を補正
    for records in initial_match_records.iter_mut() {
        // 予想は対象外
        if records.get(0).unwrap().is_prediction {
            continue;
        }
        sfl_stage.correct_records(records);
        for record in records.iter() {
            if !record.is_valid {
                continue;
            }
            let (team_key, opponent_team_key) = rate_key_function(record);
            let team_rating = rating_map.get(&team_key).unwrap();
            let opponent_team_rating = rating_map.get(&opponent_team_key).unwrap();
            let (updated_rating, updated_opponent_rating) =
                update_rating(*team_rating, *opponent_team_rating, record.win_flag);
            rating_map.insert(team_key, updated_rating);
            rating_map.insert(opponent_team_key, updated_opponent_rating);
        }
    }

    // 順位の集計map
    let mut place_sim_count = sfl::get_place_sim_count(sfl_stage);

    // 1000回試行して小数点第一位まで表示
    for _ in 0..1000 {
        // すでに行われた結果を反映したレコードをコピー
        // ここの処理はあやしくて、新規コピーができていないっぽい
        let mut match_records = initial_match_records.to_owned();

        // ランダムに結果をセット（レーティング処理を追加するならここ）
        for records in match_records.iter_mut() {
            // すでに行われた結果では is_prediction: false となっているので continue
            if !records.get(0).unwrap().is_prediction {
                continue;
            }
            for record in records.iter_mut() {
                let (ref team_key, ref opponent_team_key) = rate_key_function(record);
                let team_rating = rating_map.get(team_key).unwrap();
                let opponent_team_rating = rating_map.get(opponent_team_key).unwrap();
                let (team_win_percentage, _) =
                    get_win_percentage(*team_rating, *opponent_team_rating);
                               record.win_flag = rng.random();
                // record.win_flag = rng.gen_bool(team_win_percentage);
                record.is_valid = true;
            }
            // 予想分の補正処理
            sfl_stage.correct_records(records);
        }
        // println!("{:?}", match_records);

        //let rating_map = get_player_rating(elo_setting, record_list);
        //println!("{:?}", rating_map);

        // 一次元vectorに変更
        let mut sfl_records: Vec<SflRecord> = match_records.into_iter().flat_map(|x| x).collect();

        // SflRecordから集計ロジックを開始
        let mut point_map: HashMap<SflTeam, (u32, i32)> = HashMap::new();
        // チームの分だけ初期化
        for team in sfl_stage.get_teams().into_iter() {
            point_map.insert(team, (0, 0));
        }

        // レコードごとにポイント集計開始
        for record in sfl_records.iter() {
            // 無効ならスキップ
            if !record.is_valid {
                continue;
            }
            let team = record.sfl_match.team.to_owned();
            let opponent_team = record.sfl_match.opponent_team.to_owned();
            let (mut team_point, mut team_battle) = point_map.get(&team).unwrap();
            let (mut opponent_team_point, mut opponent_team_battle) =
                point_map.get(&opponent_team).unwrap();
            if record.win_flag {
                team_point += record.point;
                team_battle += 1;
                opponent_team_battle -= 1;
            } else {
                opponent_team_point += record.point;
                team_battle -= 1;
                opponent_team_battle += 1;
            }
            point_map.insert(team, (team_point, team_battle));
            point_map.insert(opponent_team, (opponent_team_point, opponent_team_battle));
        }
        // println!("{:?}", point_map);
        let mut sortable: Vec<(SflTeam, u32, i32)> = vec![];
        for (team, (point, battle)) in point_map.iter() {
            sortable.push(((*team).to_owned(), *point, *battle));
        }
        sortable.sort_by(|(a_team, a_point, a_battle), (b_team, b_point, b_battle)| {
            b_point
                .cmp(&a_point)
                .then(b_battle.cmp(&a_battle))
                .then((b_team.to_owned() as i32).cmp(&(a_team.to_owned() as i32)))
        });
        for n in 0..6 {
            let (team, _, _) = sortable.get(n).unwrap();
            let count = place_sim_count.get_mut(team).unwrap();
            let new_val = count.get(n).unwrap() + 1;
            count[n] = new_val;
        }
    }
    for team in sfl_stage.get_teams().iter() {
        let places_text = place_sim_count
            .get(team)
            .unwrap()
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join("\t");
        println!("{:?}\t{}", team, places_text);
    }
    println!();
    println!("TEAM\tMMAW\tMMHM\tLDAW\tLDHM");
    for team in sfl_stage.get_teams().iter() {
        let rating_text = [100, 101, 110, 111]
            .into_iter()
            .map(|n| {
                rating_map
                    .get(&(team.to_owned(), n))
                    .unwrap()
                    .round()
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join("\t");
        println!("{:?}\t{}", team, rating_text);
    }
}
