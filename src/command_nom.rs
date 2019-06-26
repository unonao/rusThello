#[macro_use]
extern crate nom;


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


fn command_parse(input: &str) -> IResult<&str, u8> {
    
}
