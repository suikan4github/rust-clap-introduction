# バージョン表示プログラム

clapクレートによるバージョン表示プログラム

`-V`または`--version`オプションによりプログラムのバージョンを表示することができる。バージョンはCargo.tomlから抽出したもの。
```
$ clap_test -V
clap_test 0.1.0
```
コマンド名の代わりに`cargo run -q --`の後ろにオプションを付けても、同じ結果を得られる。
```
cargo run -q -- -V
clap_test 0.1.0
```
`-h`または`--help`オプションを付けると、ヘルプメッセージが表示される。

```
$ cargo run -q -- -h
Usage: clap_test

Options:
  -h, --help     Print help
  -V, --version  Print version
```