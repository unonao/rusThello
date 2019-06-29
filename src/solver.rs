/*

    solver.rs: 終盤ソルバー用のファイル

*/


use crate::color::*;
use crate::play::*;

struct NextAndFlippable{
    next:u64,
    f_num:u64
}

pub fn solve(board:&Board, my_color:u32)->Move{
    /*
        読み切りをして、次の手を返す
    */
    let legals = board.legal_flip(my_color);
    let mut mask:u64 = 0x8000000000000000;
    let mut next_vec: Vec<NextAndFlippable> = Vec::new();
    while mask>0 {
        if (mask&legals)>0{
            let next_board = board.flip_board(my_color,bit_to_move(mask));
            let op_flippable = next_board.legal_flip(opposite_color(my_color));
            next_vec.push(next:mask, f_num: op_flippable.count_ones())
            mask>>1;
        }
    }

}

fn rec_solver(board:&Board, my_color:u32, turn_color:u32)->bool{
    let (black, white) = (board.black, board.white);
    if my_color==turn_color{ // 自身の手に関しては、勝利するものが見つかれば終了

    }else{ // 相手の手に関しては、すべての手に関して勝利する必要あり

    }
}
