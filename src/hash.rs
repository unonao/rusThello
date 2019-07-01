// 乱数
use rand::Rng;

use std::collections::HashMap;
use std::sync::RwLock;
lazy_static! {
    pub static ref Rand_mask :RwLock<[[[u64; 256];8];2]> = RwLock::new([[[0u64; 256];8];2]); // zobrist hash 用の乱数
    pub static ref Map_mut: RwLock<HashMap<u64, i32>> = {
        RwLock::new(HashMap::new())
    };
}

pub fn init_rand_mask() {
    let mut rng = rand::thread_rng();
    {
        let mut rand_mask = Rand_mask.write().unwrap();

        for i in 0..2 {
            for j in 0..8 {
                /*rng.fill(&mut rand_mask);*/
                for k in 0..256 {
                    rand_mask[i][j][k] = rng.gen();
                    /*if rand_mask[i][j][k] > 0x0000008000000000 {
                        println!("{}", rand_mask[i][j][k])
                    };*/
                }
            }
        }
    }
    let mut rand_mask = Rand_mask.read().unwrap();
    //println!("{}", rand_mask[1][3][100])
}

pub fn make_hash(me: u64, op: u64) -> u64 {
    let mut hasher: u64 = 0;
    let mut rand_mask = Rand_mask.write().unwrap();
    for i in 0..8 {
        hasher ^= rand_mask[0][i][((me >> 8 * i) & 255) as usize];
        hasher ^= rand_mask[1][i][((op >> 8 * i) & 255) as usize];
    }
    return hasher;
}

pub fn init_hashmap() {
    let mut map_mut = Map_mut.write().unwrap();
    *map_mut = HashMap::new();
}
pub fn hash_insert(hasher: u64, result: i32) {
    let mut map_mut = Map_mut.write().unwrap();
    map_mut.insert(hasher, result);
}

/*

*/
