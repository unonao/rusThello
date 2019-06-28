extern crate nom;
pub mod play;
pub mod command_parser;


// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        /*
        println!("{}",crate::play::opposite_color(1));
        println!("{}",crate::play::opposite_color(0));

        let mv = crate::play::Move::Mv{x:4,y:5};
        let mask = crate::play::move_to_bit(&mv);
        crate::play::print_unilateral(&mask);
        println!("{}", crate::play::move_to_string(&mv));
        let (x,y) = crate::play::bit_to_coordinate(mask);
        println!("{},{}",x,y);
        */

        // let parsed = crate::command_parser::command_parse("OPEN player12345\n");
        // let parsed = crate::command_parser::command_parse("START BLACK Player2 500000\n");
        // let parsed = crate::command_parser::command_parse("END WIN 48 16 DOUBLE_PASS\n");
        let parsed = crate::command_parser::command_parse("BYE Player2 4 4 0 rusThello -4 0 4\n");
        // let parsed = crate::command_parser::command_parse("MOVE PASS\n");
        // let parsed = crate::command_parser::command_parse("MOVE GIVEUP\n");
        // let parsed = crate::command_parser::command_parse("MOVE B3\n");
        // let parsed = crate::command_parser::command_parse("ACK 1000\n");
        match parsed {
            Ok((input, crate::command_parser::Message::Open{name})) => println!("OPEN {}",name),
            Ok((input, crate::command_parser::Message::Start{color,name,time})) => println!("START {}:{}:{}",color, name, time),
            Ok((input, crate::command_parser::Message::End{win_lose,n, m, reason})) => println!("END {}:{}:{}:{}",win_lose,n, m, reason),
            Ok((input, crate::command_parser::Message::Bye{stats})) => println!("Bye {}:{}:{}:{} {}:{}:{}:{}",stats[0].participant, stats[0].score, stats[0].win, stats[0].lose, stats[1].participant, stats[1].score, stats[1].win, stats[1].lose),
            Ok((input, crate::command_parser::Message::Pass)) => println!("Pass"),
            Ok((input, crate::command_parser::Message::Move{x,y})) => println!("Move x:{}, y:{}",x,y),
            Ok((input, crate::command_parser::Message::Ack{time})) => println!("Ack time:{}",time),
            _ => panic!("crash and burn")
        }
    }
}
