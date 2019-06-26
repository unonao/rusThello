#![allow(non_snake_case)]
mod play;
use play::*;


/*
    黒が先手
*/

fn main() {
    play_me_vs_me();
}

fn test_play(){
    let mut board = init_board();
    let player_color = BLACK;
    print_board(&board);

    let flippable:u64 = legal_flip(&board, player_color);
    println!("flippable");
    print_unilateral(&flippable);

    let next:u64 = get_next(flippable);
    println!("next");
    print_unilateral(&next);

    let reverse:u64 = reverse_stones(&board, player_color, next);
    println!("reverse");
    print_unilateral(&reverse);

    board = reverse_board(&board,player_color,next);
    print_board(&board);
    
}

fn play_me_vs_me(){
    let mut board = init_board();
    let mut player_color = BLACK;
    loop {
        print_board(&board);
        if player_color==BLACK {
            println!("Black 1");
        }else{
            println!("White 0");
        }
        let s = {
            let mut s = String::new(); // バッファを確保
            std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
            s.trim_right().to_owned() // 改行コードが末尾にくっついてくるので削る
        };
        let (n, m) = {
            let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
            let n: u32 = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
            let m: u32 = ws.next().unwrap().parse().unwrap();
            (n, m)
        };
        if n>8 {break}
        let flippable:u64 = legal_flip(&board, player_color);
        if flippable == 0 {
            println!("end game!");
        }
        let next:u64 = coordinate_to_bit(n,m);
        if (next&flippable)!=next {
            println!("not flippable!");
            continue
            }
        board = reverse_board(&board,player_color,next);
        player_color = player_color^1;
    }
    
}


fn get_next(flippable:u64)->u64{
    let mut mask:u64 = 0x8000000000000000;
    if flippable == 0 {
        return 0;
    }else{
        for _i in 0..64 {
            if (mask&flippable)==mask {
                return mask
            }
            mask = mask >> 1;
        }
    }
    return mask
}