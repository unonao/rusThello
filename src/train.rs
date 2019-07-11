/*
    10のステージに分けた評価関数を作成
    ../trian/ ディレクトリに、データを作る

*/
use crate::eval_fun::*;
use crate::global::*;
use crate::rotate::*;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
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
        def: 0.0,
        next_mobility_num_black: 0.0,
        next_mobility_num_white: 0.0,
    }
}

pub fn train() {
    for stage in 1..13 {
        // 12のステージ
        let mut index = make_init_index();
        println!("start stage{}", stage);
        make_model(&stage, &mut index);
        //save_index(index, stage);
        println!("saved stage{}", stage);
    }
}
pub fn train_continue() {
    //nohup cargo run --release -- --cntntrain -S 1 -E 4 --iter 3000> log/cntn1.log &
    //nohup cargo run --release -- --cntntrain -S 4 -E 7 --iter 3000> log/cntn2.log &
    //nohup cargo run --release -- --cntntrain -S 7 -E 10 --iter 3000> log/cntn3.log &
    //nohup cargo run --release -- --cntntrain -S 10 -E 13 --iter 3000> log/cntn4.log &
    for stage in ARGS.sttrain..ARGS.endtrain {
        let mut file = File::open(format!("./model/stage{}.txt", stage)).unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();
        let mut z = ZlibDecoder::new(&buf[..]);
        let mut s = String::new();
        z.read_to_string(&mut s).unwrap();
        let mut index: Index = serde_json::from_str(&s).unwrap();
        println!("start stage{}", stage);
        make_model(&stage, &mut index);
        //save_index(index, stage);
        println!("saved stage{}", stage);
    }
}

fn make_model(stage: &i32, mut index: &mut Index) {
    for iter in 0..ARGS.iter {
        let mut d_all = make_init_index();
        let mut count = make_init_index();
        let mut sum_e = 0.0;
        let mut data_count = 0.0;
        for file_count in 1..6 {
            // for all i
            // 1ステージ5ファイル

            for result in BufReader::new(
                File::open(format!("./train/train{}.txt", file_count * stage - 1)).unwrap(),
            )
            .lines()
            {
                match result {
                    Ok(n) => {
                        data_count += 1.0;
                        let mut iter = n.split_whitespace();
                        let black: u64 = iter.next().unwrap().parse().unwrap();
                        let white: u64 = iter.next().unwrap().parse().unwrap();
                        let def: f32 = iter.next().unwrap().parse().unwrap();
                        let next_mobility_num_black: f32 = iter.next().unwrap().parse().unwrap();
                        let next_mobility_num_white: f32 = iter.next().unwrap().parse().unwrap();
                        let result: f32 = iter.next().unwrap().parse().unwrap();

                        sum_e += update_d_all(
                            &black,
                            &white,
                            &def,
                            &next_mobility_num_black,
                            &next_mobility_num_white,
                            &result,
                            &mut index,
                            &mut d_all,
                            &mut count,
                        );
                    }
                    Err(error) => println!("error: {}", error),
                }
            }
        } // end for all i
        let e = sum_e / data_count;
        println!("iter :{}, e:{}", iter, e);

        update_index(&mut d_all, &mut index, &mut count);
        if e < 2.0 {
            break;
        };
    }
    match serde_json::to_string(&index) {
        Ok(json_string) => {
            let mut f = BufWriter::new(File::create(format!("model/stage{}.txt", stage)).unwrap());
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            let json_str: &str = &json_string;
            let mut file =
                BufWriter::new(File::create(format!("model_raw/stage{}.txt", stage)).unwrap());
            write!(file, "{}", json_str);
            e.write_all(json_str.as_bytes()).unwrap();
            let compressed_bytes = e.finish().unwrap();
            f.write(&compressed_bytes).unwrap();
        }
        Err(error) => println!("error: {}", error),
    }
}

fn update_index(d_all: &mut Index, mut index: &mut Index, count: &mut Index) {
    let beta = ARGS.beta;
    let tmp1 = beta / 50.0;

    for i in 0..(3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag4[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.diag4[i];
        index.diag4[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag5[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.diag5[i];
        index.diag5[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag6[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.diag6[i];
        index.diag6[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag7[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.diag7[i];
        index.diag7[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag8[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.diag8[i];
        index.diag8[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.hv2[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.hv2[i];
        index.hv2[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.hv3[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.hv3[i];
        index.hv3[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.hv4[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.hv4[i];
        index.hv4[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.edge2x[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.edge2x[i];
        index.edge2x[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.cor25h[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.cor25h[i];
        index.cor25h[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.cor25v[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.cor25v[i];
        index.cor25v[i] += tmp;
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.cor33[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        let tmp = alpha * d_all.cor33[i];
        index.cor33[i] += tmp;
    }

    let alpha = beta / 110000.0 * 5.0;
    index.def += alpha * d_all.def;
    index.next_mobility_num_black += alpha * d_all.next_mobility_num_black;
    index.next_mobility_num_white += alpha * d_all.next_mobility_num_white;
}

fn update_d_all(
    next_ori: &u64,
    pre_ori: &u64,
    def: &f32,
    next_mobility_num_black: &f32,
    next_mobility_num_white: &f32,
    result: &f32,
    mut index: &mut Index,
    mut d_all: &mut Index,
    count: &mut Index,
) -> f32 {
    let e = result
        - eval_by_model(
            next_ori,
            pre_ori,
            def,
            next_mobility_num_black,
            next_mobility_num_white,
            &mut index,
        );

    d_all.def += e * def;
    d_all.next_mobility_num_black += e * next_mobility_num_black;
    d_all.next_mobility_num_white += e * next_mobility_num_white;
    let next = next_ori;
    let pre = pre_ori;

    let diag4 = board_to_diag4(&next, &pre);
    d_all.diag4[diag4] += e;
    count.diag4[diag4] += 1.0;
    let diag5 = board_to_diag5(&next, &pre);
    d_all.diag5[diag5] += e;
    count.diag5[diag5] += 1.0;
    let diag6 = board_to_diag6(&next, &pre);
    d_all.diag6[diag6] += e;
    count.diag6[diag6] += 1.0;
    let diag7 = board_to_diag7(&next, &pre);
    d_all.diag7[diag7] += e;
    count.diag7[diag7] += 1.0;
    let diag8 = board_to_diag8(&next, &pre);
    d_all.diag8[diag8] += e;
    count.diag8[diag8] += 1.0;
    let hv2 = board_to_hv2(&next, &pre);
    d_all.hv2[hv2] += e;
    count.hv2[hv2] += 1.0;
    let hv3 = board_to_hv3(&next, &pre);
    d_all.hv3[hv3] += e;
    count.hv3[hv3] += 1.0;
    let hv4 = board_to_hv4(&next, &pre);
    d_all.hv4[hv4] += e;
    count.hv4[hv4] += 1.0;
    let edge2x = board_to_edge2x(&next, &pre);
    d_all.edge2x[edge2x] += e;
    count.edge2x[edge2x] += 1.0;
    let cor25h = board_to_cor25h(&next, &pre);
    d_all.cor25h[cor25h] += e;
    count.cor25h[cor25h] += 1.0;
    let cor25v = board_to_cor25v(&next, &pre);
    d_all.cor25v[cor25v] += e;
    count.cor25v[cor25v] += 1.0;
    let cor33 = board_to_cor33(&next, &pre);
    d_all.cor33[cor33] += e;
    count.cor33[cor33] += 1.0;

    let next: u64 = flip_diag_a1h8(next_ori);
    let pre: u64 = flip_diag_a1h8(pre_ori);
    let diag4 = board_to_diag4(&next, &pre);
    d_all.diag4[diag4] += e;
    count.diag4[diag4] += 1.0;
    let diag5 = board_to_diag5(&next, &pre);
    d_all.diag5[diag5] += e;
    count.diag5[diag5] += 1.0;
    let diag6 = board_to_diag6(&next, &pre);
    d_all.diag6[diag6] += e;
    count.diag6[diag6] += 1.0;
    let diag7 = board_to_diag7(&next, &pre);
    d_all.diag7[diag7] += e;
    count.diag7[diag7] += 1.0;
    //let diag8 = board_to_diag8(&next, &pre);
    //d_all.diag8[diag8] += e;
    let hv2 = board_to_hv2(&next, &pre);
    d_all.hv2[hv2] += e;
    count.hv2[hv2] += 1.0;
    let hv3 = board_to_hv3(&next, &pre);
    d_all.hv3[hv3] += e;
    count.hv3[hv3] += 1.0;
    let hv4 = board_to_hv4(&next, &pre);
    d_all.hv4[hv4] += e;
    count.hv4[hv4] += 1.0;
    let edge2x = board_to_edge2x(&next, &pre);
    d_all.edge2x[edge2x] += e;
    count.edge2x[edge2x] += 1.0;
    let cor25h = board_to_cor25h(&next, &pre);
    d_all.cor25h[cor25h] += e;
    count.diag8[diag8] += 1.0;
    let cor25v = board_to_cor25v(&next, &pre);
    d_all.cor25v[cor25v] += e;
    count.cor25v[cor25v] += 1.0;
    let cor33 = board_to_cor33(&next, &pre);
    d_all.cor33[cor33] += e;
    count.cor33[cor33] += 1.0;

    let next: u64 = rotate180(&flip_diag_a1h8(next_ori));
    let pre: u64 = rotate180(&flip_diag_a1h8(pre_ori));
    let diag4 = board_to_diag4(&next, &pre);
    d_all.diag4[diag4] += e;
    count.diag4[diag4] += 1.0;
    let diag5 = board_to_diag5(&next, &pre);
    d_all.diag5[diag5] += e;
    count.diag5[diag5] += 1.0;
    let diag6 = board_to_diag6(&next, &pre);
    d_all.diag6[diag6] += e;
    count.diag6[diag6] += 1.0;
    let diag7 = board_to_diag7(&next, &pre);
    d_all.diag7[diag7] += e;
    count.diag7[diag7] += 1.0;
    //let diag8 = board_to_diag8(&next, &pre);
    //d_all.diag8[diag8] += e;
    let hv2 = board_to_hv2(&next, &pre);
    d_all.hv2[hv2] += e;
    count.hv2[hv2] += 1.0;
    let hv3 = board_to_hv3(&next, &pre);
    d_all.hv3[hv3] += e;
    count.hv3[hv3] += 1.0;
    let hv4 = board_to_hv4(&next, &pre);
    d_all.hv4[hv4] += e;
    count.hv4[hv4] += 1.0;
    let edge2x = board_to_edge2x(&next, &pre);
    d_all.edge2x[edge2x] += e;
    count.edge2x[edge2x] += 1.0;
    let cor25h = board_to_cor25h(&next, &pre);
    d_all.cor25h[cor25h] += e;
    count.cor25h[cor25h] += 1.0;
    let cor25v = board_to_cor25v(&next, &pre);
    d_all.cor25v[cor25v] += e;
    count.cor25v[cor25v] += 1.0;
    let cor33 = board_to_cor33(&next, &pre);
    d_all.cor33[cor33] += e;
    count.cor33[cor33] += 1.0;

    let next: u64 = rotate180(next_ori);
    let pre: u64 = rotate180(pre_ori);
    let diag4 = board_to_diag4(&next, &pre);
    d_all.diag4[diag4] += e;
    count.diag4[diag4] += 1.0;
    let diag5 = board_to_diag5(&next, &pre);
    d_all.diag5[diag5] += e;
    count.diag5[diag5] += 1.0;
    let diag6 = board_to_diag6(&next, &pre);
    d_all.diag6[diag6] += e;
    count.diag6[diag6] += 1.0;
    let diag7 = board_to_diag7(&next, &pre);
    d_all.diag7[diag7] += e;
    count.diag7[diag7] += 1.0;
    let diag8 = board_to_diag8(&next, &pre);
    d_all.diag8[diag8] += e;
    count.diag8[diag8] += 1.0;
    let hv2 = board_to_hv2(&next, &pre);
    d_all.hv2[hv2] += e;
    count.hv2[hv2] += 1.0;
    let hv3 = board_to_hv3(&next, &pre);
    d_all.hv3[hv3] += e;
    count.hv3[hv3] += 1.0;
    let hv4 = board_to_hv4(&next, &pre);
    d_all.hv4[hv4] += e;
    count.hv4[hv4] += 1.0;
    let edge2x = board_to_edge2x(&next, &pre);
    d_all.edge2x[edge2x] += e;
    count.edge2x[edge2x] += 1.0;
    let cor25h = board_to_cor25h(&next, &pre);
    d_all.cor25h[cor25h] += e;
    count.cor25h[cor25h] += 1.0;
    let cor25v = board_to_cor25v(&next, &pre);
    d_all.cor25v[cor25v] += e;
    count.cor25v[cor25v] += 1.0;
    let cor33 = board_to_cor33(&next, &pre);
    d_all.cor33[cor33] += e;
    count.cor33[cor33] += 1.0;

    return e * e;
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
