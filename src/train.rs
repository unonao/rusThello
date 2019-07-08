/*
    10のステージに分けた評価関数を作成
    ../trian/ ディレクトリに、データを作る

*/
use crate::eval_fun::*;
use crate::rotate::*;
use std::cmp;
use std::fs::File;
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
        next_mobility_num: 0.0,
    }
}

pub fn train() {
    for stage in 1..15 {
        // 10のステージ
        let mut index = make_init_index();
        println!("start stage{}", stage);
        make_model(&stage, &mut index);
        //save_index(index, stage);
        println!("saved stage{}", stage);
    }
}

fn make_model(stage: &i32, mut index: &mut Index) {
    for iter in 0..40 {
        let mut d_all = make_init_index();
        let mut count = make_init_index();
        for file_count in 1..5 {
            // for all i
            // 1ステージ6ファイル

            for result in BufReader::new(
                File::open(format!("./train/train{}.txt", file_count * stage - 1)).unwrap(),
            )
            .lines()
            {
                match result {
                    Ok(n) => {
                        let mut iter = n.split_whitespace();
                        let next: u64 = iter.next().unwrap().parse().unwrap();
                        let pre: u64 = iter.next().unwrap().parse().unwrap();
                        let def: f32 = iter.next().unwrap().parse().unwrap();
                        let next_mobility_num: f32 = iter.next().unwrap().parse().unwrap();
                        let result: f32 = iter.next().unwrap().parse().unwrap();

                        update_d_all(
                            &next,
                            &pre,
                            &def,
                            &next_mobility_num,
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
        println!("iter :{}", iter);
        update_index(&mut d_all, &mut index, &mut count);
    }
    match serde_json::to_string(&index) {
        Ok(json_str) => {
            let mut f = BufWriter::new(File::create(format!("model/stage{}.txt", stage)).unwrap());
            write!(f, "{}", json_str);
        }
        Err(error) => println!("error: {}", error),
    }
}

fn update_index(d_all: &mut Index, mut index: &mut Index, count: &mut Index) {
    let beta = 0.002;
    let tmp1 = beta / 50.0;

    for i in 0..(3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag4[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.diag4[i] += alpha * d_all.diag4[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag5[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.diag5[i] += alpha * d_all.diag5[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag6[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.diag6[i] += alpha * d_all.diag6[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag7[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.diag7[i] += alpha * d_all.diag7[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.diag8[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.diag8[i] += alpha * d_all.diag8[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.hv2[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.hv2[i] += alpha * d_all.hv2[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.hv3[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.hv3[i] += alpha * d_all.hv3[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.hv4[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.hv4[i] += alpha * d_all.hv4[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.edge2x[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.edge2x[i] += alpha * d_all.edge2x[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.cor25h[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.cor25h[i] += alpha * d_all.cor25h[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.cor25v[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.cor25v[i] += alpha * d_all.cor25v[i];
    }
    for i in 0..(3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3) {
        let tmp2 = beta / (count.cor33[i] + 0.1);
        let alpha = if tmp1 > tmp2 { tmp2 } else { tmp1 };
        index.cor33[i] += alpha * d_all.cor33[i];
    }

    let alpha = beta / 110000.0;
    index.def += alpha * d_all.def;
    index.next_mobility_num += alpha * d_all.next_mobility_num;
}

fn update_d_all(
    next_ori: &u64,
    pre_ori: &u64,
    def: &f32,
    next_mobility_num: &f32,
    result: &f32,
    mut index: &mut Index,
    mut d_all: &mut Index,
    mut count: &mut Index,
) {
    let e = result - eval_by_model(next_ori, pre_ori, def, next_mobility_num, &mut index);

    d_all.def += e * def;
    d_all.next_mobility_num += e * next_mobility_num;

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
