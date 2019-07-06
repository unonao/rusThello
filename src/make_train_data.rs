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
        let mut next_mobility_num: u32; // passのときは盤面評価をしないこととする。(つまり0はない)
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
                    let x: i32 = vec_x[count];
                    let y: i32 = vec_y[count];
                    if x == 10 {
                        break;
                    } else if x > 0 && y > 0 {
                        let color = BLACK;
                        let x = x - 1;
                        let y = y - 1;
                        let next_stone: u64 = coordinate_to_bit(x, y);
                        board = board.flip_board_by_bit(color, next_stone);

                        let mobility = mobility_ps(board.white, board.black);
                        if mobility == 0 {
                            //whiteの PASS
                            next = board.black;
                            pre = board.white;
                            def = stone_def(next, pre);
                            next_mobility_num = mobility_ps(next, pre).count_ones();
                            writeln!(
                                fvec[count],
                                "{} {} {} {} {}",
                                next, pre, def, next_mobility_num, result_in_black_view
                            )?;
                        } else {
                            pre = board.black;
                            next = board.white;
                            def = stone_def(next, pre);
                            next_mobility_num = mobility_ps(next, pre).count_ones();
                            writeln!(
                                fvec[count],
                                "{} {} {} {} {}",
                                next, pre, def, next_mobility_num, -result_in_black_view
                            )?;
                        }
                    } else if x < 0 && y < 0 {
                        let color = WHITE;
                        let x = -x - 1;
                        let y = -y - 1;
                        let next_stone: u64 = coordinate_to_bit(x, y);
                        board = board.flip_board_by_bit(color, next_stone);

                        let mobility = mobility_ps(board.black, board.white);
                        if mobility == 0 {
                            //black PASS
                            pre = board.black;
                            next = board.white;
                            def = stone_def(next, pre);
                            next_mobility_num = mobility_ps(next, pre).count_ones();
                            writeln!(
                                fvec[count],
                                "{} {} {} {} {}",
                                next, pre, def, next_mobility_num, -result_in_black_view
                            )?;
                        } else {
                            next = board.black;
                            pre = board.white;
                            def = stone_def(next, pre);
                            next_mobility_num = mobility_ps(next, pre).count_ones();
                            writeln!(
                                fvec[count],
                                "{} {} {} {} {}",
                                next, pre, def, next_mobility_num, result_in_black_view
                            )?;
                        }
                    } else {
                        println!("error!!");
                        break;
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        };
    }
    Ok(())
}
