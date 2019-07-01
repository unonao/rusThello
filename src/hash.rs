
// 乱数
use rand::prelude::*;

use std::collections::HashMap;
use std::sync::RwLock;
lazy_static! {
    pub static ref Rand_mask :RwLock<[[[u64; 256];8];2]> = RwLock::new([[[0u64; 256];8];2]); // zobrist hash 用の乱数
    pub static ref Map_mut: RwLock<HashMap<u64, i32>> = {
        let mut m:HashMap<u64, i32> = HashMap::new();
        RwLock::new(m)
    };
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

pub fn make_hash(me:u64, op:u64) -> u64{
    let mut hasher:u64 = 0;
    let mut rand_mask = Rand_mask.write().unwrap();
    for i in 0..8{
        hasher ^= rand_mask[0][i][((me>>8*i)&255) as usize];
        hasher ^= rand_mask[1][i][((op>>8*i)&255) as usize];
    }
    return hasher;
}

pub fn hash_insert(hasher:u64, result:i32){
    let mut map_mut = Map_mut.write().unwrap();
    map_mut.insert(hasher, result);
}

/*
pub fn hash_get(hasher:&u64)-> Option<&i32>{
     // get()を使うと、キーが存在する場合はSome(val)を、存在しない場合はNoneを返す
     let mut tmp:Option<&i32>;
    {
        let mut map_mut = Map_mut.read().unwrap();
        tmp = map_mut.get(hasher);
    }
    tmp
}
*/