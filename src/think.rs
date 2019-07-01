/*

    think.rs: 思考ルーチン用ファイル

*/
pub const INFINITY: i32 = 1 << 30;
pub const THINE_DEPTH: i32 = 2;

use crate::eval::*;
use crate::play::*;
//use crate::color::*;

pub fn get_first_mobilitys(mobilitys: u64) -> u64 {
    // 最初の着手可能場所を取得(最も単純な思考ルーチン)
    let mut mask: u64 = 0x8000000000000000;
    if mobilitys == 0 {
        return 0;
    } else {
        for _i in 0..64 {
            if (mask & mobilitys) == mask {
                return mask;
            }
            mask = mask >> 1;
        }
    }
    return mask;
}

pub fn get_by_simple_minimax(player: u64, opponent: u64, mobilitys: u64) -> u64 {
    // 評価関数に基づいた着手可能場所を取得(単純な思考ルーチン)
    if mobilitys == 0 {
        return 0;
    } else {
        let mut next: u64 = 1;
        let mut mask: u64 = 0x8000000000000000;
        let mut best = -INFINITY;
        while mask > 0 {
            if (mask & mobilitys) > 0 {
                let (next_player, next_opponent) = flip_board(player, opponent, mask);
                let next_mobilitys = mobility_ps(next_opponent, next_player);

                let val = minimax(
                    next_player,
                    next_opponent,
                    false,
                    next_mobilitys,
                    THINE_DEPTH,
                );
                if best < val {
                    best = val;
                    next = mask;
                }
            }
            mask = mask >> 1;
        }
        return next;
    }
}

fn minimax(player: u64, opponent: u64, is_player: bool, mobilitys: u64, depth: i32) -> i32 {
    /* 葉の場合、評価値を返す */
    if (depth <= 0) {
        return simple_eval(player, opponent);
    }
    let mut mask: u64 = 0x8000000000000000;

    let mut best = -INFINITY;

    if is_player == true {
        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                if is_win(player, opponent) {
                    return INFINITY;
                } else {
                    return -INFINITY;
                }
            } else {
                let next_mobilitys = mobility_ps(opponent, player);
                return minimax(player, opponent, false, next_mobilitys, depth - 1);
            }
        } else {
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_player, next_opponent) = flip_board(player, opponent, mask);
                    let next_mobilitys = mobility_ps(next_opponent, next_player);
                    let val = minimax(next_player, next_opponent, false, next_mobilitys, depth - 1);
                    if (best < val) {
                        best = val;
                    }
                }
                mask = mask >> 1;
            }
            return best;
        }
    } else {
        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                if is_win(player, opponent) {
                    return INFINITY;
                } else {
                    return -INFINITY;
                }
            } else {
                let next_mobilitys = mobility_ps(player, opponent);
                return minimax(player, opponent, true, next_mobilitys, depth - 1);
            }
        } else {
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_opponent, next_player) = flip_board(opponent, player, mask);
                    let next_mobilitys = mobility_ps(next_player, next_opponent);
                    let val = minimax(next_player, next_opponent, true, next_mobilitys, depth - 1);
                    if (best < -val) {
                        best = -val;
                    }
                }
                mask = mask >> 1;
            }

            return -best;
        }
    }
}
