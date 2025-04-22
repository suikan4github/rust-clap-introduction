# 省略可能引数

省略可能な引数の解析（パース）機能を追加する。

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
`clap()`属性に`short`および`long`を指定することで、clapはフィールド名から類推して短いオプションと長いオプションを生成する。この場合は以下のようになる。

| フィールド名   | 短いオプション | 長いオプション |
|-----          |---------   |------      |
| manufacturer | -m          | --manufacturer |

`default_value`の値は空文字列でも構わない。`default_value`そのものを指定しない場合、そのオプションを省略すると値はNONEになる。そのため、フィールドの型は`String`ではなく`Option<String>`にしなければならない。

```rust
    #[clap(short, long, help = "Manufacturer of airclaft")]
    manufacturer: Option<String>,
```

## 実行

引数を`-m`とともに与えると、manufacturerフィールドにその引数が文字列として束縛される。省略した場合は`default_value`として指定した空文字列が束縛される。

```sh
$ cargo run -q  -- B747 -m Boeing
Cli { name: "B747", manufacturer: "Boeing" }
```

ヘルプ画面には、`-m`および`--manufacturer`が追加される。

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