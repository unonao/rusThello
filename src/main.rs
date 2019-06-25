mod play;

fn main() {
    let mut board = play::init_board();

    let black_board = board.black;
    println!("{:b}", black_board); 
}
