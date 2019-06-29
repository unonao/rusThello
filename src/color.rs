/*
    color.rs: 黒や白などの、石の色に関するファイル
*/

pub const  BLACK: i32 = 0;
pub const  WHITE: i32 = 1;
pub fn opposite_color(color:i32)->i32{
    /*
        ビット演算でカラーをスワップする
    */
    color ^ (1 as i32)
}
