/*
    boardはu64*2
    座標は(x:u8,y:u8)と受け取る
*/

const  BLACK: u8 = 0;
const  WHITE: u8 = 0;

pub struct Board {
    pub black: u64, pub white: u64
}


pub fn init_board() -> Board {
    /*
        Board型の初期オセロ板を返す関数
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

pub fn coordinate_to_bit(x:u8,y:u8)->u64{
    /*
        0~7のx,yを受け取り、座標をbitに変換
    */
    let mask:u64 = 0x8000000000000000; //左端だけが 1
    mask >> (x+y*8)
}

pub fn legal_flip(board:&Board, color:u8)->u64{
    /*
        ボードと白と黒どちらの手番かを受け取って、
        着手可能な場所をbitboardで返す関数
    */
    /*
        番人0x7e7e7e7e7e7e7e7e
            01111110
            01111110
            01111110
            01111110
            01111110
            01111110
            01111110
            01111110
    */
    let (player, opponent) = if color==BLACK {(board.black, board.white)}
                                        else {(board.white, board.black)};
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

pub fn sub_legal_l(player:u64, masked:u64, blank:u64, num:u64)->u64{
    let mut tmp = masked & (player << num);
    tmp |= masked & (tmp << num);
    tmp |= masked & (tmp << num);
    tmp |= masked & (tmp << num);
    tmp |= masked & (tmp << num);
    tmp |= masked & (tmp << num); // bitが立っているのは相手の碁が連続しているところ
    let legal = blank & (tmp << num);
    legal
}

pub fn sub_legal_r(player:u64, masked:u64, blank:u64, num:u64)->u64{
    let mut tmp = masked & (player >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num); // bitが立っているのは相手の碁が連続しているところ
    let legal = blank & (tmp >> num);
    legal
}


pub fn print_bit_board (board:u64){
    println!("{:08b}", (board) >> 8*7); 
    println!("{:08b}", (board << 8*1) >> 8*7); 
    println!("{:08b}", (board << 8*2) >> 8*7); 
    println!("{:08b}", (board << 8*3) >> 8*7); 
    println!("{:08b}", (board << 8*4) >> 8*7); 
    println!("{:08b}", (board << 8*5) >> 8*7); 
    println!("{:08b}", (board << 8*6) >> 8*7);
    println!("{:08b}", (board << 8*7) >> 8*7);
}

pub fn print_board (board:&Board){
    let (black, white) = (board.black, board.white);
    let blank = !(black|white);
    let mut mask:u64 = 0x8000000000000000;
    println!("board");
    for n in 0..8 {
        let i = 8-n;
        if (mask&blank)==mask {print!(".")} else {print!("{:01b}",(mask&black)>>i*8-1)};
        mask = mask >> 1;
        if (mask&blank)==mask {print!(".")} else {print!("{:01b}",(mask&black)>>i*8-2)};
        mask = mask >> 1;
        if (mask&blank)==mask {print!(".")} else {print!("{:01b}",(mask&black)>>i*8-3)};
        mask = mask >> 1;
        if (mask&blank)==mask {print!(".")} else {print!("{:01b}",(mask&black)>>i*8-4)};
        mask = mask >> 1;
        if (mask&blank)==mask {print!(".")} else {print!("{:01b}",(mask&black)>>i*8-5)};
        mask = mask >> 1;
        if (mask&blank)==mask {print!(".")} else {print!("{:01b}",(mask&black)>>i*8-6)};
        mask = mask >> 1;
        if (mask&blank)==mask {print!(".")} else {print!("{:01b}",(mask&black)>>i*8-7)};
        mask = mask >> 1;
        if (mask&blank)==mask {println!(".")} else {println!("{:01b}",(mask&black)>>i*8-8)};
        mask = mask >> 1;
    }
}