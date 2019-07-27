use crate::color::*;
use crate::eval::*;
use crate::global::*;
use crate::play::*;
use rand::Rng;
use std::collections::BTreeMap;

pub const FMAX: f32 = 100000000.0;

pub fn negascout(
    next: u64,
    pre: u64,
    is_player: bool,
    color: i32,
    mobilitys: u64,
    depth: i32,
    count: i32,
    alpha_ori: f32,
    beta: f32,
) -> (f32, u64) {
    let mut alpha = alpha_ori;

    if depth <= 0 {
        let (me, op) = if is_player { (next, pre) } else { (pre, next) };
        let mut v = board_eval(me, op, count, is_player, color);
        if !is_player {
            v = -v
        };
        //println!("board_eval:{}, is_player:{}", v, is_player);

        return (v, 0);
    }

    if mobilitys == 0 {
        // pass or gamesetのとき
        if is_finished(next, pre) {
            if is_win(next, pre) {
                return (FMAX, 0);
            } else {
                return (-FMAX, 0);
            }
        } else {
            let next_mobilitys = mobility_ps(pre, next);
            let (v, tmp) = negascout(
                pre,
                next,
                !is_player,
                color,
                next_mobilitys,
                depth - 1,
                count,
                -beta,
                -alpha,
            );
            return (-v, 0);
        }
    }

    let mut next_vec: Vec<u64> = Vec::new();
    let mut mask: u64 = 0x8000000000000000;
    while mask > 0 {
        if (mask & mobilitys) > 0 {
            next_vec.push(mask);
        }
        mask = mask >> 1;
    }
    //println!("vec len:{}", next_vec.len());
    /*
    必要ならnext_vecのソート(moveOrdering)
    */
    /**/
    if depth > 3 {
        if is_player {
            next_vec = move_ordering(next, pre, color, next_vec);
        } else {
            next_vec = move_ordering(next, pre, opposite_color(color), next_vec);
        };
    }

    let mut max: f32 = -FMAX;
    let mut result_pos: u64 = 0;

    // betterな手から探索
    let better = next_vec[0];
    let (next_after, pre_after) = flip_board(next, pre, better);
    let next_mobilitys = mobility_ps(pre_after, next_after);
    let (mut v, _) = negascout(
        pre_after,
        next_after,
        !is_player,
        color,
        next_mobilitys,
        depth - 1,
        count - 1,
        -beta,
        -alpha,
    );
    v = -v;
    //カット
    if beta <= v {
        return (v, better);
    };
    if alpha < v {
        alpha = v;
        max = v;
    }
    result_pos = better;

    let mut flag = 0;
    for pos in next_vec {
        if flag == 0 {
            flag += 1;
            continue;
        };
        /*
        value <= v <= alpha : 選択されないので、ちゃんとvalueを求めない
        alpha + 0.1 <= v <= value : bata <= v ならカット,
            そうでないなら、 beta<= valueか, value <= betaかわからない。
        */
        let (next_after, pre_after) = flip_board(next, pre, pos);
        let next_mobilitys = mobility_ps(pre_after, next_after);
        // null window
        let (v_tmp, _) = negascout(
            pre_after,
            next_after,
            !is_player,
            color,
            next_mobilitys,
            depth - 1,
            count - 1,
            -alpha - 0.1,
            -alpha,
        );
        v = -v_tmp;

        //カット
        if beta <= v {
            return (v, pos);
        };

        if alpha < v {
            alpha = v;
            let (v_tmp, _) = negascout(
                pre_after,
                next_after,
                !is_player,
                color,
                next_mobilitys,
                depth - 1,
                count - 1,
                -beta,
                -alpha,
            );
            v = -v_tmp;

            // カット
            if beta <= v {
                return (v, pos);
            }
            if alpha < v {
                alpha = v
            }
        }
        if max < v {
            max = v;
            result_pos = pos
        };
    }
    return (max, result_pos);
}

fn move_ordering(player: u64, opponent: u64, color: i32, next_vec: Vec<u64>) -> Vec<u64> {
    let mut max = -FMAX;
    let mut max_count = 0;
    let mut count = 0;
    let mut v = Vec::new();
    for next in next_vec {
        v.push(next);
        let (next_player, next_opponent) = flip_board(player, opponent, next);
        let next_mobilitys = mobility_ps(next_opponent, next_player);
        let val = alpha_beta(
            next_player,
            next_opponent,
            false,
            color,
            next_mobilitys,
            2,
            -FMAX,
            FMAX,
        );
        if max < val {
            max = val;
            max_count = count;
        }
        count += 1;
    }

    v.swap(0, max_count);
    v
}

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

pub fn get_by_model(player: u64, opponent: u64, mobilitys: u64, count: i32, color: i32) -> u64 {
    // 評価関数に基づいた着手可能場所を取得(単純な思考ルーチン)
    if mobilitys == 0 {
        return 0;
    } else {
        let mut next: u64 = 1;
        let mut mask: u64 = 0x8000000000000000;
        let mut best = std::f32::MIN;
        while mask > 0 {
            if (mask & mobilitys) > 0 {
                let (next_player, next_opponent) = flip_board(player, opponent, mask);
                let next_mobilitys = mobility_ps(next_opponent, next_player);

                let val: f32 = model_alpha_beta(
                    next_player,
                    next_opponent,
                    false,
                    color,
                    next_mobilitys,
                    ARGS.think_depth,
                    std::f32::MIN,
                    std::f32::MAX,
                    count - 1,
                );
                if best < val {
                    best = val;
                    next = mask;
                }
            }
            mask = mask >> 1;
        }
        //println!("count:{}, best:{}", count, best);
        return next;
    }
}
fn model_alpha_beta(
    player: u64,
    opponent: u64,
    is_player: bool,
    color: i32,
    mobilitys: u64,
    depth: i32,
    alpha: f32,
    beta: f32,
    count: i32,
) -> f32 {
    /* 葉の場合、評価値を返す */
    if depth <= 0 {
        return board_eval(player, opponent, count, is_player, color);
    }
    let mut mask: u64 = 0x8000000000000000;

    if is_player == true {
        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                if is_win(player, opponent) {
                    return std::f32::MAX;
                } else {
                    return std::f32::MIN;
                }
            } else {
                let next_mobilitys = mobility_ps(opponent, player);
                return model_alpha_beta(
                    player,
                    opponent,
                    false,
                    color,
                    next_mobilitys,
                    depth - 1,
                    alpha,
                    beta,
                    count - 1,
                );
            }
        } else {
            let mut alp: f32 = alpha;
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_player, next_opponent) = flip_board(player, opponent, mask);
                    let next_mobilitys = mobility_ps(next_opponent, next_player);
                    let tmp = model_alpha_beta(
                        next_player,
                        next_opponent,
                        false,
                        color,
                        next_mobilitys,
                        depth - 1,
                        alp,
                        beta,
                        count - 1,
                    );
                    if tmp > alp {
                        alp = tmp
                    }
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
                    return std::f32::MAX;
                } else {
                    return std::f32::MIN;
                }
            } else {
                let next_mobilitys = mobility_ps(player, opponent);
                return model_alpha_beta(
                    player,
                    opponent,
                    true,
                    color,
                    next_mobilitys,
                    depth - 1,
                    alpha,
                    beta,
                    count - 1,
                );
            }
        } else {
            let mut be: f32 = beta;
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_opponent, next_player) = flip_board(opponent, player, mask);
                    let next_mobilitys = mobility_ps(next_player, next_opponent);
                    let tmp = model_alpha_beta(
                        next_player,
                        next_opponent,
                        true,
                        color,
                        next_mobilitys,
                        depth - 1,
                        alpha,
                        be,
                        count - 1,
                    );
                    if tmp < be {
                        be = tmp
                    };
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

pub fn get_by_simple_alpha_beta(
    player: u64,
    opponent: u64,
    mobilitys: u64,
    color: i32,
    depth: i32,
) -> u64 {
    if mobilitys == 0 {
        return 0;
    } else {
        let mut next: u64 = 1;
        let mut mask: u64 = 0x8000000000000000;
        let mut best = -FMAX;
        //println!("vec len:{}", mobilitys.count_ones());
        while mask > 0 {
            if (mask & mobilitys) > 0 {
                let (next_player, next_opponent) = flip_board(player, opponent, mask);
                let next_mobilitys = mobility_ps(next_opponent, next_player);

                let val = alpha_beta(
                    next_player,
                    next_opponent,
                    false,
                    color,
                    next_mobilitys,
                    depth - 1,
                    -FMAX,
                    FMAX,
                );
                //println!("board_eval:{}, mask:{}", val, mask);
                if best < val {
                    best = val;
                    next = mask;
                }
            }
            mask = mask >> 1;
        }
        //println!("val:{}", best);
        return next;
    }
}
fn alpha_beta(
    player: u64,
    opponent: u64,
    is_player: bool,
    color: i32,
    mobilitys: u64,
    depth: i32,
    alpha: f32,
    beta: f32,
) -> f32 {
    /* 葉の場合、評価値を返す */
    if depth <= 0 {
        return board_eval(player, opponent, 0, is_player, color);
    }
    let mut mask: u64 = 0x8000000000000000;

    if is_player == true {
        if mobilitys == 0 {
            // passのとき
            if is_finished(player, opponent) {
                if is_win(player, opponent) {
                    return FMAX;
                } else {
                    return -FMAX;
                }
            } else {
                let next_mobilitys = mobility_ps(opponent, player);
                return alpha_beta(
                    player,
                    opponent,
                    false,
                    color,
                    next_mobilitys,
                    depth - 1,
                    alpha,
                    beta,
                );
            }
        } else {
            let mut alp: f32 = alpha;
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_player, next_opponent) = flip_board(player, opponent, mask);
                    let next_mobilitys = mobility_ps(next_opponent, next_player);
                    let tmp = alpha_beta(
                        next_player,
                        next_opponent,
                        false,
                        color,
                        next_mobilitys,
                        depth - 1,
                        alp,
                        beta,
                    );
                    if alp < tmp {
                        alp = tmp
                    };
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
                    return FMAX;
                } else {
                    return -FMAX;
                }
            } else {
                let next_mobilitys = mobility_ps(player, opponent);
                return alpha_beta(
                    player,
                    opponent,
                    true,
                    color,
                    next_mobilitys,
                    depth - 1,
                    alpha,
                    beta,
                );
            }
        } else {
            let mut be: f32 = beta;
            while mask > 0 {
                if (mask & mobilitys) > 0 {
                    let (next_opponent, next_player) = flip_board(opponent, player, mask);
                    let next_mobilitys = mobility_ps(next_player, next_opponent);
                    let tmp = alpha_beta(
                        next_player,
                        next_opponent,
                        true,
                        color,
                        next_mobilitys,
                        depth - 1,
                        alpha,
                        be,
                    );
                    if be > tmp {
                        be = tmp;
                    };
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
/*
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
    /* 葉の場合、評価値を������ */
    if depth <= 0 {
        return board_eval(player, opponent, 0, is_player);
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
*/
