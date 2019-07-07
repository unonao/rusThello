/*
    オープニングブック用
*/
use crate::color::*;
use crate::global::*;
use crate::play::*;
use crate::print::*;
use crate::rotate::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Bound::Included;

pub fn make_book() {
    for result in BufReader::new(File::open("./book/xxx_composed.csv").unwrap()).lines() {
        match result {
            Ok(n) => {
                let mut book = BOOK.write().unwrap();
                let mut iter = n.split_whitespace();
                let stone: u128 = iter.next().unwrap().parse().unwrap();
                let result_in_black_view: f32 = iter.next().unwrap().trim_end().parse().unwrap();
                //println!("{}: {}", &stone, result_in_black_view);
                book.insert(stone, result_in_black_view);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

pub fn init_mirror_num(first: u64) {
    if ARGS.level.as_str() == "debug" {
        print_unilateral(&first);
    }
    unsafe {
        HIST = 0;
    }
    let d3: u64 = 0x0000100000000000;
    let c4: u64 = flip_diag_a1h8(&d3);
    let f5: u64 = rotate180(&flip_diag_a1h8(&d3));
    let e6: u64 = rotate180(&d3);

    if first == d3 {
        unsafe {
            MIRROR_NUM = 0;
            println!("MIRROR_NUM: {}", MIRROR_NUM);
        }
    } else if first == c4 {
        unsafe {
            MIRROR_NUM = 1;
            println!("MIRROR_NUM: {}", MIRROR_NUM);
        }
    } else if first == f5 {
        unsafe {
            MIRROR_NUM = 2;
            println!("MIRROR_NUM: {}", MIRROR_NUM);
        }
    } else if first == e6 {
        unsafe {
            MIRROR_NUM = 3;
            println!("MIRROR_NUM: {}", MIRROR_NUM);
        }
    }
}

pub fn insert_hist(next: u64, count_1: i32) {
    // count<39ならなにもしない
    let count = count_1 - 1;
    if count >= 39 {
        let mut next_mirror = 0;
        unsafe {
            if MIRROR_NUM == 0 {
                next_mirror = next;
            } else if MIRROR_NUM == 1 {
                next_mirror = flip_diag_a1h8(&next);
            } else if MIRROR_NUM == 2 {
                next_mirror = rotate180(&flip_diag_a1h8(&next));
            } else if MIRROR_NUM == 3 {
                next_mirror = rotate180(&next);;
            }
        }

        let (x, y) = bit_to_coordinate(next_mirror);
        if ARGS.level.as_str() == "debug" {
            println!(
                "insert x:{},y:{},count{}, shift:{}",
                x,
                y,
                count,
                (count - 39) * 6
            );
        }

        let tmp: u128 = (((x as u128) << 3) + (y as u128)) << ((count - 39) * 6); // 最初1手はcount59で、120bitシフトしたい
        unsafe {
            HIST = HIST | tmp;
        }
    }
}

pub fn get_by_book(color: i32, count: i32) -> u64 {
    let next = book_search(color, count);
    unsafe {
        if MIRROR_NUM == 0 {
            next
        } else if MIRROR_NUM == 1 {
            flip_diag_a1h8(&next)
        } else if MIRROR_NUM == 2 {
            rotate180(&flip_diag_a1h8(&next))
        } else if MIRROR_NUM == 3 {
            rotate180(&next)
        } else {
            next
        }
    }
}

pub fn book_search(color: i32, count_1: i32) -> u64 {
    let count = count_1 - 1;
    // passでないときに呼び出す
    if ARGS.level.as_str() == "debug" {
        println!(
            "book search! count:{}, shift:{}",
            count,
            ((60 - count - 1) * 6) + 2,
        );
    }

    let mask: u128 = std::u128::MAX;
    let mut hist: u128;
    unsafe {
        hist = HIST;
    }
    let last = hist | (mask >> ((60 - count - 1) * 6) + 2); // はじめの1手目でcount = 59なら、maskを1*6+2ずらす
                                                            //println!("hist:{}", hist);
                                                            //println!("last:{}", last);
    let book = BOOK.read().unwrap();
    if color == BLACK {
        // 黒にとっての評価なので、最大を求める
        let mut max_key: u128 = 0;
        let mut max_val: f32 = std::f32::MIN;
        for (&key, &value) in book.range((Included(hist), Included(last))) {
            if max_val < value {
                max_key = key;
                max_val = value;
            }
        }
        if max_key == 0 {
            println!("no hit");
            return 0;
        } else {
            let next = (max_key << ((60 - count - 1) * 6 + 2)) >> 122; // 必要な6bitを左に詰めてから、右に寄せる
            let x: i32 = (next >> 3) as i32;
            let y: i32 = (next & 0b111) as i32;
            println!("{}: {}", max_key, max_val);
            println!("next, x:{}, y:{}", x, y);
            let bit = coordinate_to_bit(x, y);
            /*
            let m = bit_to_move(bit);
            println!("move:{}", move_to_string(&m));*/
            return bit;
        }
    } else {
        // 最小を求める
        // 黒にとっての評価なので、最大を求める
        let mut min_key: u128 = 0;
        let mut min_val: f32 = std::f32::MAX;
        for (&key, &value) in book.range((Included(hist), Included(last))) {
            if min_val > value {
                min_key = key;
                min_val = value;
            }
        }
        if min_key == 0 {
            println!("no hit");
            return 0;
        } else {
            let next = (min_key << ((60 - count - 1) * 6 + 2)) >> 122; // 必要な6bitを左に詰めてから、右に寄せる
            let x: i32 = (next >> 3) as i32;
            let y: i32 = (next & 0b111) as i32;
            println!("{}: {}", min_key, min_val);
            println!("next, x:{}, y:{}", x, y);
            let bit = coordinate_to_bit(x, y);
            /*
            let m = bit_to_move(bit);
            println!("move:{}", move_to_string(&m));*/

            return bit;
        }
    }
}
