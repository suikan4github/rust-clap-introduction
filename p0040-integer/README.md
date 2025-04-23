# 非文字列引数

文字列ではない引数の解析（パース）機能を追加する。

## ソースコード

省略可能な引数の属性でデフォルト値を指定する際、`default_value_t`を使用すると、引数を文字列型以外の値として解析する。
```rust:main.rs
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg()]
    name: String,

    #[arg(short, long, default_value = "")]
    manufacturer: String,

    // 文字列以外のコマンドライン引数を解析する。
    #[arg(short, long, default_value_t = 1904)]
    /// First flight year of aircraft.
    first_flight: i32,
}
```
上の例では`first_flight`フィールドが属性`#[arg]`の中に`default_value_t`を持っている。clapは`first_flight`の型が`i32`であることから、引数の文字列を符号付き整数型として解析する。

`default_value_t`を使用しない場合、`#[arg]`属性の中で`value_parser`を指定すると、やはり文字列以外の型として解析する。


## 実行

引数を`-f`とともに与えると、first_flightフィールドにその引数が整数として束縛される。省略した場合は`default_value`として指定した1906が束縛される。

```sh
$ cargo run -q  -- B747 -m Boeing -f 1964
Cli { name: "B747", manufacturer: "Boeing", first_flight: 1964 }
```

