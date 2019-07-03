/*

server
    ./reversi-serv -p 3000 -t 500
random
    ./reversi -H "localhost" -p 3000 -n Player2


クライアント
    cargo run -h "127.0.0.1" -p 3000 -n rusThello
    cargo run
verboseは -v
solve_depth(default: 18) -s 23
think_depth(default: 4) -t 7


最強
    cargo run --release -- -h "127.0.0.1" -p 3000 -n rusThello -s 23 -t 7

*/

#![allow(non_snake_case)]

extern crate rusThello;
use rusThello::color::*;
use rusThello::command_parser::*;
use rusThello::global::*;
use rusThello::hash::*;
use rusThello::play::*;
use rusThello::print::*;

// サーバ接続
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
// serverへStringを送信
use std::net::TcpStream;
// serverからのコマンドを一行読み込んでパース
pub fn input_command(reader: &mut BufReader<&TcpStream>) -> Message {
    let mut message = String::new();
    reader.read_line(&mut message).expect("Could not read!");
    // println!("{}",message); // input 内容を出力
    match command_parse(message.as_str()) {
        Ok((_input, message)) => message,
        _ => {
            println!("input error");
            Message::Giveup
        }
    }
}
// serverへStringを送信
pub fn output_command(writer: &mut BufWriter<&TcpStream>, command: String) {
    writer.write(command.as_bytes()).expect("Write failed");
    let _ = writer.flush();
}

fn my_move(
    mut writer: &mut BufWriter<&TcpStream>,
    mut reader: &mut BufReader<&TcpStream>,
    board: Board,
    count: i32,
    color: i32,
    opponent_name: String,
    time: i32,
    mut hist: &mut Vec<Move>,
) {
    let pmove: Move = board.get_next(color, count); // 次に打つ手
    let board = board.flip_board_by_move(color, &pmove);
    let move_send = format!("MOVE {}\n", move_to_string(&pmove));

    //print_board(&board);
    //println!("my_move {}", move_send);
    let count = match pmove {
        Move::Pass => count,
        _ => count - 1,
    };

    hist.push(pmove);
    output_command(&mut writer, move_send);
    match input_command(&mut reader) {
        Message::Ack { time } => op_move(
            &mut writer,
            &mut reader,
            board,
            count,
            color,
            opponent_name,
            time,
            &mut hist,
        ),
        Message::End {
            win_lose,
            n,
            m,
            reason,
        } => proc_end(
            &mut writer,
            &mut reader,
            board,
            color,
            opponent_name,
            &mut hist,
            win_lose,
            n,
            m,
            reason,
        ),
        _ => println!("Invalid Command"),
    }
}

fn op_move(
    mut writer: &mut BufWriter<&TcpStream>,
    mut reader: &mut BufReader<&TcpStream>,
    board: Board,
    count: i32,
    color: i32,
    opponent_name: String,
    time: i32,
    mut hist: &mut Vec<Move>,
) {
    match input_command(reader) {
        Message::Move { x, y } => {
            let omove: Move = Move::Mv { x: x, y: y };
            let board = board.flip_board_by_move(opposite_color(color), &omove);
            hist.push(omove);
            my_move(
                &mut writer,
                &mut reader,
                board,
                count - 1,
                color,
                opponent_name,
                time,
                &mut hist,
            )
        }
        Message::Pass => {
            hist.push(Move::Pass);
            my_move(
                &mut writer,
                &mut reader,
                board,
                count,
                color,
                opponent_name,
                time,
                &mut hist,
            )
        }
        Message::End {
            win_lose,
            n,
            m,
            reason,
        } => proc_end(
            &mut writer,
            &mut reader,
            board,
            color,
            opponent_name,
            &mut hist,
            win_lose,
            n,
            m,
            reason,
        ),
        _ => println!("Invalid Command"),
    }
}

fn proc_end(
    mut writer: &mut BufWriter<&TcpStream>,
    mut reader: &mut BufReader<&TcpStream>,
    board: Board,
    color: i32,
    opponent_name: String,
    hist: &mut Vec<Move>,
    win_lose: String,
    n: i32,
    m: i32,
    reason: String,
) {
    /*
                    println!("Oppnent name: {} ({}).\n", opponent_name, opposite_color(color));
                    print_board(&board);
                    println!("{}",board.is_win(color));
    */
    match win_lose.as_str() {
        "WIN" => println!("You win! ({} vs. {}) -- {}.\n", n, m, reason),
        "LOSE" => println!("You lose! ({} vs. {}) -- {}.\n", n, m, reason),
        "TIE" => println!("Draw! ({}vs. {}) -- {}.\n", n, m, reason),
        _ => println!("parse error!"),
    };

    wait_start(&mut writer, &mut reader);
}

// ゲームスタート
fn start_game(
    mut writer: &mut BufWriter<&TcpStream>,
    mut reader: &mut BufReader<&TcpStream>,
    color: String,
    opponent_name: String,
    time: i32,
) {
    let board = Board::init();

    let mut hist_vec: Vec<Move> = Vec::new();
    if color == "BLACK" {
        my_move(
            &mut writer,
            &mut reader,
            board,
            60,
            BLACK,
            opponent_name,
            time,
            &mut hist_vec,
        )
    } else {
        op_move(
            &mut writer,
            &mut reader,
            board,
            60,
            WHITE,
            opponent_name,
            time,
            &mut hist_vec,
        )
    }
}

// スタート待ち
fn wait_start(mut writer: &mut BufWriter<&TcpStream>, mut reader: &mut BufReader<&TcpStream>) {
    /*
    input : writer, reader
    "START color opponent_name time"を受け取るまで待機
    */

    init_hashmap();
    match input_command(&mut reader) {
        Message::Bye { stats } => print_stats(stats),
        Message::Start { color, name, time } => {
            start_game(&mut writer, &mut reader, color, name, time)
        }
        _ => println!("Invalid Command"),
    }
}

fn client() {
    /*
    サーバーへ接続し、OPEN nameを送信。wait_startを呼び出す
    */

    // サーバーへ接続
    let addr = format!("{}:{}", ARGS.host, ARGS.port);
    println!("Connecting to {}.", addr);
    let stream = TcpStream::connect(addr).expect("Connection refused");
    let mut writer = BufWriter::new(&stream);
    let mut reader = BufReader::new(&stream);

    // OPEN name を送信
    let open_and_name = format!("OPEN {}\n", ARGS.name);
    output_command(&mut writer, open_and_name);

    wait_start(&mut writer, &mut reader);
}

fn main() {
    init_rand_mask();
    // クライアントとして接続
    client()
}
