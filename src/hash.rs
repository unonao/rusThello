/*

    置換表用のhashやhashMapをまとめたファイル

*/
use rand::Rng;
use std::collections::HashMap;

use crate::global::*;

pub fn init_rand_mask() {
    let mut rng = rand::thread_rng();
    {
        let mut rand_mask = RAND_MASK.write().unwrap();
        for i in 0..2 {
            for j in 0..8 {
                for k in 0..256 {
                    rand_mask[i][j][k] = rng.gen();
                }
            }
        }
    }
}

pub fn make_hash(me: u64, op: u64) -> u64 {
    let mut hasher: u64 = 0;
    let rand_mask = RAND_MASK.read().unwrap();
    for i in 0..8 {
        hasher ^= rand_mask[0][i][((me >> 8 * i) & 255) as usize];
        hasher ^= rand_mask[1][i][((op >> 8 * i) & 255) as usize];
    }
    return hasher;
}

pub fn init_hashmap() {
    let mut map_mut = MAP_MUT.write().unwrap();
    *map_mut = HashMap::new();
}
pub fn hash_insert(hasher: u64, result: i32) {
    let mut map_mut = MAP_MUT.write().unwrap();
    map_mut.insert(hasher, result);
}
