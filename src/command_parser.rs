/*

    command_parser.rs: プロトコルで定められたメッセージをパースする関数や、構造体・列挙型を定義するファイル

    COMMAND Arg1 ... Argn\n  という形式のメッセージをやりとり
        OPEN PLAYER_NAME
        START WB OPPONENT_NAME TIME
        END WL n m REASON
        BYE stat
        MOVE M
        ACK TIME
*/

//#[macro_use]
use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::char,
    character::streaming::{alpha1, alphanumeric1, digit1, not_line_ending},
    combinator::opt,
    sequence::tuple,
    IResult,
};

pub enum Message {
    Open {
        name: String,
    },
    Start {
        color: String,
        name: String,
        time: i32,
    },
    End {
        win_lose: String,
        n: i32,
        m: i32,
        reason: String,
    },
    Bye {
        stats: Vec<Stat>,
    },
    Move {
        x: i32,
        y: i32,
    },
    Pass,
    Giveup,
    Ack {
        time: i32,
    },
}

pub struct Stat {
    pub participant: String,
    pub score: i32,
    pub win: i32, // 勝数
    pub lose: i32,
}

pub fn command_parse(input: &str) -> IResult<&str, Message> {
    let (input, head) = take_until(" ")(input)?;
    match head {
        "OPEN" => return open_parse(input),
        "START" => return start_parse(input),
        "END" => return end_parse(input),
        "BYE" => return bye_parse(input),
        "MOVE" => return move_parse(input),
        "ACK" => return ack_parse(input),
        _ => panic!("crash and burn"),
    }
}

fn open_parse(input: &str) -> IResult<&str, Message> {
    let (input, (_, name, _)) = tuple((tag(" "), alphanumeric1, tag("\n")))(input)?;
    let name_string = name.to_string();
    Ok((input, Message::Open { name: name_string }))
}
fn start_parse(input: &str) -> IResult<&str, Message> {
    let (input, (_, color, _, name, _, time, _)) = tuple((
        tag(" "),
        alpha1,
        tag(" "),
        alphanumeric1,
        tag(" "),
        digit1,
        tag("\n"),
    ))(input)?;
    let time_num: i32 = time.parse().unwrap();
    Ok((
        input,
        Message::Start {
            color: color.to_string(),
            name: name.to_string(),
            time: time_num,
        },
    ))
}
fn end_parse(input: &str) -> IResult<&str, Message> {
    let (input, (_, win_lose, _, n, _, m, _, reason, _)) = tuple((
        tag(" "),
        alpha1,
        tag(" "),
        digit1,
        tag(" "),
        digit1,
        tag(" "),
        not_line_ending,
        tag("\n"),
    ))(input)?;
    let n_num: i32 = n.parse().unwrap();
    let m_num: i32 = m.parse().unwrap();
    Ok((
        input,
        Message::End {
            win_lose: win_lose.to_string(),
            n: n_num,
            m: m_num,
            reason: reason.to_string(),
        },
    ))
}
fn bye_parse(input: &str) -> IResult<&str, Message> {
    let mut stat_vec: Vec<Stat> = Vec::new();
    let mut input_tmp = input;
    loop {
        let (input, _) = tag(" ")(input_tmp)?;
        let (input, (participant, _, minus, score, _, win, _, lose)) = tuple((
            alphanumeric1,
            tag(" "),
            opt(char('-')),
            digit1,
            tag(" "),
            digit1,
            tag(" "),
            digit1,
        ))(input)?;
        let score: i32 = score.parse().unwrap();
        let score: i32 = if minus.is_some() { -1 * score } else { score };
        let win: i32 = win.parse().unwrap();
        let lose: i32 = lose.parse().unwrap();
        let one_stat: Stat = Stat {
            participant: participant.to_string(),
            score: score,
            win: win,
            lose: lose,
        };
        stat_vec.push(one_stat);

        if input == "\n" {
            break;
        } // 終端に来たらbreak
        input_tmp = input;
    }
    Ok((input, Message::Bye { stats: stat_vec }))
}

fn move_parse(input: &str) -> IResult<&str, Message> {
    // Move{x:i32, y:i32}
    let (input, (_, m, _)) = tuple((tag(" "), alphanumeric1, tag("\n")))(input)?;
    //let (_, (x, y)) = tuple((nom::character::complete::char, nom::character::complete::char))(input)?;
    match m {
        "PASS" => return Ok((input, Message::Pass)),
        "GIVEUP" => return Ok((input, Message::Giveup)),
        _ => sub_move_parse(m),
    }
}
fn sub_move_parse(input: &str) -> IResult<&str, Message> {
    let (input, x) = take(1u8)(input)?;
    let (input, y) = take(1u8)(input)?;
    let x: i32 = ((x.chars().next().unwrap() as i32) - ('A' as i32)) as i32;
    let y: i32 = ((y.chars().next().unwrap() as i32) - ('1' as i32)) as i32;
    return Ok((input, Message::Move { x: x, y: y }));
}

fn ack_parse(input: &str) -> IResult<&str, Message> {
    //Ack{time:i32}
    let (input, (_, time, _)) = tuple((tag(" "), digit1, tag("\n")))(input)?;
    let time_num: i32 = time.parse().unwrap();
    Ok((input, Message::Ack { time: time_num }))
}
