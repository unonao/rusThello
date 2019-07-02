use clap::{App, Arg, SubCommand};

pub fn arg_get() {
    let app = App::new("rusThello")
        .version("0.1.0") // バージョン情報
        .author("unonao") // 作者情報
        .about("othello by rust") // このアプリについて
        .arg(
            Arg::with_name("pa") // 位置引数を定義
                .help("sample positional argument") // ヘルプメッセージ
                .required(true), // この引数は必須であることを定義
        )
        .arg(
            Arg::with_name("flg") // フラグを定義
                .help("sample flag") // ヘルプメッセージ
                .short("f") // ショートコマンド
                .long("flag"), // ロングコマンド
        )
        .arg(
            Arg::with_name("opt") // オプションを定義
                .help("sample option") // ヘルプメッセージ
                .short("o") // ショートコマンド
                .long("opt") // ロングコマンド
                .takes_value(true), // 値を持つことを定義
        )
        .subcommand(
            SubCommand::with_name("sub") // サブコマンドを定義
                .about("sample subcommand") // このサブコマンドについて
                .arg(
                    Arg::with_name("subflg") // フラグを定義
                        .help("sample flag by sub") // ヘルプメッセージ
                        .short("f") // ショートコマンド
                        .long("flag"), // ロングコマンド
                ),
        );

    let matches = app.get_matches();
    // paが指定されていれば値を表示
    if let Some(o) = matches.value_of("pa") {
        println!("Value for pa: {}", o);
    }

    // optが指定されていれば値を表示
    if let Some(o) = matches.value_of("opt") {
        println!("Value for opt: {}", o);
    }

    // flgのON/OFFで表示するメッセージを切り替え
    println!(
        "flg is {}",
        if matches.is_present("flg") {
            "ON"
        } else {
            "OFF"
        }
    );

    // subサブコマンドの解析結果を取得
    if let Some(ref matches) = matches.subcommand_matches("sub") {
        println!("used sub"); // subが指定されていればメッセージを表示
                              // subflgのON/OFFで表示するメッセージを切り替え
        println!(
            "subflg is {}",
            if matches.is_present("subflg") {
                "ON"
            } else {
                "OFF"
            }
        );
    }
}
