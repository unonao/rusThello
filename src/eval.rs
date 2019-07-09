/*
    評価関数
*/
use crate::eval_fun::*;
use crate::global::*;
use crate::play::*;

pub fn board_eval(me: u64, op: u64, _count: i32) -> i32 {
    match ARGS.name.as_str() {
        "rusThello" => second_eval(me, op),
        _ => second_eval(me, op),
    }
}
pub fn board_eval_by_model(me: u64, op: u64, count: i32, is_player: bool) -> f32 {
    let stage = (60 - count - 1) / 5;
    //let next_stage = (60 - count) / 5;
    //let pre_stage = (60 - count - 2) / 5;
    if is_player {
        let def = stone_def(me, op) as f32;
        let next_mobilitys_num = mobility_ps(me, op).count_ones() as f32;
        /*
        println!(
            "eval val:{}",
            eval_by_model(&me, &op, &def, &next_mobilitys_num, &MODEL[stage as usize])
        );
        */
        return eval_by_model(&me, &op, &def, &next_mobilitys_num, &MODEL[stage as usize]);
    } else {
        let def = stone_def(op, me) as f32;
        let next_mobilitys_num = mobility_ps(op, me).count_ones() as f32;
        /*
        println!(
            "eval val:{}",
            -eval_by_model(&op, &me, &def, &next_mobilitys_num, &MODEL[stage as usize])
        );
        */
        return -eval_by_model(&op, &me, &def, &next_mobilitys_num, &MODEL[stage as usize]);
    }
}

pub fn eval_use_model(me: u64, op: u64) -> i32 {
    let blank = !(me | op);
    let me_num: i32 = me.count_ones() as i32;
    let op_num: i32 = op.count_ones() as i32;

    if blank.count_ones() > 15 {
        return sub_eval3(me, blank) - sub_eval3(op, blank) - (me_num - op_num) * 5;
    } else {
        // 終盤
        return sub_eval3(me, blank) - sub_eval3(op, blank) + (me_num - op_num) * 10;
    }
}

pub fn sub_eval3(one: u64, blank: u64) -> i32 {
    let corner: u64 = 0x8100000000000081; // 120
    let adj_corner: u64 = 0x4281000000008142; // -20
    let adj_corner2: u64 = 0x0042000000004200; //-40
    let advantage: u64 = 0x2400810000810024; // 20
    let advantage2: u64 = 0x0000240000240000; //15
    let five: u64 = 0x1800008181000018; // 5
    let three: u64 = 0x0000183c3c180000; // 3
    let minus_five: u64 = 0x003c424242423c00; // -5
    if blank.count_ones() > 15 {
        (((corner & one).count_ones() as i32) * 120)
            + (((adj_corner & one).count_ones() as i32) * -20)
            + (((adj_corner2 & one).count_ones() as i32) * -40)
            + (((advantage & one).count_ones() as i32) * 20)
            + (((advantage2 & one).count_ones() as i32) * 15)
            + (((five & one).count_ones() as i32) * 5)
            + (((three & one).count_ones() as i32) * 3)
            + (((minus_five & one).count_ones() as i32) * -5)
    } else {
        (((corner & one).count_ones() as i32) * 50)
            + (((adj_corner & one).count_ones() as i32) * -10)
            + (((adj_corner2 & one).count_ones() as i32) * -20)
            + (((advantage & one).count_ones() as i32) * 20)
            + (((advantage2 & one).count_ones() as i32) * 5)
            + (((five & one).count_ones() as i32) * 5)
            + (((three & one).count_ones() as i32) * 3)
            + (((minus_five & one).count_ones() as i32) * -5)
    }
}

pub fn second_eval(me: u64, op: u64) -> i32 {
    let blank = !(me | op);
    let me_num: i32 = me.count_ones() as i32;
    let op_num: i32 = op.count_ones() as i32;

    if blank.count_ones() > 15 {
        return sub_second_eval(me, blank) - sub_second_eval(op, blank) - (me_num - op_num) * 5;
    } else {
        // 終盤
        return sub_second_eval(me, blank) - sub_second_eval(op, blank) + (me_num - op_num) * 10;
    }
}

pub fn sub_second_eval(one: u64, blank: u64) -> i32 {
    let corner: u64 = 0x8100000000000081; // 120
    let adj_corner: u64 = 0x4281000000008142; // -20
    let adj_corner2: u64 = 0x0042000000004200; //-40
    let advantage: u64 = 0x2400810000810024; // 20
    let advantage2: u64 = 0x0000240000240000; //15
    let five: u64 = 0x1800008181000018; // 5
    let three: u64 = 0x0000183c3c180000; // 3
    let minus_five: u64 = 0x003c424242423c00; // -5
    if blank.count_ones() > 15 {
        (((corner & one).count_ones() as i32) * 120)
            + (((adj_corner & one).count_ones() as i32) * -20)
            + (((adj_corner2 & one).count_ones() as i32) * -40)
            + (((advantage & one).count_ones() as i32) * 20)
            + (((advantage2 & one).count_ones() as i32) * 15)
            + (((five & one).count_ones() as i32) * 5)
            + (((three & one).count_ones() as i32) * 3)
            + (((minus_five & one).count_ones() as i32) * -5)
    } else {
        (((corner & one).count_ones() as i32) * 50)
            + (((adj_corner & one).count_ones() as i32) * -10)
            + (((adj_corner2 & one).count_ones() as i32) * -20)
            + (((advantage & one).count_ones() as i32) * 20)
            + (((advantage2 & one).count_ones() as i32) * 15)
            + (((five & one).count_ones() as i32) * 5)
            + (((three & one).count_ones() as i32) * 3)
            + (((minus_five & one).count_ones() as i32) * -5)
    }
}

pub fn simple_eval(one: u64) -> i32 {
    let corner: u64 = 0x8100000000000081; // 120
    let adj_corner: u64 = 0x4281000000008142; // -20
    let adj_corner2: u64 = 0x0042000000004200; //-40
    let advantage: u64 = 0x2400810000810024; // 20
    let advantage2: u64 = 0x0000240000240000; //15
    let five: u64 = 0x1800008181000018; // 5
    let three: u64 = 0x0000183c3c180000; // 3
    let minus_five: u64 = 0x003c424242423c00; // -5

    return (((corner & one).count_ones() as i32) * 120)
        + (((adj_corner & one).count_ones() as i32) * -20)
        + (((adj_corner2 & one).count_ones() as i32) * -40)
        + (((advantage & one).count_ones() as i32) * 20)
        + (((advantage2 & one).count_ones() as i32) * 15)
        + (((five & one).count_ones() as i32) * 5)
        + (((three & one).count_ones() as i32) * 3)
        + (((minus_five & one).count_ones() as i32) * -5);
}
