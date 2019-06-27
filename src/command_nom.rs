
#[macro_use]
use nom::{
  IResult,
  character::streaming::alphanumeric0,
  bytes::complete::{tag,take_until},
  combinator::{map_res},
  sequence::{terminated,tuple}};

pub enum Message {
    Open{name:String},
    Start{color:String, name:u32},
    End {win_lose:String, n:u32,  m:u32, reasen:String},
    Bye{stats:Vec<stat>},
    Move{x:u32, y:u32}, // PassやGiveUpに関してはparserで対処
    Ack{time:u32}
}

pub struct stat {
    pub participant:String,
    pub score:u32,
    pub win:u32, // 勝数
    pub lose:u32
}


pub fn command_parse(input: &str) -> IResult<&str,Message> {
    let (input, head) = take_until(" ")(input)?;
    println!("{}",head);
    println!("{}",input);
    match head {
        "OPEN" => return open_parse(input),
        /*
        "START" => return start_parse(input);
        "END" => return end_parse(input);
        "BYE" => return bye_parse(input);
        "MOVE" => return move_parse(input);
        "ACK" => return ack_parse(input);
        */
        _ => panic!("crash and burn")
    }
}


pub fn open_parse(input: &str) -> IResult<&str,Message> {
        println!("open!!");
        let (input, (_, name, _)) = tuple((tag(" "), alphanumeric0, tag("\n")))(input)?;
        let name_string = name.to_string();
        println!("open!!2");
        Ok((input ,Message::Open{name :name_string}))
}
