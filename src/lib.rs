extern crate nom;
pub mod play;
pub mod command_parser;
pub mod print;
pub mod think;
pub mod color;
pub mod solver;

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x :u64 = 0x0101010101010100;
        crate::print::print_bit(&x);
        println!("{}", x.leading_zeros());
        /*
        let mut vec : Vec<crate::solver::NextAndFlippable> = Vec::new();
        vec.push(crate::solver::NextAndFlippable{next:1,f_num:100});
        vec.push(crate::solver::NextAndFlippable{next:4,f_num:10});
        vec.push(crate::solver::NextAndFlippable{next:2,f_num:10});
        vec.sort_unstable_by(|a,b| a.f_num.cmp(&b.f_num));
        println!("{}:{}, {}:{}, {}:{}", vec[0].next, vec[0].f_num,vec[1].next, vec[1].f_num,vec[2].next, vec[2].f_num)

        Board ( 1:Black 35(9187071716383739772), 0:White 28(9259672357325811842))
            ABCDEFGH
            101111111
            201111111
            300001001
            400001101
            500000101
            600010111
            701001111
            80111110.
        */


        /*

        println!("is_finished {}", crate::play::Board{black:9187071716383739772,white:9259672357325811842}.is_finished());        println!("is_finished {}", crate::play::Board{black:9187071716383477116,white:9259672357326074499}.is_finished());
        println!("is_win {}", crate::play::Board{black:9187071716383477116,white:9259672357326074499}.is_win(crate::color::BLACK));
        println!("{}",crate::play::opposite_color(1));
        println!("{}",crate::play::opposite_color(0));

        let mv = crate::play::Move::Mv{x:3,y:5};
        let mask = crate::play::move_to_bit(&mv);
        crate::print::print_unilateral(&mask);
        println!("{}", crate::play::move_to_string(&mv));
        let (x,y) = crate::play::bit_to_coordinate(mask);
        println!("{},{}",x,y);



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
