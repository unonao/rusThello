#![allow(non_snake_case)]
mod play;

fn main() {
    let mut board = play::init_board();
    play::print_board(&board);
    //let black_board = board.black;
    //play::print_bit_board(black_board);
    
}
