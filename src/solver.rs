/*

solver.rs: 終盤ソルバー用のファイル

*/

use crate::hash::*;
use crate::play::*;

pub const SOLVE_COUNT: i32 = 18;
pub const SOLVE_SORT: i32 = 5;
pub const HASH_DEPTH: i32 = 0;

pub struct NextAndFlippable {
    pub next: u64,
    pub player: u64,
    pub opponent: u64,
    pub f_num: i32,
}

pub fn solve(player: u64, opponent: u64, count: i32) -> u64 {
    //let blank = !(player|opponent);
    //println!("count:{}, blank:{}",count, blank.count_ones());
    /*
    読み切りをして、次の手を返す
    速さ優先探索: 相手が次打てる手が少ないものから探索
    */
    let mobilitys = mobility_ps(player, opponent);
    if mobilitys == 0 {
        // passのとき
        return 0;
    } else {
        let mut next_vec: Vec<NextAndFlippable> = Vec::new();
        let mut mask: u64 = 0x8000000000000000;
        while mask > 0 {
            if (mask & mobilitys) > 0 {
                let (next_player, next_opponent) = flip_board(player, opponent, mask);
                let op_flippable_num = mobility_ps(next_opponent, next_player).count_ones() as i32;
                next_vec.push(NextAndFlippable {
                    next: mask,
                    player: next_player,
                    opponent: next_opponent,
                    f_num: op_flippable_num,
                });
            }
            mask = mask >> 1;
        }

        next_vec.sort_unstable_by(|a, b| a.f_num.cmp(&b.f_num)); // f_numについて昇順にsort

        /**/
        if count > HASH_DEPTH {
            for next_and_f in &next_vec {
                let def = rec_solver_with_hashmap(
                    next_and_f.player,
                    next_and_f.opponent,
                    false,
                    count - 1,
                    1,
                );
                if def > 0 {
                    // 見つけたら終了
                    println!("solved! def:{}", def);
                    return next_and_f.next;
                }
            }
        } else {
            for next_and_f in &next_vec {
                let def = rec_solver(next_and_f.player, next_and_f.opponent, false, count - 1);
                if def > 0 {
                    // 見つけたら終了
                    println!("solved! def:{}", def);
                    return next_and_f.next;
                }
            }
            //
        }
        println!("not solved");
        return next_vec[0].next;
    }
}

fn rec_solver(player: u64, opponent: u64, is_player: bool, count: i32) -> i32 {
    /*
    探索をして、石数を返す
    */
    let mut mask: u64 = 0x8000000000000000;
    let mut next_vec: Vec<NextAndFlippable> = Vec::new();

    if is_player {
        // 自身の手に関しては、勝利するものが見つかれば終了
        let mobilitys = mobility_ps(player, opponent);

        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                return stone_def(player, opponent);
            } else {
                return rec_solver(player, opponent, false, count); // 空きマスはまだあるので、countは減らさずturnを変える
            }
        } else {
            if count == 1 {
                // 最終1手(自分)をその場で処理
                let (next_player, next_opponent) = flip_board(player, opponent, mobilitys); // mobilitysは0でないので1のみ
                return stone_def(next_player, next_opponent);
            } else if count == 2 {
                // 最終2手(自分->相手)をその場で処理
                // mobilitysは0ではないので、1か2
                let next1: u64 = 0x8000000000000000 >> mobilitys.leading_zeros();
                let (next1_player, next1_opponent) = flip_board(player, opponent, next1); // mobilitysは0でないので1のみ
                let def1 = rec_solver(next1_player, next1_opponent, false, count - 1);
                if def1 > 0 {
                    def1
                } else {
                    let next2: u64 = 0x0000000000000001 << mobilitys.trailing_zeros();
                    let (next2_player, next2_opponent) = flip_board(player, opponent, next2); // mobilitysは2つある
                    let def2 = rec_solver(next2_player, next2_opponent, false, count - 1);
                    def2
                }
            } else {
                let mut def = -64;
                while mask > 0 {
                    if (mask & mobilitys) > 0 {
                        let (next_player, next_opponent) = flip_board(player, opponent, mask);
                        let op_flippable_num =
                            mobility_ps(next_opponent, next_player).count_ones() as i32;
                        next_vec.push(NextAndFlippable {
                            next: mask,
                            player: next_player,
                            opponent: next_opponent,
                            f_num: op_flippable_num,
                        });
                    }
                    mask = mask >> 1;
                }
                if count > SOLVE_SORT {
                    // 最終6手ほどからは、ソートせずに全探索
                    next_vec.sort_unstable_by(|a, b| a.f_num.cmp(&b.f_num)); // f_numについて昇順に(速さ優先探索)
                }
                for next_and_f in next_vec {
                    def = rec_solver(next_and_f.player, next_and_f.opponent, false, count - 1);
                    if def > 0 {
                        // 見つけたら終了
                        return def;
                    }
                }
                return def; // 相手が
            }
        }
    } else {
        // 相手の手に関しては、すべての手に関して勝利する必要あり(引き分けもだめ)
        let mobilitys = mobility_ps(opponent, player);

        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                return stone_def(player, opponent);
            } else {
                return rec_solver(player, opponent, true, count); // 空きマスはまだあるので、countは減らさずturnを変える
            }
        } else {
            if count == 1 {
                // 最終1手(相手)をその場で処理
                let (next_opponent, next_player) = flip_board(opponent, player, mobilitys); // mobilitysは0でないので1のみ
                return stone_def(next_player, next_opponent);
            } else if count == 2 {
                // 最終2手(相手->自分)をその場で処理
                // mobilitysは0ではないので、1か2
                let next1: u64 = 0x8000000000000000 >> mobilitys.leading_zeros();
                let (next1_opponent, next1_player) = flip_board(opponent, player, next1); // mobilitysは0でないので1のみ
                let def1 = rec_solver(next1_player, next1_opponent, true, count - 1);
                if def1 <= 0 {
                    def1
                } else {
                    let next2: u64 = 0x0000000000000001 << mobilitys.trailing_zeros();
                    let (next2_opponent, next2_player) = flip_board(opponent, player, next2); // mobilitysは2つある
                    let def2 = rec_solver(next2_player, next2_opponent, true, count - 1);
                    def2
                }
            } else {
                let mut def = 0;
                while mask > 0 {
                    if (mask & mobilitys) > 0 {
                        let (next_opponent, next_player) = flip_board(opponent, player, mask);
                        let pl_flippable_num =
                            mobility_ps(next_player, next_opponent).count_ones() as i32;
                        next_vec.push(NextAndFlippable {
                            next: mask,
                            player: next_player,
                            opponent: next_opponent,
                            f_num: pl_flippable_num,
                        }); // f_numは使わない
                    }
                    mask = mask >> 1;
                }
                if count > SOLVE_SORT {
                    // 最終6手ほどからは、ソートせずに全探索
                    next_vec.sort_unstable_by(|a, b| a.f_num.cmp(&b.f_num)); // f_numについて昇順に(速さ優先探索)
                }
                for next_and_f in next_vec {
                    def = rec_solver(next_and_f.player, next_and_f.opponent, true, count - 1);
                    if def <= 0 {
                        // 見つけたら終了
                        return def; // マイナス値
                    }
                }
                return def; // プラス値
            }
        }
    }
}

fn rec_solver_with_hashmap(
    player: u64,
    opponent: u64,
    is_player: bool,
    count: i32,
    depth: i32,
) -> i32 {
    /*
    探索をして、石数を返す。
    hash_mapを利用して、すでに探索した部分をカット
    */

    let mut mask: u64 = 0x8000000000000000;
    let mut next_vec: Vec<NextAndFlippable> = Vec::new();

    if is_player {
        // 自身の手に関しては、勝利するものが見つかれば終了
        let hasher = make_hash(player, opponent);
        {
            let mut map_mut = Map_mut.read().unwrap();
            match map_mut.get(&hasher) {
                Some(result) => {
                    /*
                    println!(
                        "matched! result:{}, count:{}, depth:{}",
                        result, count, depth
                    );*/
                    return *result;
                }
                None => {}
            }
        }

        let mobilitys = mobility_ps(player, opponent);

        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                {
                    let result = stone_def(player, opponent);
                    hash_insert(hasher, result);
                    return result;
                }
            } else {
                {
                    let result = rec_solver_with_hashmap(player, opponent, false, count, depth); // 空きマスはまだあるので、countは減らさずturnを変える
                    hash_insert(hasher, result);
                    return result;
                }
            }
        } else {
            let mut def = -64;
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_player, next_opponent) = flip_board(player, opponent, mask);
                    let op_flippable_num =
                        mobility_ps(next_opponent, next_player).count_ones() as i32;
                    next_vec.push(NextAndFlippable {
                        next: mask,
                        player: next_player,
                        opponent: next_opponent,
                        f_num: op_flippable_num,
                    });
                }
                mask = mask >> 1;
            }
            // 最終6手より絶対に前、全探索せずにソート
            next_vec.sort_unstable_by(|a, b| a.f_num.cmp(&b.f_num)); // f_numについて昇順に(速さ優先探索)

            if depth < HASH_DEPTH {
                for next_and_f in next_vec {
                    def = rec_solver_with_hashmap(
                        next_and_f.player,
                        next_and_f.opponent,
                        false,
                        count - 1,
                        depth + 1,
                    );
                    if def > 0 {
                        // 見つけたら終了
                        hash_insert(hasher, def);
                        return def;
                    }
                }
            } else {
                for next_and_f in next_vec {
                    def = rec_solver(next_and_f.player, next_and_f.opponent, false, count - 1);
                    if def > 0 {
                        // 見つけたら終了
                        hash_insert(hasher, def);
                        return def;
                    }
                }
            }
            hash_insert(hasher, def);
            return def;
        }
    } else {
        let hasher = make_hash(opponent, player);
        {
            let mut map_mut = Map_mut.read().unwrap();
            match map_mut.get(&hasher) {
                Some(result) => return *result,
                None => {}
            }
        }

        // 相手の手に関しては、すべての手に関して勝利する必要あり(引き分けもだめ)
        let mobilitys = mobility_ps(opponent, player);

        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                {
                    let result = stone_def(player, opponent);
                    hash_insert(hasher, result);
                    return result;
                }
            } else {
                {
                    let result = rec_solver_with_hashmap(player, opponent, true, count, depth); // 空きマスはまだあるので、countは減らさずturnを変える
                    hash_insert(hasher, result);
                    return result;
                }
            }
        } else {
            let mut def = 0;
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_opponent, next_player) = flip_board(opponent, player, mask);
                    let pl_flippable_num =
                        mobility_ps(next_player, next_opponent).count_ones() as i32;
                    next_vec.push(NextAndFlippable {
                        next: mask,
                        player: next_player,
                        opponent: next_opponent,
                        f_num: pl_flippable_num,
                    }); // f_numは使わない
                }
                mask = mask >> 1;
            }
            // 最終6手より絶対に前、全探索せずにソート
            next_vec.sort_unstable_by(|a, b| a.f_num.cmp(&b.f_num)); // f_numについて昇順に(速さ優先探索)

            if depth < HASH_DEPTH {
                for next_and_f in next_vec {
                    def = rec_solver_with_hashmap(
                        next_and_f.player,
                        next_and_f.opponent,
                        true,
                        count - 1,
                        depth + 1,
                    );
                    if def <= 0 {
                        // 見つけたら終了
                        hash_insert(hasher, def);
                        return def; // マイナス値
                    }
                }
            } else {
                for next_and_f in next_vec {
                    def = rec_solver(next_and_f.player, next_and_f.opponent, true, count - 1);
                    if def <= 0 {
                        // 見つけたら終了
                        hash_insert(hasher, def);
                        return def; // マイナス値
                    }
                }
            }
            hash_insert(hasher, def);
            return def; // プラス値
        }
    }
}
