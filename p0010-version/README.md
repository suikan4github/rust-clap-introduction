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
main.rsの冒頭では`clap::Parser`トレイトのインポートを宣言する。
```rust:main.rs
use clap::Parser;
```

`struct Cli`はコマンドライン引数を格納する構造体である。
```rust:main.rs
#[derive(Parser, Debug)]
#[command(version, about)]
/// Demonstration of the simple applicaiton with version and help.
///
/// Without expliciit programming, the clap crate add -h and --help options.
/// The added help display is well formatted and easy to read.
/// In addtion, the programmer can add -V and --version options explicitly.
struct Cli {}
```
この構造体にはclapからふたつの属性が付加されている。

`#[derive]`属性は、この構造体のソースからコマンドライン引数をコンパイラが類推することを宣言している。

`#[command]`属性は、このプログラムには`version`コマンドが引数として実装されるべきであることを宣言している。バージョン情報はCargo.tomlから持って来る。実装はclapが行うのでプログラマは何もしなくてよい。

また、`#[command]`属性は、`about`を宣言することでCargo.tomlのdescriptionから説明文を持って来ることを宣言している。ヘルプ機能に関してはデフォルトでclapが実装するのでプログラマは何もしなくてよい。

Cli{}構造体にDocコメントを付けてヘルプに使用することも可能である。この場合、`#[command]`にaboutを宣言する必要はない。
```rust:main.rs
#[derive(Parser, Debug)]
#[command(version)]
/// Demonstration of the simple applicaiton with version and help.
///
/// Without expliciit programming, the clap crate add -h and --help options.
/// The added help display is well formatted and easy to read.
/// In addtion, the programmer can add -V and --version options explicitly.
struct Cli {}
```

このDocコメントはclapによってaboutおよびlong_about情報として使用される。about情報として扱われるのはDocコメントの最初の行である。1行開けて3行目からはlong_about情報として扱われる。

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
`-h`オプションをつけるとヘルプメッセージが表示される。これは暗黙に実装された機能である。冒頭にabout情報が表示されている。

```
$ cargo run -q -- -h
Demonstration of the simple applicaiton with version and help

Usage: aircraft

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

`--help`オプションをつけると冒頭にlong_aboutも表示される。

```
$ cargo run -q -- --help
Demonstration of the simple applicaiton with version and help.

Usage: aircraft

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
