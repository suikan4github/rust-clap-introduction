# 非文字列引数

文字列ではない引数の解析（パース）機能を追加する。

## ソースコード

省略可能な引数の属性でデフォルト値を指定する際、`default_value_t`を使用すると、引数を文字列型以外の値として解析する。
```rust:main.rs
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[clap(help = "Name of airclaft", required = true)]
    name: String,

    #[clap(short, long, default_value = "", help = "Manufacturer of airclaft")]
    manufacturer: String,

    #[clap(
        short,
        long,
        default_value_t = 1904,
        help = "First flight year of airclaft"
    )]
    first_flight: i32,
}
```
上の例では`first_flight`メンバー変数が属性`clap`の中に`default_value_t`を持っている。clapは`first_flight`の型が`i32`であることから、引数の文字列を符号付き整数型として解析する。

`default_value_t`を使用しない場合、`clap`属性の中で`value_parser`を指定すると、やはり文字列以外の型として解析する。


## 実行

引数を`-f`とともに与えると、first_flightメンバー変数にその引数が整数として束縛される。省略した場合は`default_value`として指定した1906が束縛される。

```sh
$ cargo run -q  -- B747 -m Boeing -f 1964
Cli { name: "B747", manufacturer: "Boeing", first_flight: 1964 }
```

