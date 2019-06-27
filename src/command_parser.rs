
#[macro_use]
use nom::{
  IResult,
  take,
  character::streaming::{alphanumeric1,alpha1,digit1,char},
  bytes::complete::{tag,take_until,take},
  combinator::{map_res},
  sequence::{terminated,tuple}
  };

pub enum Message {
    Open{name:String},
    Start{color:String, name:String, time:u32},
    End {win_lose:String, n:u32,  m:u32, reason:String},
    Bye{stats:Vec<stat>},
    Move {x:u32, y:u32}, 
    Pass,
    Giveup,
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
        "START" => return start_parse(input),
        "END" => return end_parse(input),
        "BYE" => return bye_parse(input),
        "MOVE" => return move_parse(input),
        /*
        
        "ACK" => return ack_parse(input)
        */
        _ => panic!("crash and burn")
    }
}


pub fn open_parse(input: &str) -> IResult<&str,Message> {
        let (input, (_, name, _)) = tuple((tag(" "), alphanumeric1, tag("\n")))(input)?;
        let name_string = name.to_string();
        Ok((input ,Message::Open{name :name_string}))
}
pub fn start_parse(input: &str) -> IResult<&str,Message> {
        let (input, (_, color, _,  name, _, time, _)) = tuple((tag(" "), alpha1 ,tag(" "), alphanumeric1, tag(" "), digit1, tag("\n")))(input)?;
        let time_num:u32 = time.parse().unwrap();
        Ok((input ,Message::Start{color: color.to_string(), name :name.to_string(), time:time_num}))
}
pub fn end_parse(input: &str) -> IResult<&str,Message> {
        let (input, (_, win_lose, _,  n, _, m, _, reason, _)) = tuple((tag(" "), alpha1 ,tag(" "), digit1, tag(" "), digit1, tag(" "), alphanumeric1, tag("\n")))(input)?;
        let n_num:u32 = n.parse().unwrap();
        let m_num:u32 = m.parse().unwrap();
        Ok((input ,Message::End{win_lose: win_lose.to_string(), n :n_num, m :m_num, reason:reason.to_string()}))
}
pub fn bye_parse(input: &str) -> IResult<&str,Message> {
        let mut stat_vec: Vec<stat> = Vec::new();
        let mut input_tmp = input;
        while true {
                let (input, _) = tag(" ")(input_tmp)?;
                let (input, (participant, _,  score, _, win, _, lose)) = tuple((alphanumeric1 ,tag(" "), digit1, tag(" "), digit1, tag(" "), digit1))(input)?;

                let score:u32 = score.parse().unwrap();
                let win:u32 = win.parse().unwrap();
                let lose:u32 = lose.parse().unwrap();
                let one_stat:stat = stat{participant:participant.to_string(), score:score, win:win, lose:lose};
                stat_vec.push(one_stat);

                if input=="\n" {break} // 終端に来たらbreak
                input_tmp = input;
                println!("bye2");
        }
        Ok((input ,Message::Bye{stats:stat_vec}))
}

pub fn move_parse(input: &str) -> IResult<&str,Message> { // Move{x:u32, y:u32}
        let (input, (_, m,_)) = tuple((tag(" "), alphanumeric1, tag("\n")))(input)?;
        println!("{}",input);
        //let (_, (x, y)) = tuple((nom::character::complete::char, nom::character::complete::char))(input)?;
        match m {
                "PASS"          => return Ok((input ,Message::Pass)),
                "GIVEUP"        => return Ok((input ,Message::Giveup)),
                _               => sub_move_parse(m)
                                   
        }
}
pub fn sub_move_parse(input: &str) -> IResult<&str,Message> { 
        let (input,x) = take(1u8)(input)?;
        println!("{}",x);
        let (input,y) = take(1u8)(input)?;
        println!("{}",y);
        let x:u32 = ((((x.chars().next().unwrap() as u32) - ('A' as u32)) as u32)) + 1;
        let y:u32 = ((y.chars().next().unwrap() as u32) - ('0' as u32)) as u32;
        return Ok((input ,Message::Move{x:x,y:y}))
}