# rusThello

## install
Rustのインストールのために、Unix系なら以下のコマンドを入力する

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


## nom
parserのために、nom(version 5.0.0)を使用した。

## ディレクトリ構成
srcディレクトリにソースコード一式がある。
- color.rs: 黒や白などの、石の色に関するファイル
- command_parser.rs: プロトコルで定められたメッセージをパースする関数や、構造体・列挙型を定義するファイル
- main.rs
- play.rs: オセロ用の基本関数やBoard構造体についてのファイル
- print.rs : print系の出力用関数を集めたファイル
- solver.rs: 終盤ソルバー用のファイル
- think.rs: 思考ルーチン用ファイル

## 実装
### Board
ファイルはplay.rs
bit boardで実装。
`u64`型２つで定義されたBoard構造体に対して、メソッドを定義することで基本的な操作を行う。
### solver
ファイルはsolver.rs
終盤ソルバーを実装。現在は15手読み
