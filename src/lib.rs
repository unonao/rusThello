extern crate nom;
pub mod play;


pub mod command_parser;


// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let parsed = crate::command_parser::command_parse("OPEN player12345\n");
        // let parsed = crate::command_parser::command_parse("START BLACK opponent 100\n");
        // let parsed = crate::command_parser::command_parse("END Win 33 31 Over\n");
        // let parsed = crate::command_parser::command_parse("BYE player1 100 21 22 player2 50 22 21 player2 50 22 21\n");
        // let parsed = crate::command_parser::command_parse("MOVE PASS\n"); 
        // let parsed = crate::command_parser::command_parse("MOVE GIVEUP\n"); 
        // let parsed = crate::command_parser::command_parse("MOVE B3\n"); 
        match parsed {
            Ok((input, crate::command_parser::Message::Open{name})) => println!("OPEN {}",name),
            Ok((input, crate::command_parser::Message::Start{color,name,time})) => println!("START {}:{}:{}",color, name, time),
            Ok((input, crate::command_parser::Message::End{win_lose,n, m, reason})) => println!("END {}:{}:{}:{}",win_lose,n, m, reason),
            Ok((input, crate::command_parser::Message::Bye{stats})) => println!("Bye {}:{}:{}:{} {}:{}:{}:{}",stats[0].participant, stats[0].score, stats[0].win, stats[0].lose, stats[1].participant, stats[1].score, stats[1].win, stats[1].lose),
            Ok((input, crate::command_parser::Message::Pass)) => println!("Pass"),
            Ok((input, crate::command_parser::Message::Move{x,y})) => println!("Move x:{}, y:{}",x,y),
            _ => panic!("crash and burn")
        }
    }
}
