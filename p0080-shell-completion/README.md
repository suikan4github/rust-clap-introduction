# シェル補完スクリプト

列挙型に対応する引数の解析（パース）機能を追加する。

CLIから文字列として与えられた引数を、対応する列挙型にマッピングする事が狙いである。

## ソースコード

この例題はこれまでより少し大がかりな変更が入る。

まず、clapクレートのインポートが変わる。これまで`clap::Parser`のみをインポートしていたが、この例題では`clap::ValueEnum`も必要になる。
```rust:main.rs
use clap::{Parser, ValueEnum};
```
次に、列挙型の宣言をする。この列挙型に対応する引数を解析するので、`derive`属性に`ValueEnum`を指定しておく。これで、clapは文字列を受け取ってこの列挙型の値にマッピングさせることを知る。
```rust:main.rs
#[derive(Debug, Clone, ValueEnum)]
enum EngineType {
    Reciprocating,
    Turboprop,
    Turbojet,
    Turbofan,
}
```
最後にコマンドライン引数構造体に、先に宣言した列挙型のフィールドを追加する。この関数には`#[arg]`属性で、`value_enum`を指定することで、platに列挙型の値の解析を行うよう指示する。

```rust:main.rs
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(help = "Name of airclaft")]
    name: String,

    #[arg(short, long, default_value = "", help = "Manufacturer of airclaft")]
    manufacturer: String,

    #[arg(
        short,
        long,
        default_value_t = 1904,
        help = "First flight year of airclaft"
    )]
    first_flight: i32,

    // enum型のコマンドライン引数を解析する。
    #[arg(short, long, value_enum, default_value_t = EngineType::Reciprocating,
        help = "Engine type")]
    engine_type: EngineType,
}
```
上の例では`engin_type`フィールドが列挙型`EnginType`である。

clapはこの変数名から類推して`-e`および`--engine-type`オプションを作り出す。引数は列挙型のリテラルと同じ文字列である。


## 実行

引数を`-e`とともに与えると、その引数に対応する列挙型の値がengine_typeフィールドに束縛される。省略した場合は`default_value`として指定した`Reciprocating`が束縛される。

```sh
$ cargo run -q -- B747 -m Boeing -f 1964 -e turbofan
Cli { name: "B747", manufacturer: "Boeing", first_flight: 1964, engine_type: Turbofan }
```
なお、`-e`オプションに与えることのできる引数は、`-h`で表示される文字列だけである。この文字列は列挙型のリテラルそのものとは限らないので注意する。

```sh
$ cargo run -q -- -h
Usage: aircraft [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of airclaft

Options:
  -m, --manufacturer <MANUFACTURER>  Manufacturer of airclaft [default: ]
  -f, --first-flight <FIRST_FLIGHT>  First flight year of airclaft [default: 1904]
  -e, --engine-type <ENGINE_TYPE>    Engine type [default: reciprocating] [possible values: reciprocating, turboprop, turbojet, turbofan]
  -h, --help                         Print help
  -V, --version                      Print version
```


