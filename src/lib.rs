extern crate nom;
extern crate rand;
#[macro_use]
extern crate lazy_static;

pub mod color;
pub mod command_parser;
pub mod hash;
pub mod play;
pub mod print;
pub mod solver;
pub mod think;

// cargo test -- --nocapture
#[cfg(test)]
mod tests {

    use crate::play::*;
    use crate::print::*;
    use crate::solver::*;
    #[test]
    fn it_works() {
        let board = Board {
            black: 0b0000000100000001001100010010001100000011000000010000000100000000,
            white: 0b1001111001111110110011101101110011111100000111100000100000000000,
        };

        print_board(&board);
        print_unilateral(&solve(
            0b0000000100000001001100010010001100000011000000010000000100000000,
            0b1001111001111110110011101101110011111100000111100000100000000000,
            20,
        ))

        /*
        // let parsed = crate::command_parser::command_parse("OPEN player12345\n");
        // let parsed = crate::command_parser::command_parse("START BLACK Player2 500000\n");
        // let parsed = crate::command_parser::command_parse("END WIN 48 16 DOUBLE_PASS\n");
        let parsed = crate::command_parser::command_parse("BYE Player2 4 4 0 rusThello -4 0 4\n");
        // let parsed = crate::command_parser::command_parse("MOVE PASS\n");
        // let parsed = crate::command_parser::command_parse("MOVE GIVEUP\n");
        // let parsed = crate::command_parser::command_parse("MOVE B3\n");
        // let parsed = crate::command_parser::command_parse("ACK 1000\n");
        match parsed {
            Ok((_, crate::command_parser::Message::Open{name})) => println!("OPEN {}",name),
            Ok((_, crate::command_parser::Message::Start{color,name,time})) => println!("START {}:{}:{}",color, name, time),
            Ok((_, crate::command_parser::Message::End{win_lose,n, m, reason})) => println!("END {}:{}:{}:{}",win_lose,n, m, reason),
            Ok((_, crate::command_parser::Message::Bye{stats})) => println!("Bye {}:{}:{}:{} {}:{}:{}:{}",stats[0].participant, stats[0].score, stats[0].win, stats[0].lose, stats[1].participant, stats[1].score, stats[1].win, stats[1].lose),
            Ok((_, crate::command_parser::Message::Pass)) => println!("Pass"),
            Ok((_, crate::command_parser::Message::Move{x,y})) => println!("Move x:{}, y:{}",x,y),
            Ok((_, crate::command_parser::Message::Ack{time})) => println!("Ack time:{}",time),
            _ => panic!("crash and burn")
        }
        */
    }
}
