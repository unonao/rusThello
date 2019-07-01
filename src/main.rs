/*

server
./reversi-serv -p 30000 -t 500
random
./reversi -H "localhost" -p 30000 -n Player2

クライアント
cargo run "127.0.0.1" 30000 rusThello
cargo run --release "127.0.0.1" 30000 rusThello

*/


#![allow(non_snake_case)]



extern crate rusThello;
use rusThello::play::*;
use rusThello::command_parser::*;
use rusThello::print::*;
use rusThello::color::*;
use rusThello::hash::*;

// サーバ接続
use std::net::TcpStream;
use std::io::{BufReader,BufRead};
use std::io::{BufWriter, Write};
// serverからのコマンドを一行読み込んでパース
pub fn input_command (reader: &mut BufReader<&TcpStream>) -> Message {
    let mut message = String::new();
    reader.read_line(&mut message).expect("Could not read!");

    // println!("{}",message); // input 内容を出力

    match command_parse(message.as_str()) {
        Ok((_input, message)) => {
            message
        }
        _ => {
            println!("input error");
            Message::Giveup
        }
    }
}
// serverへStringを送信
pub fn output_command (writer:&mut BufWriter<&TcpStream>, command:String) {
    writer.write(command.as_bytes()).expect("Write failed");
    let _ = writer.flush();
}



// コマンドライン引数
use std::env;
fn get_args()->(String,String,String){
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    let opt_host = if len>0 {&args[1]} else {"127.0.0.1"};
    let opt_port = if len>1 {&args[2]} else {"30000"};
    let opt_player_name = if len>2 {&args[3]} else {"rusThello"};
    (opt_host.to_string() ,opt_port.to_string() ,opt_player_name.to_string())
}




fn my_move(mut writer:&mut BufWriter<&TcpStream>, mut reader: &mut BufReader<&TcpStream>, board:Board, count:i32, color:i32, opponent_name:String, time:i32, mut hist:&mut Vec<Move>){
    let pmove:Move = board.get_next(color, count); // 次に打つ手
    let board = board.flip_board_by_move(color, &pmove);
    let move_send = format!("MOVE {}\n", move_to_string(&pmove));

    //print_board(&board);
    //println!("my_move {}", move_send);
    let count = match pmove {
        Move::Pass => count,
        _ => count-1
    };

    hist.push(pmove);
    output_command(&mut writer, move_send);
    match input_command(&mut reader){
        Message::Ack{time} =>
        op_move(&mut writer, &mut reader, board, count, color, opponent_name, time, &mut hist),
        Message::End{win_lose,n,m,reason} =>
        proc_end(&mut writer, &mut reader, board, color, opponent_name, &mut hist, win_lose,n,m,reason),
        _ =>
        println!("Invalid Command")
    }
}

fn op_move(mut writer:&mut BufWriter<&TcpStream>, mut reader: &mut BufReader<&TcpStream>, board:Board, count:i32, color:i32, opponent_name:String, time:i32, mut hist:&mut Vec<Move>){
    match input_command(reader){
        Message::Move{x, y} =>{
            let omove:Move = Move::Mv{x:x, y:y};
            let board = board.flip_board_by_move(opposite_color(color), &omove);
            hist.push(omove);
            my_move(&mut writer, &mut reader, board, count-1, color, opponent_name, time, &mut hist)
        }
        Message::Pass =>{
            hist.push(Move::Pass);
            my_move(&mut writer, &mut reader, board, count, color, opponent_name, time, &mut hist)
        }
        Message::End{win_lose,n,m,reason} =>
        proc_end(&mut writer, &mut reader, board, color, opponent_name, &mut hist, win_lose,n,m,reason),
        _ => {
            println!("Invalid Command")
        }
    }
}

fn proc_end(mut writer:&mut BufWriter<&TcpStream>, mut reader: &mut BufReader<&TcpStream>, board:Board, color:i32, opponent_name:String, hist:&mut Vec<Move>, win_lose:String, n:i32,  m:i32, reason:String){
    /*
    println!("Oppnent name: {} ({}).\n", opponent_name, opposite_color(color));
    print_board(&board);
    println!("{}",board.is_win(color));

    match win_lose.as_str() {
        "WIN" => println!("You win! ({} vs. {}) -- {}.\n", n,m,reason),
        "LOSE" => println!("You lose! ({} vs. {}) -- {}.\n", n,m,reason),
        "TIE" => println!("Draw! ({}vs. {}) -- {}.\n", n,m,reason),
        _ => println!("parse error!")
    };*/

    wait_start(&mut writer, &mut reader);

}


// ゲームスタート
fn start_game(mut writer:&mut BufWriter<&TcpStream>, mut reader: &mut BufReader<&TcpStream>, color:String, opponent_name:String, time:i32){
    let board = Board::init() ;
    
    let mut hist_vec: Vec<Move> = Vec::new();
    if color=="BLACK" {
        my_move(&mut writer, &mut reader, board, 60, BLACK, opponent_name, time, &mut hist_vec)
    }else{
        op_move(&mut writer, &mut reader, board, 60, WHITE, opponent_name, time, &mut hist_vec)
    }
}


// スタート待ち
fn wait_start(mut writer:&mut BufWriter<&TcpStream>, mut reader: &mut BufReader<&TcpStream>){
    /*
    input : writer, reader
    "START color opponent_name time"を受け取るまで待機
    */
    match input_command(&mut reader) {
        Message::Bye{stats} => print_stats(stats),
        Message::Start{color, name, time} => {
            start_game(&mut writer, &mut reader, color, name, time)},
            _ => println!("Invalid Command")
        }

    }


    fn client(host:String, port:String, name:String){
        /*
        input : ホスト、ポート、プレーヤー名
        サーバーへ接続し、OPEN nameを送信。wait_startを呼び出す
        */


        // サーバーへ接続
        let addr    = format!("{}:{}", host, port);
        println!("Connecting to {}.", addr);
        let stream = TcpStream::connect(addr).expect("Connection refused");
        let mut writer = BufWriter::new(&stream);
        let mut reader = BufReader::new(&stream);

        // OPEN name を送信
        let open_and_name = format!("OPEN {}\n", name);
        output_command(&mut writer, open_and_name);

        wait_start(&mut writer, &mut reader);
    }

    fn main() {
        // コマンドライン引数を変数に保存
        let (opt_host, opt_port, opt_player_name) = get_args();
        init_rand_mask();
        // クライアントとして接続
        client(opt_host, opt_port, opt_player_name)
    }








    /*
    fn test_play(){
    let mut board = Board::init();
    let player_color = BLACK;
    print_board(&board);

    let flippable:u64 = board.mobility_ps(player_color);
    println!("flippable");
    print_unilateral(&flippable);

    let next:u64 = get_first_mobility(flippable);
    println!("next");
    print_unilateral(&next);

    let flippable:u64 = board.flippable_stones(player_color, next);
    println!("flippable");
    print_unilateral(&flippable);

    board = board.flip_board(player_color, flippable);
    print_board(&board);

}

fn play_me_vs_me(){
let mut board = Board::init();
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
let n: i32 = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
let m: i32 = ws.next().unwrap().parse().unwrap();
(n, m)
};
if n>8 {break}
let flippable:u64 = board.mobility_ps(player_color);
if flippable == 0 {
println!("end game!");
}
let next:u64 = coordinate_to_bit(n,m);
if (next&flippable)!=next {
println!("not flippable!");
continue
}
board = board.flip_board(player_color,next);
player_color = player_color^1;
}

}



*/
