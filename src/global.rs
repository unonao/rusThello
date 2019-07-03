/*
    共通の定数や変数などをまとめたファイル
*/
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;
pub static mut Thinker: &str = "rusThello";

lazy_static! {
    /*
    pub static ref Thinker: RwLock<&'static str> = {
        RwLock::new("rusThello")
    };*/
    pub static ref Args: Args_st = {
        let args: Vec<String> = env::args().collect();
        let len = args.len();
        let opt_host = if len > 1 { &args[1] } else { "127.0.0.1" };
        let opt_port = if len > 2 { &args[2] } else { "30000" };
        let opt_player_name = if len > 3 { &args[3] } else { "rusThello" };
        let opt_solve_start:i32 = if len > 4 { args[4].parse().unwrap() } else { 18 };
        let think_depth:i32 = if len > 5 { args[5].parse().unwrap() } else { 2 };
        Args_st{
            host:opt_host.to_string(),
            port:opt_port.to_string(),
            name:opt_player_name.to_string(),
            solve_start:opt_solve_start,
            think_depth:think_depth,
        }
    };

    pub static ref Rand_mask :RwLock<[[[u64; 256];8];2]> = RwLock::new([[[0u64; 256];8];2]); // zobrist hash 用の乱数
    pub static ref Map_mut: RwLock<HashMap<u64, i32>> = {
        RwLock::new(HashMap::new())
    };
}

pub struct Args_st {
    pub host: String,
    pub port: String,
    pub name: String,
    pub solve_start: i32,
    pub think_depth: i32,
}

pub const INFINITY: i32 = 1 << 30;

// for solver
pub const SOLVE_SORT_END: i32 = 5;
pub const HASH_DEPTH: i32 = 0;

// for think
pub const THINE_DEPTH: i32 = 2;
