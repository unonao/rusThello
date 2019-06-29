/*

    think.rs: 思考ルーチン用ファイル

*/
//use crate::play::*;
//use crate::color::*;

pub fn get_first_legal(legal:u64)->u64{
    // 最初の着手可能場所を取得(単純な思考ルーチン)
    let mut mask:u64 = 0x8000000000000000;
    if legal == 0 {
        return 0;
    }else{
        for _i in 0..64 {
            if (mask&legal)==mask {
                return mask
            }
            mask = mask >> 1;
        }
    }
    return mask
}