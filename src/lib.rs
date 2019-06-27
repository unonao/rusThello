extern crate nom;
pub mod play;


pub mod command_nom;


// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let parsed = crate::command_nom::command_parse("OPEN player12345\n");


        match parsed {

            Ok((input, crate::command_nom::Message::Open{name})) => println!("{}",name),

            _ => panic!("crash and burn")

        }
    }
}
