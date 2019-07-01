/*

    print.rs : print系の出力用関数を集めたファイル

*/

use crate::command_parser::*;
use crate::play::*;

pub fn print_stats(stats: Vec<Stat>) {
    /*
        Vec<Stat>を受け取って、内容をすべて出力
    */
    for i in stats {
        println!(
            "participant:{} score:{}, win:{}, lose:{}",
            i.participant, i.score, i.win, i.lose
        )
    }
}

pub fn print_unilateral(flippable: &u64) {
    /*
        白と黒の片側ボード or 反転可能位置 or 着手位置 or 反転する石の位置 などを出力
    */
    let mut mask: u64 = 0x8000000000000000;
    println!(" ABCDEFGH");
    for n in 0..8 {
        if (mask & flippable) != mask {
            print!("{}.", n + 1)
        } else {
            print!("{}1", n + 1)
        };
        mask = mask >> 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask >> 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask >> 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask >> 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask >> 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask >> 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask >> 1;
        if (mask & flippable) != mask {
            println!(".")
        } else {
            println!("1")
        };
        mask = mask >> 1;
    }
    println!("");
}

pub fn print_board(board: &Board) {
    /*
        ボード状況を出力
        黒や白の石の合計数も出力
    */
    let (black, white) = (board.black, board.white);
    let black_num = black.count_ones();
    let white_num = white.count_ones();

    let blank = !(black | white);
    let mut mask: u64 = 0x8000000000000000;
    println!(
        " Board ( 1:Black {}({}), 0:White {}({}))",
        black_num, black, white_num, white
    );
    println!(" ABCDEFGH");
    for n in 0..8 {
        let i = 8 - n;
        if (mask & blank) == mask {
            print!("{}.", n + 1)
        } else {
            print!("{}{:01b}", n + 1, (mask & black) >> i * 8 - 1)
        };
        mask = mask >> 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> i * 8 - 2)
        };
        mask = mask >> 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> i * 8 - 3)
        };
        mask = mask >> 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> i * 8 - 4)
        };
        mask = mask >> 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> i * 8 - 5)
        };
        mask = mask >> 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> i * 8 - 6)
        };
        mask = mask >> 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> i * 8 - 7)
        };
        mask = mask >> 1;
        if (mask & blank) == mask {
            println!(".")
        } else {
            println!("{:01b}", (mask & black) >> i * 8 - 8)
        };
        mask = mask >> 1;
    }
    println!("");
}

pub fn print_unilateral_rev(flippable: &u64) {
    /*
        白と黒の片側ボード or 反転可能位置 or 着手位置 or 反転する石の位置 などを出力
    */
    let mut mask: u64 = 0x0000000000000001;
    println!(" 01234567");
    for n in 0..8 {
        if (mask & flippable) != mask {
            print!("{}.", n)
        } else {
            print!("{}1", n)
        };
        mask = mask << 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask << 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask << 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask << 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask << 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask << 1;
        if (mask & flippable) != mask {
            print!(".")
        } else {
            print!("1")
        };
        mask = mask << 1;
        if (mask & flippable) != mask {
            println!(".")
        } else {
            println!("1")
        };
        mask = mask << 1;
    }
    println!("");
}

pub fn print_board_rev(board: &Board) {
    /*
        ボード状況を出力
        黒や白の石の合計数も出力
    */
    let (black, white) = (board.black, board.white);
    let black_num = black.count_ones();
    let white_num = white.count_ones();

    let blank = !(black | white);
    let mut mask: u64 = 0x0000000000000001;
    println!(
        " Board ( 1:Black {}({}), 0:White {}({}))",
        black_num, black, white_num, white
    );
    println!(" 01234567");
    for n in 0..8 {
        if (mask & blank) == mask {
            print!("{}.", n)
        } else {
            print!("{}{:01b}", n, (mask & black) >> n * 8)
        };
        mask = mask << 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> n * 8 + 1)
        };
        mask = mask << 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> n * 8 + 2)
        };
        mask = mask << 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> n * 8 + 3)
        };
        mask = mask << 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> n * 8 + 4)
        };
        mask = mask << 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> n * 8 + 5)
        };
        mask = mask << 1;
        if (mask & blank) == mask {
            print!(".")
        } else {
            print!("{:01b}", (mask & black) >> n * 8 + 6)
        };
        mask = mask << 1;
        if (mask & blank) == mask {
            println!(".")
        } else {
            println!("{:01b}", (mask & black) >> n * 8 + 7)
        };
        mask = mask << 1;
    }
    println!("");
}
