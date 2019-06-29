/*
    color.rs: 色に関するファイル
*/

pub const  BLACK: u32 = 0;
pub const  WHITE: u32 = 1;
pub fn opposite_color(color:u32)->u32{
    /*
        ビット演算でカラーをスワップする
    */
    color ^ (1 as u32)
}
