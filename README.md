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

#### flip
参考サイト :https://primenumber.hatenadiary.jp/entry/2016/12/26/063226
着手位置から見て、上、左、右上、左下のマス目をマスクするビット列を計算する。

### solver
ファイルはsolver.rs
終盤ソルバーを実装。速さ優先探索で走査。

現在は16手読みに数秒程度。50回やってworstが6.999秒
`--release`オプションをつければ、ほぼ1秒以内に押さまる。

実際に使う場合は19手読みほどまで可能
