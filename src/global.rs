/*
    共通の定数や変数などをまとめたファイル
*/
use clap::{App, Arg, ArgGroup};
use std::collections::BTreeMap;
use std::collections::HashMap;
//use std::env;
use std::sync::RwLock;
lazy_static! {
    pub static ref ARGS: ArgsSt = {

        let app = App::new(crate_name!())   // Cargo.tomlのnameを参照する
            .version(crate_version!())      // Cargo.tomlのversionを参照する
            .author(crate_authors!())       // Cargo.tomlのauthorsを参照する
            .about(crate_description!())    // Cargo.tomlのdescriptionを参照する
            .arg(Arg::from_usage("[eval] -e --eval 'eval thinker'"))
            .arg(Arg::from_usage("-h --host [HOST] 'host ip address'").default_value("127.0.0.1"))
            .arg(Arg::from_usage("-p --port [PORT] 'port number'").default_value("3000"))
            .arg(Arg::from_usage("-n --name [NAME] 'player name'").default_value("rusThello"))
            .arg(Arg::from_usage("-s --solve [SOLVE] 'start solver depth'").default_value("12"))
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
            no_solve:if matches.value_of("name").unwrap()=="random"{true}else{false},
            eval:if matches.is_present("eval") {true} else {false}
        }
    };

    pub static ref BOOK :RwLock<BTreeMap<u128, f32>> = RwLock::new(BTreeMap::new()); // openning book
    pub static ref HIST :RwLock<u128> = RwLock::new(0);
    pub static ref ROTATE_NUM :RwLock<i32> = RwLock::new(0);


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
    pub no_solve: bool,
    pub eval: bool,
}

pub const MAX: i32 = 1 << 30;
pub const MIN: i32 = -(1 << 30);
// for solver
pub const SOLVE_SORT_END: i32 = 5;
pub const HASH_DEPTH: i32 = 0;

// for think
pub const THINE_DEPTH: i32 = 2;
