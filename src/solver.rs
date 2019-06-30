/*

solver.rs: 終盤ソルバー用のファイル

*/


use crate::play::*;

pub const  SOLVE_COUNT: i32 = 20;

pub struct NextAndFlippable{
    pub next:u64,
    pub player:u64,
    pub opponent:u64,
    pub f_num:i32
}

/**/


pub fn solve(player:u64, opponent:u64, count:i32)->u64{
    /*
    読み切りをして、次の手を返す
    速さ優先探索: 相手が次打てる手が少ないものから探索
    */
    let mobilitys = mobility_ps(player, opponent);
    if mobilitys==0{ // passのとき
        return 0
    }else{
        let mut next_vec: Vec<NextAndFlippable> = Vec::new();
        let mut mask:u64 = 0x8000000000000000;
        while mask>0 {
            if (mask&mobilitys)>0{
                let (next_player, next_opponent) = flip_board(player, opponent, mask);
                let op_flippable_num = mobility_ps(next_opponent, next_player).count_ones() as i32;
                next_vec.push(NextAndFlippable{next:mask, player:next_player,opponent:next_opponent, f_num: op_flippable_num});
            }
            mask = mask>>1;
        }

        next_vec.sort_unstable_by(|a,b| a.f_num.cmp(&b.f_num)); // f_numについて昇順にsort


        for next_and_f in &next_vec {
            let def = rec_solver(next_and_f.player, next_and_f.opponent, false, count-1);
            if def > 0 { // 見つけたら終了
                //println!("solved! def:{}",def);
                return next_and_f.next
            }

        }
        return next_vec[0].next
    }
}


fn rec_solver(player:u64, opponent:u64, is_player:bool, count:i32)->i32{
    /*
    探索をして、石数を返す
    */
    let mut mask:u64 = 0x8000000000000000;
    let mut next_vec: Vec<NextAndFlippable> = Vec::new();

    if is_player{ // 自身の手に関しては、勝利するものが見つかれば終了
        let mobilitys = mobility_ps(player, opponent);

        if mobilitys==0{ // passのとき
            if is_finished(player, opponent){
                return stone_def(player, opponent)
            }else{
                return rec_solver(player, opponent, false, count) // 空きマスはまだあるので、countは減らさずturnを変える
            }
        }else{
            while mask>0 {
                if (mask&mobilitys)>0{
                    let (next_player, next_opponent) = flip_board(player, opponent, mask);
                    let op_flippable_num = mobility_ps(next_opponent, next_player).count_ones() as i32;
                    next_vec.push(NextAndFlippable{next:mask, player:next_player,opponent:next_opponent, f_num: op_flippable_num});
                }
                mask = mask>>1;
            }
        }

        if count > 6{ // 最終6手ほどからは、ソートせずに全探索
            next_vec.sort_unstable_by(|a,b| a.f_num.cmp(&b.f_num)); // f_numについて昇順に(速さ優先探索)
        }

        let mut def = -64;

        // 最終1手のために条件分岐を毎回するのはコストが高い
        /**/if count==2{ // 最終1手(相手)をその場で処理
            let next = mobility_ps(next_vec[0].opponent, next_vec[0].player);
            let (final_opponent, final_player) = flip_board(next_vec[0].opponent, next_vec[0].player, next);
            return stone_def(final_player,final_opponent)
        }else{
            for next_and_f in next_vec {
                def = rec_solver(next_and_f.player, next_and_f.opponent, false, count-1);
                if def > 0 { // 見つけたら終了
                    return def
                }
            }
        }
        return def // 相手が


    }else{ // 相手の手に関しては、すべての手に関して勝利する必要あり
        let mobilitys = mobility_ps(opponent, player);

        if mobilitys==0{ // passのとき
            if is_finished(player, opponent){
                return stone_def(player, opponent)
            }else{
                return rec_solver(player, opponent, !is_player, count) // 空きマスはまだあるので、countは減らさずturnを変える
            }
        }else{
            while mask>0 {
                if (mask&mobilitys)>0{
                    let (next_opponent, next_player) = flip_board( opponent, player, mask);
                    let pl_flippable_num = mobility_ps(next_player, next_opponent).count_ones() as i32;
                    next_vec.push(NextAndFlippable{next:mask, player:next_player,opponent:next_opponent, f_num: pl_flippable_num}); // f_numは使わない
                }
                mask = mask>>1;
            }
        }

        if count > 6{ // 最終6手ほどからは、ソートせずに全探索
            next_vec.sort_unstable_by(|a,b| a.f_num.cmp(&b.f_num)); // f_numについて昇順に(速さ優先探索)
        }

        let mut def = 0;
        if count==2{ // 最終1手(自分)をその場で処理
            let next = mobility_ps(next_vec[0].player, next_vec[0].opponent);
            let (final_player, final_opponent) = flip_board(next_vec[0].player, next_vec[0].opponent, next);
            return stone_def(final_player,final_opponent)
        }else{/**/
            for next_and_f in next_vec {
                def = rec_solver(next_and_f.player, next_and_f.opponent, true, count-1);
                if def < 0 { // 見つけたら終了
                    return def // マイナス値
                }
            }
        }
        return def // プラス
    }


}
