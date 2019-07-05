/*
    オープニングブック用
*/
use crate::color::*;
use crate::global::*;
use crate::play::*;
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

pub fn book_serch(hist: u128, color: i32, count: i32) -> u64 {
    // passでないときに呼び出す
    println!("book search!");
    let mask: u128 = std::u128::MAX;
    let last = hist | (mask >> ((60 - count) * 6) + 2); // はじめの1手目でcount = 59なら、maskを1*6+2ずらす
    println!("last:{}", last);
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
            /*let m = bit_to_move(bit);
            println!("move:{}", move_to_string(&m));
            */
            return bit;
        }
    }
}
