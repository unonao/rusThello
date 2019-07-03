/*
    評価関数
*/
//use crate::print::*;

pub fn simple_eval(me: u64, op: u64) -> i32 {
    sub_simple_eval(me) - sub_simple_eval(op)
}
pub fn sub_simple_eval(one: u64) -> i32 {
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
    /*
    print_unilateral(&corner);
    print_unilateral(&adj_corner);
    print_unilateral(&adj_corner2);
    print_unilateral(&advantage);
    print_unilateral(&advantage2);
    print_unilateral(&five);
    print_unilateral(&three);
    print_unilateral(&minus_five);
    */
}
