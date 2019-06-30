/*

    solver.rs: 終盤ソルバー用のファイル

*/


use crate::color::*;
use crate::play::*;

pub const  SOLVE_COUNT: i32 = 16;

pub struct NextAndFlippable{
    pub next:u64,
    pub board:Board,
    pub f_num:i32
}

/**/
impl Board{
    pub fn solve(&self, my_color:i32, count:i32)->u64{
        /*
            読み切りをして、次の手を返す
            速さ優先探索: 相手が次打てる手が少ないものから探索
        */
        let mobilitys = self.mobility_ps(my_color);
        if mobilitys==0{ // passのとき
            return 0
        }else{
            let mut next_vec: Vec<NextAndFlippable> = Vec::new();
            let mut mask:u64 = 0x8000000000000000;
            while mask>0 {
                if (mask&mobilitys)>0{
                    let next_board = self.flip_board(my_color, mask);
                    let op_flippable_num = next_board.mobility_ps(opposite_color(my_color)).count_ones() as i32;
                    next_vec.push(NextAndFlippable{next:mask, board:next_board, f_num: op_flippable_num});
                }
                mask = mask>>1;
            }

            next_vec.sort_unstable_by(|a,b| a.f_num.cmp(&b.f_num)); // f_numについて昇順にsort

            for next_and_f in &next_vec {
                if next_and_f.board.rec_solver(my_color, opposite_color(my_color), count-1) { // 見つけたら終了
                    // println!("solved!");
                    return next_and_f.next
                }
            }
            return next_vec[0].next
        }
    }


    fn rec_solver(&self, my_color:i32, turn_color:i32, count:i32)->bool{

        if count==0{ // boardが埋まったとき
            return self.is_win(my_color)
        }


        let mobilitys = self.mobility_ps(turn_color);

        if mobilitys==0{ // passのとき
            if self.is_finished(){
                return self.is_win(my_color)
            }else{
                self.rec_solver(my_color, opposite_color(turn_color), count)
            }
        }else{

            let mut mask:u64 = 0x8000000000000000;
            let mut next_vec: Vec<NextAndFlippable> = Vec::new();
            while mask>0 {
                if (mask&mobilitys)>0{
                    let next_board = self.flip_board(turn_color, mask);
                    let op_flippable_num = next_board.mobility_ps(turn_color).count_ones() as i32;
                    next_vec.push(NextAndFlippable{next:mask, board:next_board, f_num: op_flippable_num});
                }
                mask = mask>>1;
            }

            if my_color==turn_color{ // 自身の手に関しては、勝利するものが見つかれば終了

                if count > 6{ // 最終6手ほどからは、ソートせずに全探索
                    next_vec.sort_unstable_by(|a,b| a.f_num.cmp(&b.f_num)); // f_numについて昇順に
                }

                if count==2{ // 最終1手(相手)をその場で処理
                    let next = next_vec[0].board.mobility_ps(opposite_color(turn_color));
                    let final_board = next_vec[0].board.flip_board(opposite_color(turn_color), next);
                    return final_board.is_win(my_color)
                }else{
                    for next_and_f in next_vec {
                        if next_and_f.board.rec_solver(my_color, opposite_color(turn_color), count-1) { // 見つけたら終了
                            return true
                        }
                    }
                }
                return false
            }else{ // 相手の手に関しては、すべての手に関して勝利する必要あり
                if count==2{ // 最終1手(相手)をその場で処理
                    let next = next_vec[0].board.mobility_ps(opposite_color(turn_color));
                    let final_board = next_vec[0].board.flip_board(opposite_color(turn_color), next);
                    return final_board.is_win(my_color)
                }else{
                    for next_and_f in next_vec {
                        if !(next_and_f.board.rec_solver(my_color, opposite_color(turn_color), count-1)) { // 負けたら終了
                            return false
                        }
                    }
                }
                return true
            }

        }




    }
}
