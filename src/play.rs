/*
play.rs: オセロ用の基本関数を集めたファイル

boardはu64*2のbit boardで表現する

*/
// 時間計測
use std::time::Instant;

use crate::color::*;
use crate::global::*;
//use crate::print::*;
use crate::solver::*;
use crate::think::*;
use rand::Rng;

pub enum Move {
    Mv { x: i32, y: i32 }, // x,yは0~7
    Pass,
    GiveUp,
}

/*
    座標(0~7が2つ), u64のbit列, Move構造体、プロトコル用の文字列の相互変換
*/
pub fn coordinate_to_bit(x: i32, y: i32) -> u64 {
    let mask: u64 = 0x8000000000000000;
    mask >> (x + y * 8)
}
pub fn bit_to_coordinate(mask: u64) -> (i32, i32) {
    let lead_zeros = mask.leading_zeros() as i32;
    (lead_zeros % 8, lead_zeros / 8)
}
pub fn move_to_bit(m: &Move) -> u64 {
    match m {
        Move::Pass => 0,
        Move::GiveUp => 0,
        Move::Mv { x: i, y: j } => coordinate_to_bit(*i, *j),
    }
}
pub fn bit_to_move(mask: u64) -> Move {
    if mask > 0 {
        let (x, y) = bit_to_coordinate(mask);
        Move::Mv { x: x, y: y }
    } else {
        Move::Pass
    }
}
pub fn move_to_string(m: &Move) -> String {
    match m {
        Move::Pass => "PASS".to_string(),
        Move::GiveUp => "GIVEUP".to_string(),
        Move::Mv { x: i, y: j } => {
            let ci = (i + ('A' as i32)) as u8 as char;
            let cj = (j + ('1' as i32)) as u8 as char;
            ci.to_string() + cj.to_string().as_str()
        }
    }
}

pub fn stone_def(player: u64, opponent: u64) -> i32 {
    // 石数の差を返す関数
    let player_num = player.count_ones() as i32;
    let opponent_num = opponent.count_ones() as i32;
    player_num - opponent_num
}

pub fn mobility_ps(player: u64, opponent: u64) -> u64 {
    //着手可能な場所をbitで返す関数(高速)
    let blank = !(player | opponent);
    let mo = opponent & 0x7e7e7e7e7e7e7e7e;

    //右
    let ps = player << 1;
    let mut mob = (mo + ps) & blank & !ps;

    // 左
    let mut t = player >> 1 & mo;
    t |= t >> 1 & mo;
    t |= t >> 1 & mo;
    t |= t >> 1 & mo;
    t |= t >> 1 & mo;
    t |= t >> 1 & mo;
    mob |= t >> 1 & blank;

    // 上下
    let mo = opponent & 0x00ffffffffffff00;

    // 下
    let mut t = player << 8 & mo;
    t |= t << 8 & mo;
    t |= t << 8 & mo;
    t |= t << 8 & mo;
    t |= t << 8 & mo;
    t |= t << 8 & mo;
    mob |= t << 8 & blank;

    // 上
    let mut t = player >> 8 & mo;
    t |= t >> 8 & mo;
    t |= t >> 8 & mo;
    t |= t >> 8 & mo;
    t |= t >> 8 & mo;
    t |= t >> 8 & mo;
    mob |= t >> 8 & blank;

    // 斜め

    let mo = opponent & 0x007e7e7e7e7e7e00;

    // 右下
    let mut t = player << 9 & mo;
    t |= t << 9 & mo;
    t |= t << 9 & mo;
    t |= t << 9 & mo;
    t |= t << 9 & mo;
    t |= t << 9 & mo;
    mob |= t << 9 & blank;

    // 左上
    let mut t = player >> 9 & mo;
    t |= t >> 9 & mo;
    t |= t >> 9 & mo;
    t |= t >> 9 & mo;
    t |= t >> 9 & mo;
    t |= t >> 9 & mo;
    mob |= t >> 9 & blank;

    // 左下
    let mut t = player << 7 & mo;
    t |= t << 7 & mo;
    t |= t << 7 & mo;
    t |= t << 7 & mo;
    t |= t << 7 & mo;
    t |= t << 7 & mo;
    mob |= t << 7 & blank;

    // 右上
    let mut t = player >> 7 & mo;
    t |= t >> 7 & mo;
    t |= t >> 7 & mo;
    t |= t >> 7 & mo;
    t |= t >> 7 & mo;
    t |= t >> 7 & mo;
    mob |= t >> 7 & blank;

    mob
}

pub fn is_flippable(player: u64, opponent: u64) -> bool {
    let mobility = mobility_ps(player, opponent);
    if mobility > 0 {
        true
    } else {
        false
    }
}

pub fn is_win(player: u64, opponent: u64) -> bool {
    /*
    勝利判定
    前提: 終了判定が済んだboardに対して行う
    */
    let player_num = player.count_ones();
    let opponent_num = opponent.count_ones();
    if player_num > opponent_num {
        true
    } else {
        false
    }
}

pub fn is_finished(player: u64, opponent: u64) -> bool {
    // 終了判定
    if is_flippable(player, opponent) || is_flippable(opponent, player) {
        false
    } else {
        true
    }
}
pub fn flip_board(player: u64, opponent: u64, next: u64) -> (u64, u64) {
    // flipしたあとのboardを返す
    if next > 0 {
        let rev = flippable_stones(player, opponent, next);
        return (player ^ (rev ^ next), opponent ^ rev);
    } else {
        return (player, opponent);
    }
}

pub fn flippable_stones(player: u64, opponent: u64, next: u64) -> u64 {
    // 高速に反転位置を求めるメソッド
    let mut omask = opponent;

    let pos = next.leading_zeros();

    // 上下
    // 上 (右端に1(右下除く))
    let mask: u64 = 0x0101010101010100 << (63 - pos);
    let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
    let tmp = if outflank != 0 { 1 } else { 0 };
    let mut flipped = (outflank - tmp) & mask;

    // 下 (右端に1(右下除く))
    let mask = 0x0080808080808080 >> pos;
    let outflank =
        (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
    flipped |= if outflank > 0 {
        ((!outflank + 1) << 1) & mask
    } else {
        0
    };
    let mut ret = flipped;

    omask &= 0x7e7e7e7e7e7e7e7e;

    // 左右
    let mask: u64 = 0x00000000000000fe << (63 - pos);
    let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
    let tmp = if outflank != 0 { 1 } else { 0 };
    let mut flipped = (outflank - tmp) & mask;

    let mask = 0x7f00000000000000 >> pos;
    let outflank =
        (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
    flipped |= if outflank > 0 {
        ((!outflank + 1) << 1) & mask
    } else {
        0
    };
    ret |= flipped;

    // 斜め
    let mask: u64 = 0x0002040810204080 << (63 - pos);
    let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
    let tmp = if outflank != 0 { 1 } else { 0 };
    let mut flipped = (outflank - tmp) & mask;

    let mask = 0x0102040810204000 >> pos;
    let outflank =
        (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
    flipped |= if outflank > 0 {
        ((!outflank + 1) << 1) & mask
    } else {
        0
    };
    ret |= flipped;

    let mask: u64 = 0x8040201008040200 << (63 - pos);
    let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
    let tmp = if outflank != 0 { 1 } else { 0 };
    let mut flipped = (outflank - tmp) & mask;

    let mask = 0x0040201008040201 >> pos;
    let outflank =
        (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
    flipped |= if outflank > 0 {
        ((!outflank + 1) << 1) & mask
    } else {
        0
    };
    ret |= flipped;

    return ret;
}

pub fn flippable_count(player: u64, opponent: u64, next: u64) -> i32 {
    //高速に反転数を求めるメソッド
    let mut omask = opponent;

    let pos = next.leading_zeros();

    // 上下
    // 上 (右端に1(右下除く))
    let mask: u64 = 0x0101010101010100 << (63 - pos);
    let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
    let tmp = if outflank != 0 { 1 } else { 0 };
    let mut flipped = (outflank - tmp) & mask;

    // 下 (右端に1(右下除く))
    let mask = 0x0080808080808080 >> pos;
    let outflank =
        (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
    flipped |= if outflank > 0 {
        ((!outflank + 1) << 1) & mask
    } else {
        0
    };
    let mut ret = flipped.count_ones();

    omask &= 0x7e7e7e7e7e7e7e7e;

    // 左右
    let mask: u64 = 0x00000000000000fe << (63 - pos);
    let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
    let tmp = if outflank != 0 { 1 } else { 0 };
    let mut flipped = (outflank - tmp) & mask;

    let mask = 0x7f00000000000000 >> pos;
    let outflank =
        (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
    flipped |= if outflank > 0 {
        ((!outflank + 1) << 1) & mask
    } else {
        0
    };
    ret += flipped.count_ones();

    // 斜め
    let mask: u64 = 0x0002040810204080 << (63 - pos);
    let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
    let tmp = if outflank != 0 { 1 } else { 0 };
    let mut flipped = (outflank - tmp) & mask;

    let mask = 0x0102040810204000 >> pos;
    let outflank =
        (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
    flipped |= if outflank > 0 {
        ((!outflank + 1) << 1) & mask
    } else {
        0
    };
    ret += flipped.count_ones();

    let mask: u64 = 0x8040201008040200 << (63 - pos);
    let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
    let tmp = if outflank != 0 { 1 } else { 0 };
    let mut flipped = (outflank - tmp) & mask;

    let mask = 0x0040201008040201 >> pos;
    let outflank =
        (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
    flipped |= if outflank > 0 {
        ((!outflank + 1) << 1) & mask
    } else {
        0
    };
    ret += flipped.count_ones();

    return ret as i32;
}

pub struct Board {
    // bit board の構造体
    pub black: u64,
    pub white: u64,
}

impl Board {
    pub fn new(x: u64, y: u64) -> Board {
        Board { black: x, white: y }
    }

    pub fn init() -> Board {
        /*
        Board型の初期オセロ板を返すメソッド
        whiteは0x0000001008000000で
        00000000
        00000000
        00000000
        00010000
        00001000
        00000000
        00000000
        00000000
        */
        Board {
            black: 0x0000000810000000,
            white: 0x0000001008000000,
        }
    }

    pub fn is_flippable(&self, color: i32) -> bool {
        let mobility = self.mobility_ps(color);
        if mobility > 0 {
            true
        } else {
            false
        }
    }

    pub fn is_finished(&self) -> bool {
        // 終了判定
        if self.is_flippable(BLACK) || self.is_flippable(WHITE) {
            false
        } else {
            true
        }
    }

    pub fn is_win(&self, my_color: i32) -> bool {
        /*
        勝利判定
        前提: 終了判定が済んだboardに対して行う
        */
        let (black, white) = (self.black, self.white);
        let black_num = black.count_ones();
        let white_num = white.count_ones();
        if black_num > white_num {
            if my_color == BLACK {
                true
            } else {
                false
            }
        } else if white_num > black_num {
            if my_color == WHITE {
                true
            } else {
                false
            }
        } else {
            // 引き分けのとき
            false
        }
    }

    pub fn flip_board_by_move(&self, color: i32, next: &Move) -> Board {
        /*
        input: board, 打ち手の色, 次の手
        output: flipしたあとのboard
        */
        let next: u64 = move_to_bit(next);
        if next > 0 {
            let rev = self.flippable_stones(color, next);
            if color == BLACK {
                return Board::new(self.black ^ (rev ^ next), self.white ^ rev);
            } else {
                return Board::new(self.black ^ rev, self.white ^ (rev ^ next));
            }
        } else {
            return Board::new(self.black, self.white);
        }
    }

    pub fn flip_board(&self, color: i32, next: u64) -> Board {
        /*
        input: board, 打ち手の色, 次の手
        output: flipしたあとのboard
        */
        if next > 0 {
            let rev = self.flippable_stones(color, next);
            if color == BLACK {
                return Board::new(self.black ^ (rev ^ next), self.white ^ rev);
            } else {
                return Board::new(self.black ^ rev, self.white ^ (rev ^ next));
            }
        } else {
            return Board::new(self.black, self.white);
        }
    }
    pub fn get_next(&self, color: i32, count: i32) -> Move {
        /*
        次の手を取得
        思考ルーチンによって変更する
        */
        let (player, opponent) = if color == BLACK {
            (self.black, self.white)
        } else {
            (self.white, self.black)
        };

        let mobilitys = mobility_ps(player, opponent);
        if count > ARGS.solve_start || ARGS.no_solve {
            let next: u64 = {
                //let args: Vec<String> = env::args().collect();
                match ARGS.name.as_str() {
                    "random" => get_by_random(mobilitys),
                    "first" => get_by_first(mobilitys), // 先頭のものを取得
                    "rusThello" => get_by_simple_alpha_beta(player, opponent, mobilitys), // simple_minimax
                    "rusThedom" => {
                        let mut rng = rand::thread_rng();
                        if rng.gen() {
                            get_by_first(mobilitys) // 先頭のものを取得
                        } else {
                            get_by_simple_alpha_beta(player, opponent, mobilitys) // simple_minimax
                        }
                    }

                    _ => get_by_simple_minimax(player, opponent, mobilitys),
                }
            };
            if next == 0 {
                Move::Pass
            } else {
                let (x, y) = bit_to_coordinate(next);
                Move::Mv { x: x, y: y }
            }
        } else {
            let start = Instant::now();
            let next: u64 = solve(player, opponent, count);
            let end = start.elapsed();
            if count == ARGS.solve_start {
                println!(
                    "count:{}  {}.{:03}秒経過しました。",
                    count,
                    end.as_secs(),
                    end.subsec_nanos() / 1_000_000
                );
            }
            if next == 0 {
                Move::Pass
            } else {
                let (x, y) = bit_to_coordinate(next);
                Move::Mv { x: x, y: y }
            }
        }
    }

    pub fn mobility_ps(&self, color: i32) -> u64 {
        /*
        ボードと白と黒どちらの手番かを受け取って、
        着手可能な場所をbitで返す関数(高速)
        */
        let (player, opponent) = if color == BLACK {
            (self.black, self.white)
        } else {
            (self.white, self.black)
        };
        let blank = !(player | opponent);
        let mo = opponent & 0x7e7e7e7e7e7e7e7e;

        //右
        let ps = player << 1;
        let mut mob = (mo + ps) & blank & !ps;

        // 左
        let mut t = player >> 1 & mo;
        t |= t >> 1 & mo;
        t |= t >> 1 & mo;
        t |= t >> 1 & mo;
        t |= t >> 1 & mo;
        t |= t >> 1 & mo;
        mob |= t >> 1 & blank;

        // 上下
        let mo = opponent & 0x00ffffffffffff00;

        // 下
        let mut t = player << 8 & mo;
        t |= t << 8 & mo;
        t |= t << 8 & mo;
        t |= t << 8 & mo;
        t |= t << 8 & mo;
        t |= t << 8 & mo;
        mob |= t << 8 & blank;

        // 上
        let mut t = player >> 8 & mo;
        t |= t >> 8 & mo;
        t |= t >> 8 & mo;
        t |= t >> 8 & mo;
        t |= t >> 8 & mo;
        t |= t >> 8 & mo;
        mob |= t >> 8 & blank;

        // 斜め

        let mo = opponent & 0x007e7e7e7e7e7e00;

        // 右下
        let mut t = player << 9 & mo;
        t |= t << 9 & mo;
        t |= t << 9 & mo;
        t |= t << 9 & mo;
        t |= t << 9 & mo;
        t |= t << 9 & mo;
        mob |= t << 9 & blank;

        // 左上
        let mut t = player >> 9 & mo;
        t |= t >> 9 & mo;
        t |= t >> 9 & mo;
        t |= t >> 9 & mo;
        t |= t >> 9 & mo;
        t |= t >> 9 & mo;
        mob |= t >> 9 & blank;

        // 左下
        let mut t = player << 7 & mo;
        t |= t << 7 & mo;
        t |= t << 7 & mo;
        t |= t << 7 & mo;
        t |= t << 7 & mo;
        t |= t << 7 & mo;
        mob |= t << 7 & blank;

        // 右上
        let mut t = player >> 7 & mo;
        t |= t >> 7 & mo;
        t |= t >> 7 & mo;
        t |= t >> 7 & mo;
        t |= t >> 7 & mo;
        t |= t >> 7 & mo;
        mob |= t >> 7 & blank;

        mob
    }

    pub fn flippable_stones(&self, color: i32, next: u64) -> u64 {
        /*
        ���速に���転位置を求めるメソッド
        */

        let (player, opponent) = if color == BLACK {
            (self.black, self.white)
        } else {
            (self.white, self.black)
        };

        let mut omask = opponent;

        let pos = next.leading_zeros();

        // 上下
        // 上 (右端に1(右下除く))
        let mask: u64 = 0x0101010101010100 << (63 - pos);
        let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
        let tmp = if outflank != 0 { 1 } else { 0 };
        let mut flipped = (outflank - tmp) & mask;

        // 下
        let mask = 0x0080808080808080 >> pos;
        let outflank =
            (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
        flipped |= if outflank > 0 {
            ((!outflank + 1) << 1) & mask
        } else {
            0
        };
        let mut ret = flipped;

        omask &= 0x7e7e7e7e7e7e7e7e;

        // 左右
        let mask: u64 = 0x00000000000000fe << (63 - pos);
        let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
        let tmp = if outflank != 0 { 1 } else { 0 };
        let mut flipped = (outflank - tmp) & mask;

        let mask = 0x7f00000000000000 >> pos;
        let outflank =
            (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
        flipped |= if outflank > 0 {
            ((!outflank + 1) << 1) & mask
        } else {
            0
        };
        ret |= flipped;

        // 斜め
        let mask: u64 = 0x0002040810204080 << (63 - pos);
        let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
        let tmp = if outflank != 0 { 1 } else { 0 };
        let mut flipped = (outflank - tmp) & mask;

        let mask = 0x0102040810204000 >> pos;
        let outflank =
            (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
        flipped |= if outflank > 0 {
            ((!outflank + 1) << 1) & mask
        } else {
            0
        };
        ret |= flipped;

        let mask: u64 = 0x8040201008040200 << (63 - pos);
        let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
        let tmp = if outflank != 0 { 1 } else { 0 };
        let mut flipped = (outflank - tmp) & mask;

        let mask = 0x0040201008040201 >> pos;
        let outflank =
            (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
        flipped |= if outflank > 0 {
            ((!outflank + 1) << 1) & mask
        } else {
            0
        };
        ret |= flipped;

        return ret;
    }

    pub fn flippable_count(&self, color: i32, next: u64) -> i32 {
        /*
        高速に反転数を求めるメソッド(solverの速さ優先探索で利用)
        */

        let (player, opponent) = if color == BLACK {
            (self.black, self.white)
        } else {
            (self.white, self.black)
        };

        let mut omask = opponent;

        let pos = next.leading_zeros();

        // 上下
        // 上 (右端に1(右下除く))
        let mask: u64 = 0x0101010101010100 << (63 - pos);
        let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
        let tmp = if outflank != 0 { 1 } else { 0 };
        let mut flipped = (outflank - tmp) & mask;

        // 下 (右端に1(右下除く))
        let mask = 0x0080808080808080 >> pos;
        let outflank =
            (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
        flipped |= if outflank > 0 {
            ((!outflank + 1) << 1) & mask
        } else {
            0
        };
        let mut ret = flipped.count_ones();

        omask &= 0x7e7e7e7e7e7e7e7e;

        // 左右
        let mask: u64 = 0x00000000000000fe << (63 - pos);
        let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
        let tmp = if outflank != 0 { 1 } else { 0 };
        let mut flipped = (outflank - tmp) & mask;

        let mask = 0x7f00000000000000 >> pos;
        let outflank =
            (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
        flipped |= if outflank > 0 {
            ((!outflank + 1) << 1) & mask
        } else {
            0
        };
        ret += flipped.count_ones();

        // 斜め
        let mask: u64 = 0x0002040810204080 << (63 - pos);
        let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
        let tmp = if outflank != 0 { 1 } else { 0 };
        let mut flipped = (outflank - tmp) & mask;

        let mask = 0x0102040810204000 >> pos;
        let outflank =
            (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
        flipped |= if outflank > 0 {
            ((!outflank + 1) << 1) & mask
        } else {
            0
        };
        ret += flipped.count_ones();

        let mask: u64 = 0x8040201008040200 << (63 - pos);
        let outflank = mask & ((omask | !mask).wrapping_add(1)) & player;
        let tmp = if outflank != 0 { 1 } else { 0 };
        let mut flipped = (outflank - tmp) & mask;

        let mask = 0x0040201008040201 >> pos;
        let outflank =
            (0x8000000000000000 as u64).wrapping_shr(((!omask) & mask).leading_zeros()) & player;
        flipped |= if outflank > 0 {
            ((!outflank + 1) << 1) & mask
        } else {
            0
        };
        ret += flipped.count_ones();

        return ret as i32;
    }

    /*
        pub fn old_mobility_ps(&self, color:i32)->u64{
        /*
        (旧 遅い)
        ボードと白と黒どちらの手番かを受け取って、
        着手可能な場所をbitで返す関数
        */
        let (player, opponent) = if color==BLACK {(self.black, self.white)}
        else {(self.white, self.black)};
        let blank = !(player|opponent);
        let horizontal = opponent & 0x7e7e7e7e7e7e7e7e;
        let vertical = opponent & 0x00FFFFFFFFFFFF00;
        let all_side = opponent & 0x007e7e7e7e7e7e00;
        let mut mobility = sub_mobility_l(player, horizontal, blank, 1); // 左
        mobility |= sub_mobility_l(player, vertical, blank, 8); // 上
        mobility |= sub_mobility_l(player, all_side, blank, 7); // 右上
        mobility |= sub_mobility_l(player, all_side, blank, 9); // 左上
        mobility |= sub_mobility_r(player, horizontal, blank, 1); // 右
        mobility |= sub_mobility_r(player, vertical, blank, 8); // 下
        mobility |= sub_mobility_r(player, all_side, blank, 7); // 左下
        mobility |= sub_mobility_r(player, all_side, blank, 9); // 右下
        mobility
    }


    pub fn old_flippable_stones(&self, color:i32, next:u64)->u64{
    /*
    input : ボード, 白と黒どちらの手番か, 着手箇所
    output : ひっくり返る場所
    */
    let (player, opponent) = if color==BLACK {(self.black, self.white)}
    else {(self.white, self.black)};

    let blank_h = !(player | opponent & 0x7e7e7e7e7e7e7e7e);
    let blank_v = !(player | opponent & 0x00ffffffffffff00);
    let blank_a = !(player | opponent & 0x007e7e7e7e7e7e00);
    let mut rev = sub_flippable_l(player, blank_h, next, 1); // 左
    rev |= sub_flippable_l(player, blank_v, next, 8); // 上
    rev |= sub_flippable_l(player, blank_a, next, 7); // 右上
    rev |= sub_flippable_l(player, blank_a, next, 9); // 左上
    rev |= sub_flippable_r(player, blank_h, next, 1); // 右
    rev |= sub_flippable_r(player, blank_v, next, 8); // 下
    rev |= sub_flippable_r(player, blank_a, next, 7); // 左下
    rev |= sub_flippable_r(player, blank_a, next, 9); // 右下
    rev
    }
    */
}

/*
fn sub_mobility_l(player:u64, masked:u64, blank:u64, num:u64)->u64{
// mobility_ps() 用
let mut tmp = masked & (player << num);
tmp |= masked & (tmp << num);
tmp |= masked & (tmp << num);
tmp |= masked & (tmp << num);
tmp |= masked & (tmp << num);
tmp |= masked & (tmp << num); // bitが立っているのは相手の碁が連続しているところ
let mobility = blank & (tmp << num);
mobility
}
fn sub_mobility_r(player:u64, masked:u64, blank:u64, num:u64)->u64{
// mobility_ps() 用
let mut tmp = masked & (player >> num);
tmp |= masked & (tmp >> num);
tmp |= masked & (tmp >> num);
tmp |= masked & (tmp >> num);
tmp |= masked & (tmp >> num);
tmp |= masked & (tmp >> num); // bitが立っているのは���手の碁が連続している���ころ
let mobility = blank & (tmp >> num);
mobility
}


fn sub_flippable_l(player:u64, masked:u64, next:u64, num:i32)->u64{
let mut rev = 0;
let mut tmp = !(player | masked) & (next<<num);
if tmp>0 {
for _i in 0..6{
tmp <<= num;
if (tmp & masked)>0 {break}     // となりが空白
else if (tmp & player)>0 {      // となりが自身の石
rev |= tmp >> num;
break
}else{                      // となりが相手の石
tmp |= tmp >> num;
}
}
}
return rev;
}
fn sub_flippable_r(player:u64, masked:u64, next:u64, num:i32)->u64{
let mut rev = 0;
let mut tmp = !(player | masked) & (next>>num);
if tmp>0 {
for _i in 0..6{
tmp >>= num;
if (tmp & masked)>0 {break}     // となりが空白
else if (tmp & player)>0 {      // となりが自身の石
rev |= tmp << num;
break
}else{                      // となりが相手の石
tmp |= tmp << num;
}
}
}
return rev;
}
*/
