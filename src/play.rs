/*
    boardはu64*2
    座標は(x:u32,y:u32)と受け取る

    use num_traits::PrimInt;
    n.count_ones(); // ビット表現したときに現れる1の数を求める
    n.count_zeros(); // ビットで表現したときに現れる0の数を求める
    n.leading_zeros(); // ビットで表現したときの頭の0の数を求める
    n.trailing_zeros(); // ビットで表現したときの末尾の0の数を求める
    n.swap_bytes(); // byte順序を逆にする
    n.rotate_right(4); // ラップする右シフト 
    n.rotate_left(4); // ラップする左シフト
*/


pub const  BLACK: u32 = 0;
pub const  WHITE: u32 = 1;

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



pub fn coordinate_to_bit(x:u32,y:u32)->u64{
    /*
        1~8のx,yを受け取り、座標をbitに変換
    */
    let mask:u64 = 0x8000000000000000; //左端だけが 1
    mask >> ((x-1)+(y-1)*8)
}
pub fn bit_to_coordinate(mask:u64)->(u32, u32){
    /*
        input : u64で表した座標
        output : 1~8の座標x,y
    */
    let lead_zeros = mask.leading_zeros()+1;
    let x:u32 =  (lead_zeros / 8).into();
    let y:u32 =  (lead_zeros - (x*8)).into();
    (x+1,y+1)
}


pub fn legal_flip(board:&Board, color:u32)->u64{
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
fn sub_legal_l(player:u64, masked:u64, blank:u64, num:u64)->u64{
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
    let mut tmp = masked & (player >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num);
    tmp |= masked & (tmp >> num); // bitが立っているのは相手の碁が連続しているところ
    let legal = blank & (tmp >> num);
    legal
}




pub fn reverse_stones(board:&Board, color:u32, next:u64)->u64{
    /*
        input : ボード, 白と黒どちらの手番か, 着手箇所
        output : ひっくり返る場所
    */
    let (player, opponent) = if color==BLACK {(board.black, board.white)}
                                        else {(board.white, board.black)};

    let blank_h = !(player | opponent & 0x7e7e7e7e7e7e7e7e);
    let blank_v = !(player | opponent & 0x00ffffffffffff00);
    let blank_a = !(player | opponent & 0x007e7e7e7e7e7e00);
    let mut rev = sub_reverse_l(player, blank_h, next, 1); // 左
    rev |= sub_reverse_l(player, blank_v, next, 8); // 上
    rev |= sub_reverse_l(player, blank_a, next, 7); // 右上
    rev |= sub_reverse_l(player, blank_a, next, 9); // 左上
    rev |= sub_reverse_r(player, blank_h, next, 1); // 右
    rev |= sub_reverse_r(player, blank_v, next, 8); // 下
    rev |= sub_reverse_r(player, blank_a, next, 7); // 左下
    rev |= sub_reverse_r(player, blank_a, next, 9); // 右下
    rev
}
fn sub_reverse_l(player:u64, masked:u64, next:u64, num:u32)->u64{
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
fn sub_reverse_r(player:u64, masked:u64, next:u64, num:u32)->u64{
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



pub fn reverse_board(board:&Board, color:u32, next:u64)->Board{
    let rev = reverse_stones(board, color, next);
    if color==BLACK{
        return Board{black:(board.black^(rev^next)), white:(board.white^rev)}
    }else{
        return Board{black:(board.black^rev), white:(board.white^(rev^next))}
    }
}





pub fn print_bit (board:&u64){
    println!(" 12345678");
    println!("1{:08b}", (board) >> 8*7); 
    println!("2{:08b}", (board << 8*1) >> 8*7); 
    println!("3{:08b}", (board << 8*2) >> 8*7); 
    println!("4{:08b}", (board << 8*3) >> 8*7); 
    println!("5{:08b}", (board << 8*4) >> 8*7); 
    println!("6{:08b}", (board << 8*5) >> 8*7); 
    println!("7{:08b}", (board << 8*6) >> 8*7);
    println!("8{:08b}", (board << 8*7) >> 8*7);
}

pub fn print_unilateral (flippable:&u64){
    let mut mask:u64 = 0x8000000000000000;
    println!(" 12345678");
    for n in 0..8 {
        if (mask&flippable)!=mask {print!("{}.",n+1)} else {print!("{}1",n+1)};
        mask = mask >> 1;
        if (mask&flippable)!=mask {print!(".")} else {print!("1")};
        mask = mask >> 1;
        if (mask&flippable)!=mask {print!(".")} else{print!("1")};
        mask = mask >> 1;
        if (mask&flippable)!=mask {print!(".")} else {print!("1")};
        mask = mask >> 1;
        if (mask&flippable)!=mask {print!(".")} else {print!("1")};
        mask = mask >> 1;
        if (mask&flippable)!=mask {print!(".")} else {print!("1")};
        mask = mask >> 1;
        if (mask&flippable)!=mask {print!(".")} else {print!("1")};
        mask = mask >> 1;
        if (mask&flippable)!=mask {println!(".")} else {print!("1")};
        mask = mask >> 1;
    }
    println!("");
}

pub fn print_board (board:&Board){
    let (black, white) = (board.black, board.white);
    let blank = !(black|white);
    let mut mask:u64 = 0x8000000000000000;
    println!("board");
    println!(" 12345678");
    for n in 0..8 {
        let i = 8-n;
        if (mask&blank)==mask {print!("{}.",n+1)} else {print!("{}{:01b}",n+1,(mask&black)>>i*8-1)};
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
    println!("");
}