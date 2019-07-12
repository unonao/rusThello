use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};

use crate::color::*;
use crate::play::*;

pub fn make_train_data() -> Result<(), Box<std::error::Error>> {
    let mut fvec = Vec::new();
    for i in 0..60 {
        fvec.push(BufWriter::new(
            File::create(format!("train/train{}.txt", i)).unwrap(),
        ));
    }

    for result in BufReader::new(File::open("./train/pre_train_data.csv").unwrap()).lines() {
        //1試合ごとにループ
        let mut next: u64;
        let mut pre: u64;
        let mut def: i32;
        let mut next_mobility_num_black: u32;
        let mut next_mobility_num_white: u32;
        match result {
            Ok(n) => {
                let mut board = Board::init();
                let mut iter = n.split_whitespace();
                let mut vec_x = Vec::new();
                let mut vec_y = Vec::new();
                for _i in 0..60 {
                    let x: i32 = iter.next().unwrap().parse().unwrap();
                    let y: i32 = iter.next().unwrap().parse().unwrap();
                    vec_x.push(x);
                    vec_y.push(y);
                }
                let result_in_black_view: f32 = iter.next().unwrap().trim_end().parse().unwrap();
                for count in 0..60 {
                    let mut x: i32 = vec_x[count];
                    let mut y: i32 = vec_y[count];
                    if x == 10 {
                        break;
                    } else {
                        let mut color = BLACK;
                        if x > 0 && y > 0 {
                            color = BLACK;
                            x = x - 1;
                            y = y - 1;
                        } else if x < 0 && y < 0 {
                            color = WHITE;
                            x = -x - 1;
                            y = -y - 1;
                        } else {
                            println!("error!!");
                            break;
                        }
                        let next_stone: u64 = coordinate_to_bit(x, y);

                        // invalid move のチェック
                        let (player, opponent) = if color == BLACK {
                            (board.black, board.white)
                        } else {
                            (board.white, board.black)
                        };
                        let mobilitys = mobility_ps(player, opponent);
                        if (mobilitys & next_stone) != next_stone {
                            println!("invalid move!!");
                            break;
                        }

                        board = board.flip_board_by_bit(color, next_stone);

                        def = stone_def(board.black, board.white);
                        next_mobility_num_black =
                            mobility_ps(board.black, board.white).count_ones();
                        next_mobility_num_white =
                            mobility_ps(board.white, board.black).count_ones();
                        writeln!(
                            fvec[count],
                            "{} {} {} {} {} {}",
                            board.black,
                            board.white,
                            def,
                            next_mobility_num_black,
                            next_mobility_num_white,
                            result_in_black_view
                        )?;
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        };
    }
    Ok(())
}
