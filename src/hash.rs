
// 乱数
use rand::prelude::*;

use std::sync::RwLock;
lazy_static! {
    pub static ref Rand_mask :RwLock<[[[u64; 256];8];2]> = RwLock::new([[[0u64; 256];8];2]); // zobrist hash 用の乱数
}


pub fn init_rand_mask(){
        let mut rng = rand::prelude::thread_rng();
        {
            let mut rand_mask = Rand_mask.write().unwrap();
            for i in 0..2 {
                for j in 0..8{
                    for k in 0..256{
                        rand_mask[i][j][k] = rng.gen();
                    }
                }
            }
        }
        let mut rand_mask = Rand_mask.read().unwrap();
        println!("{}", rand_mask[1][3][100])
}
/*

*/