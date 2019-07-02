/*
    共通の定数や変数などをまとめたファイル
*/
use std::collections::HashMap;
use std::sync::RwLock;

pub static Thinker: &str = "rusThello";

lazy_static! {
    /*
    pub static ref Thinker: RwLock<&'static str> = {
        RwLock::new("rusThello")
    };*/

    pub static ref Rand_mask :RwLock<[[[u64; 256];8];2]> = RwLock::new([[[0u64; 256];8];2]); // zobrist hash 用の乱数
    pub static ref Map_mut: RwLock<HashMap<u64, i32>> = {
        RwLock::new(HashMap::new())
    };
}

pub const INFINITY: i32 = 1 << 30;

// for solver
pub const SOLVE_START: i32 = 18;
pub const SOLVE_SORT_END: i32 = 5;
pub const HASH_DEPTH: i32 = 0;

// for think
pub const THINE_DEPTH: i32 = 2;
