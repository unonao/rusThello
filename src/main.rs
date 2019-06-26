/*
クライアント
    cargo run "127.0.0.1" 30000 Player1

*/


#![allow(non_snake_case)]

// 自作モジュール
mod play;
use play::*;


// サーバ接続
use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::io::{BufWriter, Write};



// コマンドライン引数
use std::env;
fn get_args()->(String,String,String){
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    let opt_host = &args[1];
    let opt_port = &args[2];
    let opt_player_name = &args[3];
    (opt_host.to_string() ,opt_port.to_string() ,opt_player_name.to_string())
}




fn client(host:String, port:String, name:String){
    let addr    = format!("{}:{}", host, port);
    let _ = println!("Connecting to {}.", addr);
    let mut stream = TcpStream::connect(addr).expect("Connection refused");
    let mut reader = BufReader::new(stream);
    let mut message = String::new();
    reader.read_line(&mut message).expect("Could not read!");
    println!("{}",message);
    // wait_start
}

fn main() {
    //play_me_vs_me();

/*
    let mut opt_player_name:String =  "Anon.".to_string();
    let mut opt_port:String        =  "3000".to_string();
    let mut opt_host:String        =  "127.0.0.1".to_string();
    */
    // コマンドライン引数を変数に保存
    let (opt_host, opt_port, opt_player_name) = get_args();

    // クライアントとして接続
    client(opt_host, opt_port, opt_player_name)
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
