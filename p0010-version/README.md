# バージョン表示プログラム

clapクレートによるバージョン表示プログラム。次の二つの機能を実装する。

- ヘルプ表示機能
- バージョン表示機能

## Cargo.toml
Cargo.tomlは以下の通り。`[package]`の`version`は、プログラムの中で参照して実行時にバージョン情報として表示される。
```toml:Cargo.toml
[package]
name = "aircraft"
version = "0.1.0"
authors = ["Seiichi Horie<mail.example.com>"]
edition = "2021"

[dependencies]
clap = { version = "4.*", features = ["derive"] }
```
{[dependencies]}に追加した`clap = { version = "4.*", features = ["derive"] }`はclapの必要バージョンを宣言するほか、`derive`機能を利用することを宣言する。

clapのderive機能はソースコードから引数の名前や利用方法を類推する機能で、プログラムを大幅に簡素化する。

## ソースコード
main.rsの冒頭では`clap::Parser`のインポートを宣言する。
```rust:main.rs
use clap::Parser;
```

`struct Cli`はコマンドライン引数を格納する構造体である。
```rust:main.rs
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {}
```
この構造体にはclapからふたつの属性が付加されている。

`derive`属性は、この構造体のソースからコマンドライン引数をコンパイラが類推することを宣言している。

`command`属性は、このプログラムには`version`コマンドが引数として実装されるべきであることを宣言している。実装はclapが行うのでプログラマは何もしなくてよい。

ヘルプ機能に関してはデフォルトでclapが実装するのでプログラマは何もしなくてよい。


main()関数の以下の行は、コマンドライン引数を解析（パース）してCli構造体型の変数cliを生成している。
```rust:main.rs
    let cli = Cli::parse();
```
## 実行
`-V`または`--version`オプションによりプログラムのバージョンを表示することができる。バージョンはCargo.tomlから抽出したもの。
```
$ aircraft -V
aircraft 0.1.0
```
コマンド名の代わりに`cargo run -q --`の後ろにオプションを付けても、同じ結果を得られる。
```
cargo run -q -- -V
aircraft 0.1.0
```
`-h`または`--help`オプションを付けると、ヘルプメッセージが表示される。これは暗黙に実装された機能である。

```
$ cargo run -q -- -h
Usage: aircraft

Options:
  -h, --help     Print help
  -V, --version  Print version
```
