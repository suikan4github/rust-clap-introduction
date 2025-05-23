# 省略可能引数

省略可能な引数の解析（パース）機能を追加する。

## ソースコード

コマンドライン構造体の要素に`#[arg()]`属性を付け、かつ`default_value`を指定した場合には省略可能な引数として扱われる。
引数の値は文字列として処理される。

```rust
#[derive(Parser, Debug)]
#[command(version,about)]
struct Cli {
    #[arg()]
    /// Name of aircraft
    name: String,

    // 省略可能なコマンドライン引数。
    #[arg(short, long, default_value = "")]
    /// Manufacturer of aircraft.
    manufacturer: String,
}
```
`#[arg]`属性に`short`および`long`を指定することで、clapはフィールド名から類推して短いオプションと長いオプションを生成する。この場合は以下のようになる。

| フィールド名   | 短いオプション | 長いオプション |
|-----          |---------   |------      |
| manufacturer | -m          | --manufacturer |

`default_value`の値は空文字列でも構わない。`default_value`そのものを指定しない場合、そのオプションを省略すると値はNONEになる。そのため、フィールドの型は`String`ではなく`Option<String>`にしなければならない。

```rust
    #[arg(short, long, help = "Manufacturer of airclaft")]
    manufacturer: Option<String>,
```

## 実行

引数を`-m`とともに与えると、manufacturerフィールドにその引数が文字列として束縛される。省略した場合は`default_value`として指定した空文字列が束縛される。
```
$ cargo run -q -- B747 -m Boeing
Cli { name: "B747", manufacturer: "Boeing" }
```
`-h`オプションを指定すると、新たに追加した省略可能な引数が加わっている。
```
$ cargo run -q -- -h
Demonstration of a optional arguments

Usage: aircraft [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of aircraft

Options:
  -m, --manufacturer <MANUFACTURER>  Manufacturer of aircraft [default: ]
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version
```