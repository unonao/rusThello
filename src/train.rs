/*
    10のステージに分けた評価関数を作成
    ../trian/ ディレクトリに、データを作る

*/
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Index {
    diag4: Vec<f32>,
    diag5: Vec<f32>,
    diag6: Vec<f32>,
    diag7: Vec<f32>,
    diag8: Vec<f32>,
    hv2: Vec<f32>,
    hv3: Vec<f32>,
    hv4: Vec<f32>,
    edge2x: Vec<f32>,
    cor25h: Vec<f32>,
    cor25v: Vec<f32>,
    cor33: Vec<f32>,
}

pub fn make_init_index() -> Index {
    Index {
        diag4: vec![0.0; 3 * 3 * 3 * 3],
        diag5: vec![0.0; 3 * 3 * 3 * 3 * 3],
        diag6: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3],
        diag7: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3 * 3],
        diag8: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3],
        hv2: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3],
        hv3: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3],
        hv4: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3],
        edge2x: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3],
        cor25h: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3],
        cor25v: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3],
        cor33: vec![0.0; 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3],
    }
}

pub fn train() {
    let mut index = make_init_index();
    for stage in 1..11 {
        println!("start stage{}", stage);
        index = make_model(&stage, &index);
        save_index(index, stage);
        println!("saved stage{}", stage);
    }
}

fn make_model(stage: &i32, index: &Index) {
    for count in 1..7 {
        for result in
            BufReader::new(File::open(format!("./train/train{}.txt", count * stage - 1)).unwrap())
                .lines()
        {
            match result {
                Ok(n) => {
                    let mut iter = n.split_whitespace();
                    let next: u64 = iter.next().unwrap().parse().unwrap();
                    let pre: u64 = iter.next().unwrap().parse().unwrap();
                    let def: i32 = iter.next().unwrap().parse().unwrap();
                    let next_mobility_num: i32 = iter.next().unwrap().parse().unwrap();
                }
                Err(error) => println!("error: {}", error),
            }
        }
    }
}

pub fn board_to_bitvec(vec: &mut Vec<f64>, next: u64, pre: u64) {
    let mut mask: u64 = 0x8000000000000000;
    for _i in 0..64 {
        if (next & mask) == mask {
            vec.push(1.0);
        } else {
            vec.push(0.0);
        }

        if (pre & mask) == mask {
            vec.push(1.0);
        } else {
            vec.push(0.0);
        }
        mask = mask >> 1;
    }
}
