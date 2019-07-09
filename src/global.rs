/*
    共通の定数や変数などをまとめたファイル
*/
use clap::{App, Arg, ArgGroup};
use std::collections::BTreeMap;
use std::collections::HashMap;
//use std::env;
use std::sync::RwLock;

pub static mut MIRROR_NUM: i32 = 0;
pub static mut HIST: u128 = 0;

lazy_static! {
    pub static ref ARGS: ArgsSt = {

        let app = App::new(crate_name!())   // Cargo.tomlのnameを参照する
            .version(crate_version!())      // Cargo.tomlのversionを参照する
            .author(crate_authors!())       // Cargo.tomlのauthorsを参照する
            .about(crate_description!())    // Cargo.tomlのdescriptionを参照する
            .arg(Arg::from_usage("[eval] -e --eval 'eval thinker'"))
            .arg(Arg::from_usage("[nobook] --nobook 'do not use openning book'"))
            .arg(Arg::from_usage("[mktrain] --mktrain 'make train data'"))
            .arg(Arg::from_usage("[dotrain] --dotrain 'do train'"))
            .arg(Arg::from_usage("[cntntrain] --cntntrain 'continue train'"))
            .arg(Arg::from_usage("-H --host [HOST] 'host ip address'").default_value("127.0.0.1"))
            .arg(Arg::from_usage("-p --port [PORT] 'port number'").default_value("3000"))
            .arg(Arg::from_usage("-n --name [NAME] 'player name'").default_value("rusThello"))
            .arg(Arg::from_usage("-s --solve [SOLVE] 'start solver depth'").default_value("12"))
            .arg(Arg::from_usage("-S --sttrain [STTRAIN] 'start train'").default_value("2"))
            .arg(Arg::from_usage("-E --endtrain [ENDTRAIN] 'end train'").default_value("13"))
            .arg(Arg::from_usage("-B --beta [BETA] 'train beta'").default_value("0.0004"))
            .arg(Arg::from_usage("-I --iter [ITER] 'train iterate num'").default_value("100"))
            .arg(Arg::from_usage("-t --think [THINK] 'think depth'").default_value("3"))
            .args_from_usage("--verb 'verbose mode: level group'
                                --debug 'debug mode: level group'
                                --info 'info mode: level group'")
            .group(ArgGroup::with_name("level") // グループ名
                .args(&["verb", "debug", "info"])
            );
        let matches = app.get_matches();// 引数を解析
        if matches.is_present("level") {
            let (verb, debug, _) = (matches.is_present("verb"),
                                    matches.is_present("debug"),
                                    matches.is_present("info"));
            println!("level is {}", if verb {"verb"} else if debug {"debug"} else {"info"});
        }else{
            println!("level is info");
        }

        ArgsSt{
            host:matches.value_of("host").unwrap().to_string(),
            port:matches.value_of("port").unwrap().to_string(),
            name:matches.value_of("name").unwrap().to_string(),
            solve_start:matches.value_of("solve").unwrap().parse().unwrap(),
            think_depth:matches.value_of("think").unwrap().parse().unwrap(),
            level: if matches.is_present("level") {
                let (verb, debug, _) = (matches.is_present("verb"),
                        matches.is_present("debug"),
                        matches.is_present("info"));
                if verb {"verb".to_string()} else if debug {"debug".to_string()} else {"info".to_string()}
            } else {
                "info".to_string()
            },
            random:if matches.value_of("name").unwrap()=="random"{true}else{false},
            eval:if matches.is_present("eval") {true} else {false},
            book:if matches.is_present("nobook") {false} else {true},
            mktrain:if matches.is_present("mktrain") {true} else {false},
            dotrain:if matches.is_present("dotrain") {true} else {false},
            cntntrain:if matches.is_present("cntntrain") {true} else {false},
            sttrain:matches.value_of("sttrain").unwrap().parse().unwrap(),
            endtrain:matches.value_of("endtrain").unwrap().parse().unwrap(),
            beta:matches.value_of("beta").unwrap().parse().unwrap(),
            iter:matches.value_of("iter").unwrap().parse().unwrap(),
        }
    };

    pub static ref BOOK :RwLock<BTreeMap<u128, f32>> = RwLock::new(BTreeMap::new()); // openning book




    pub static ref RAND_MASK :RwLock<[[[u64; 256];8];2]> = RwLock::new([[[0u64; 256];8];2]); // zobrist hash 用の乱数
    pub static ref MAP_MUT: RwLock<HashMap<u64, i32>> = {
        RwLock::new(HashMap::new())
    };
}

pub struct ArgsSt {
    pub host: String,
    pub port: String,
    pub name: String,
    pub solve_start: i32,
    pub think_depth: i32,
    pub level: String,
    pub random: bool,
    pub eval: bool,
    pub book: bool,
    pub mktrain: bool,
    pub dotrain: bool,
    pub cntntrain: bool,
    pub endtrain: i32,
    pub sttrain: i32,
    pub beta: f32,
    pub iter: i32,
}

pub const MAX: i32 = 1 << 30;
pub const MIN: i32 = -(1 << 30);
// for solver
pub const SOLVE_SORT_END: i32 = 5;
pub const HASH_DEPTH: i32 = 0;

// for think
//pub const THINK_DEPTH: i32 = 2;
