/*

    think.rs: 思考ルーチン用ファイル
*/
use rand::Rng;

use crate::eval::*;
use crate::global::*;
use crate::play::*;

pub fn get_by_random(mobilitys: u64) -> u64 {
    // 最初の着手可能場所を取得(最も単純な思考ルーチン)
    let mut mask: u64 = 0x8000000000000000;
    let mobility_count = mobilitys.count_ones();
    if mobility_count == 0 {
        return 0;
    } else {
        let mut rng = rand::thread_rng();
        let random_num = rng.gen_range(0, mobility_count);
        let mut count = 0;
        for _i in 0..64 {
            if (mask & mobilitys) == mask {
                if count == random_num {
                    return mask;
                }
                count = count + 1;
            }
            mask = mask >> 1;
        }
    }
    return mask;
}

pub fn get_by_first(mobilitys: u64) -> u64 {
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

pub fn get_by_simple_alpha_beta(player: u64, opponent: u64, mobilitys: u64) -> u64 {
    // 評価関数に基づいた着手可能場所を取得(単純な思考ルーチン)
    if mobilitys == 0 {
        return 0;
    } else {
        let mut next: u64 = 1;
        let mut mask: u64 = 0x8000000000000000;
        let mut best = MIN;
        while mask > 0 {
            if (mask & mobilitys) > 0 {
                let (next_player, next_opponent) = flip_board(player, opponent, mask);
                let next_mobilitys = mobility_ps(next_opponent, next_player);

                let val = alpha_beta(
                    next_player,
                    next_opponent,
                    false,
                    next_mobilitys,
                    ARGS.think_depth,
                    MIN,
                    MAX,
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

// 未完
fn alpha_beta(
    player: u64,
    opponent: u64,
    is_player: bool,
    mobilitys: u64,
    depth: i32,
    alpha: i32,
    beta: i32,
) -> i32 {
    /* 葉の場合、評価値を返す */
    if depth <= 0 {
        return board_eval(player, opponent);
    }
    let mut mask: u64 = 0x8000000000000000;

    if is_player == true {
        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                if is_win(player, opponent) {
                    return MAX;
                } else {
                    return MIN;
                }
            } else {
                let next_mobilitys = mobility_ps(opponent, player);
                return alpha_beta(
                    player,
                    opponent,
                    false,
                    next_mobilitys,
                    depth - 1,
                    alpha,
                    beta,
                );
            }
        } else {
            let mut alp: i32 = alpha;
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_player, next_opponent) = flip_board(player, opponent, mask);
                    let next_mobilitys = mobility_ps(next_opponent, next_player);
                    alp = std::cmp::max(
                        alp,
                        alpha_beta(
                            next_player,
                            next_opponent,
                            false,
                            next_mobilitys,
                            depth - 1,
                            alp,
                            beta,
                        ),
                    );
                    if beta <= alp {
                        break;
                    }
                }
                mask = mask >> 1;
            }
            return alp;
        }
    } else {
        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                if is_win(player, opponent) {
                    return MAX;
                } else {
                    return MIN;
                }
            } else {
                let next_mobilitys = mobility_ps(player, opponent);
                return alpha_beta(
                    player,
                    opponent,
                    true,
                    next_mobilitys,
                    depth - 1,
                    alpha,
                    beta,
                );
            }
        } else {
            let mut be: i32 = beta;
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_opponent, next_player) = flip_board(opponent, player, mask);
                    let next_mobilitys = mobility_ps(next_player, next_opponent);
                    be = std::cmp::min(
                        be,
                        alpha_beta(
                            next_player,
                            next_opponent,
                            true,
                            next_mobilitys,
                            depth - 1,
                            alpha,
                            be,
                        ),
                    );
                    if be <= alpha {
                        break;
                    }
                }
                mask = mask >> 1;
            }
            return be;
        }
    }
}

pub fn get_by_simple_minimax(player: u64, opponent: u64, mobilitys: u64) -> u64 {
    // 評価関数に基づいた着手可能場所を取得(単純な思考ルーチン)
    if mobilitys == 0 {
        return 0;
    } else {
        let mut next: u64 = 1;
        let mut mask: u64 = 0x8000000000000000;
        let mut best = MIN;
        while mask > 0 {
            if (mask & mobilitys) > 0 {
                let (next_player, next_opponent) = flip_board(player, opponent, mask);
                let next_mobilitys = mobility_ps(next_opponent, next_player);

                let val = minimax(
                    next_player,
                    next_opponent,
                    false,
                    next_mobilitys,
                    ARGS.think_depth,
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
    if depth <= 0 {
        return board_eval(player, opponent);
    }
    let mut mask: u64 = 0x8000000000000000;

    let mut best = MIN;

    if is_player == true {
        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                if is_win(player, opponent) {
                    return MAX;
                } else {
                    return MIN;
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
                    if best < val {
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
                    return MAX;
                } else {
                    return MIN;
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
                    if best < -val {
                        best = -val;
                    }
                }
                mask = mask >> 1;
            }

            return -best;
        }
    }
}
