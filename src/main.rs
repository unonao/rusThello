/*

server
    ./reversi-serv -p 3000 -t 500
random
    ./reversi -H "localhost" -p 3000 -n Player2


クライアント
    cargo run -- -H "127.0.0.1" -p 3000 -n rusThello
    cargo run -- -H "127.0.0.1" -p 3000 -n evalTest --nobook -e
    cargo run -- -H "127.0.0.1" -p 3000 -n random
    cargo run -- --verb -e
    cargo run -- --nobook -e
verboseは --verb, debugは --debug, infoはデフォルトで --info
solve_depth(default: 18) -s 23
think_depth(default: 4) -t 7


最強
    cargo run --release -- -H "127.0.0.1" -p 3000 -n rusThello -s 23 -t 7

*/

#![allow(non_snake_case)]

extern crate rusThello;
use rusThello::book::*;
use rusThello::color::*;
use rusThello::command_parser::*;
use rusThello::global::*;
use rusThello::hash::*;
use rusThello::make_train_data::*;
use rusThello::play::*;
use rusThello::print::*;
use rusThello::train::*;

// サーバ接続
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
use std::net::TcpStream;

pub fn input_command(reader: &mut BufReader<&TcpStream>) -> Message {
    // serverからのコマンドを一行読み込んでパース
    let mut message = String::new();
    reader.read_line(&mut message).expect("Could not read!");
    //
    if ARGS.level.as_str() == "verb" {
        println!("{}", message); // input 内容を出力
    }
    match command_parse(message.as_str()) {
        Ok((_input, message)) => message,
        _ => {
            println!("input error");
            Message::Giveup
        }
    }
}

pub fn output_command(writer: &mut BufWriter<&TcpStream>, command: String) {
    // serverへStringを送信
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
    _time: i32,
) {
    let pmove: Move = board.get_next(color, count);
    let bit: u64 = move_to_bit(&pmove);
    if count == 60 {
        init_mirror_num(bit)
    }
    insert_hist(bit, count);
    let board = board.flip_board_by_move(color, &pmove);
    let move_send = format!("MOVE {}\n", move_to_string(&pmove));

    if ARGS.level.as_str() == "verb" {
        print_board(&board);
        println!("my move: {}", move_send);
    }
    let count = match pmove {
        Move::Pass => count,
        _ => count - 1,
    };

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
) {
    match input_command(reader) {
        Message::Move { x, y } => {
            let bit = coordinate_to_bit(x, y);
            if count == 60 {
                init_mirror_num(bit)
            }
            insert_hist(bit, count);

            let omove: Move = Move::Mv { x: x, y: y };
            let board = board.flip_board_by_move(opposite_color(color), &omove);
            my_move(
                &mut writer,
                &mut reader,
                board,
                count - 1,
                color,
                opponent_name,
                time,
            )
        }
        Message::Pass => my_move(
            &mut writer,
            &mut reader,
            board,
            count,
            color,
            opponent_name,
            time,
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
    win_lose: String,
    n: i32,
    m: i32,
    reason: String,
) {
    println!(
        "Oppnent name: {} (color:{}).",
        opponent_name,
        opposite_color(color)
    );
    println!("Your name: {} (color:{}).\n", ARGS.name, color);
    print_board(&board);
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

    if color == "BLACK" {
        my_move(
            &mut writer,
            &mut reader,
            board,
            60,
            BLACK,
            opponent_name,
            time,
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
        )
    }
}

fn wait_start(mut writer: &mut BufWriter<&TcpStream>, mut reader: &mut BufReader<&TcpStream>) {
    /*
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
    //サーバーへ接続し、OPEN nameを送信。wait_startを呼び出す

    // サーバーへ接続
    let addr = format!("{}:{}", ARGS.host, ARGS.port);
    println!("Connecting to {}.", addr);
    let stream = TcpStream::connect(addr).expect("Connection refused");
    let mut writer = BufWriter::new(&stream);
    let mut reader = BufReader::new(&stream);

    // OPEN name を送信
    let open_and_name = format!("OPEN {}\n", ARGS.name);
    output_command(&mut writer, open_and_name);
    println!("waiting start ...");

    wait_start(&mut writer, &mut reader);
}

fn main() {
    if ARGS.mktrain {
        match make_train_data() {
            Ok(_n) => println!("ok!"),
            Err(_n) => println!("err!"),
        };
    } else if ARGS.dotrain {
        train()
    } else if ARGS.cntntrain {
        train_continue()
    } else {
        init_rand_mask();
        make_book();

        // クライアントとして接続
        client()
    }
}
