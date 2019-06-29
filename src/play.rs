/*
    play.rs: オセロ用の基本関数を集めたファイル

    boardはu64*2のbit boardで表現する

*/

use crate::think::*;
use crate::color::*;
use crate::solver::*;


pub enum Move {
  Mv {x:u32, y:u32}, // x,yは0~7
  Pass,
  GiveUp
}
/*
pub enum Opmove{
    PMove(Move),
    OMove(Move)
}
*/
pub fn coordinate_to_bit(x:u32,y:u32)->u64{
    /*
        0~7のx,yを受け取り、座標をbitに変換
    */
    let mask:u64 = 0x8000000000000000; //左端だけが 1
    mask >> (x+y*8)

}
pub fn bit_to_coordinate(mask:u64)->(u32, u32){
    /*
        input : u64で表した座標
        output : 0~7の座標x,y
    */
    let lead_zeros = mask.leading_zeros();
    (lead_zeros % 8, lead_zeros / 8)
}


pub fn move_to_string(m: &Move) -> String {
    /*
        input: Move構造体
        output: プロトコル用の座標に変換
    */
  match m {
    Move::Pass => {
      "PASS".to_string()
    }
    Move::GiveUp => {
      "GIVEUP".to_string()
    }
    Move::Mv{x:i, y:j} => {
      let ci = (i + ('A' as u32)) as u8 as char;
      let cj = (j + ('1' as u32)) as u8 as char;
      ci.to_string() + cj.to_string().as_str()
    }
  }
}

pub fn move_to_bit(m: &Move) -> u64 {
    /*
        input: Move構造体
        output: bitで表した座標
    */
  match m {
    Move::Pass => {
      0
    }
    Move::GiveUp => {
      0
    }
    Move::Mv{x:i, y:j} => {
      coordinate_to_bit(*i,*j)
    }
  }
}
pub fn bit_to_move(mask:u64) -> Move {
    /*
        input: bitで表した座標
        output: Move構造体
    */
    if mask>0{
        let (x,y) = bit_to_coordinate(mask);
        Move::Mv{x:x,y:y}
    }else{
        Move::Pass
    }
}




pub struct Board {
    // bit board の構造体
    pub black: u64, pub white: u64
}
impl Board{
    pub fn new(x:u64,y:u64)->Board{
        Board{black:x, white:y}
    }

    pub fn init()->Board{
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
        Board{black:0x0000000810000000, white:0x0000001008000000}
    }

    pub fn is_flippable(&self, color:u32)->bool{
        let legal = self.legal_flip(color);
        if legal>0 {
            true
        }else{
            false
        }
    }

    pub fn is_finished(&self)->bool{
        // 終了判定
        if self.is_flippable(BLACK)||self.is_flippable(WHITE) {
            false
        }else{
            true
        }
    }
    pub fn is_win(&self, color:u32)->bool{
        /*
         勝利判定
         前提: 終了判定が済んだboardに対して行う
        */
        let (black, white) = (self.black, self.white);
        let black_num = black.count_ones();
        let white_num = white.count_ones();
        if black_num > white_num {
            if color==BLACK {
                true
            }else{
                false
            }
        }else if white_num > black_num {
            if color==WHITE {
                true
            }else{
                false
            }
        }else{// 引き分けのとき
            false
        }
    }

    pub fn flip_board(&self, color:u32, next:&Move)->Board{
        /*
            input: board, 打ち手の色, 次の手
            output: flipしたあとのboard
        */
        let next:u64 = move_to_bit(next);
        if next >0{
            let rev = self.flippable_stones(color, next);
            if color==BLACK{
                return Board::new(self.black^(rev^next), self.white^rev)
            }else{
                return Board::new(self.black^rev, self.white^(rev^next))
            }
        }else{
            return Board::new(self.black,self.white)
        }
    }

    pub fn get_next(&self, color:u32, count:u32) -> Move{
        /*
            次の手を取得
            思考ルーチンによって変更する必要あり
        */
        let legals = self.legal_flip(color);
        if count > SOLVE_COUNT {
            let next:u64 = get_first_legal(legals); // 先頭のものを取得

            if  next==0 {
                Move::Pass
            }else{
                let (x,y) = bit_to_coordinate(next);
                Move::Mv{x:x, y:y}
            }
        }else{
            self.solve(color, count)
        }
    }

    pub fn legal_flip(&self, color:u32)->u64{
        /*
            ボードと白と黒どちらの手番かを受け取って、
            着手可能な場所をbitで返す関数
        */
        let (player, opponent) = if color==BLACK {(self.black, self.white)}
                                            else {(self.white, self.black)};
        let blank = !(player|opponent);
        let horizontal = opponent & 0x7e7e7e7e7e7e7e7e;
        let vertical = opponent & 0x00FFFFFFFFFFFF00;
        let all_side = opponent & 0x007e7e7e7e7e7e00;
        let mut legal = sub_legal_l(player, horizontal, blank, 1); // 左
        legal |= sub_legal_l(player, vertical, blank, 8); // 上
        legal |= sub_legal_l(player, all_side, blank, 7); // 右上
        legal |= sub_legal_l(player, all_side, blank, 9); // 左上
        legal |= sub_legal_r(player, horizontal, blank, 1); // 右
        legal |= sub_legal_r(player, vertical, blank, 8); // 下
        legal |= sub_legal_r(player, all_side, blank, 7); // 左下
        legal |= sub_legal_r(player, all_side, blank, 9); // 右下
        legal
    }

    pub fn flippable_stones(&self, color:u32, next:u64)->u64{
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

}



fn sub_legal_l(player:u64, masked:u64, blank:u64, num:u64)->u64{
    // legal_flip() 用
    let mut tmp = masked & (player << num);
    tmp |= masked & (tmp << num);
    tmp |= masked & (tmp << num);
    tmp |= masked & (tmp << num);
    tmp |= masked & (tmp << num);
    tmp |= masked & (tmp << num); // bitが立っているのは相手の碁が連続しているところ
    let legal = blank & (tmp << num);
    legal
}
fn sub_legal_r(player:u64, masked:u64, blank:u64, num:u64)->u64{
    // legal_flip() 用
    let mut tmp = masked & (player >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num); // bitが立っているのは相手の碁が連続しているところ
    let legal = blank & (tmp >> num);
    legal
}


fn sub_flippable_l(player:u64, masked:u64, next:u64, num:u32)->u64{
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
fn sub_flippable_r(player:u64, masked:u64, next:u64, num:u32)->u64{
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
