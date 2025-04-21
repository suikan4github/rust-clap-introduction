# 省略可能引数

省略可能な引数の解析機能を追加する。

## ソースコード

コマンドライン構造体の要素に`#[clap()]`属性を付け、かつ`required = true`を指定しない場合には省略可能な引数として扱われる。
特に指定しない場合、引数の値は文字列として処理される。

```rust:main.rs
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[clap(help = "Name of airclaft", required = true)]
    name: String,

    #[clap(short, long, default_value = "", help = "Manufacturer of airclaft")]
    manufacturer: String,
}
```
`clap()`属性に`short`および`long`を指定することで、clapはメンバー変数名から類推して短いオプションと長いオプションを生成する。この場合は以下のようになる。

| メンバー変数名   | 短いオプション | 長いオプション |
|-----          |---------   |------      |
| manufacturer | -m          | --manufacturer |

`default_value`の値は空文字列でも構わない。`default_value`そのものを指定しない場合、そのオプションを省略すると値はNONEになる。そのため、メンバー変数の型は`String`ではなく`Option<String>`にしなければならない。

```rust
    #[clap(short, long, help = "Manufacturer of airclaft")]
    manufacturer: Option<String>,
```

## 実行

引数を省略するとプログラムはエラー終了する。

```sh
$ cargo run -q -- B747
Cli { name: "B747" }
```

ヘルプ画面では、引数名は`<NAME>`と表示される。


`<HELP>`はコマンド名の横に表示されており、省略不能であることがわかる。
`NAME`はコンパイラが`[Derive]`によってソースコードの要素名から類推した引数名である。
また、`<NAME>`の横に`help`メソッドに与えたヘルプ文字列が表示されている。
```sh
$ cargo run -q -- -h
Usage: aircraft [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of airclaft

Options:
  -m, --manufacturer <MANUFACTURER>  Manufacturer of airclaft
  -h, --help                         Print help
  -V, --version                      Print version
```