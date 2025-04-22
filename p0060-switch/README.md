# 引数を持たないスイッチ

ブール型を導入してスイッチを実装する。

スイッチは引数を持たないオプションである。何らかの機能をオンにするために対応するスイッチをコマンドラインで指定する。

## ソースコード

最後にコマンドライン引数構造体に、先に宣言した列挙型のフィールドを追加する。この関数には`[clap]`属性で、`value_enum`を指定することで、platに列挙型の値の解析を行うよう指示する。

```rust:main.rs
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[clap(help = "Name of airclaft")]
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

    // enum型のコマンドライン引数を解析する。
    #[clap(short, long, value_enum, default_value_t = EngineType::Reciprocating,
        help = "Engine type")]
    engine_type: EngineType,

    #[clap(short, long, help = "Specify pretty print mode")]
    pretty_print: bool,
}
```
上の例では`pretty_print`フィールドが論理型である。

clapはこの変数名から類推して`-p`および`--pretty-print`オプションを作り出す。引数は不要である。`-p`または`--pretty-print`を指定した場合、`pretty_print`フィールドには`true`が束縛される。そうでない場合は、`false`が束縛される。

main()関数側では、このスイッチの値に応じて処理を変更する。

```rust:main.rs
fn main() {
    // コマンドライン引数を解析する。
    let cli = Cli::parse();

    if cli.pretty_print {
        // コマンドライン引数をきれいに表示する。
        println!("Name         {:#?}", cli.name);
        println!("Manufacturer {:#?}", cli.manufacturer);
        println!("First flight {:#?}", cli.first_flight);
        println!("Engine       {:#?}", cli.engine_type);
    } else {
        // コマンドライン引数をそのまま表示する。
        println!("{:?}", cli);
    }
}
```


## 実行

引数を`-p`とともに与えると、その引数に対応する列挙型の値がengine_typeフィールドに束縛される。省略した場合は`default_value`として指定した`Reciprocating`が束縛される。

```sh
$ cargo run -q -- B747 -m Boeing -f 1964 -e turbofan
Cli { name: "B747", manufacturer: "Boeing", first_flight: 1964, engine_type: Turbofan }
```
なお、`-e`オプションに与えることのできる引数は、`-h`で表示される文字列だけである。この文字列は列挙型のリテラルそのものとは限らないので注意する。

`-h`オプションを与えて表示すると、`-p`オプションには引数が無いことがわかる。

```sh
$ cargo run -q -- -h
Usage: aircraft [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of airclaft

Options:
  -m, --manufacturer <MANUFACTURER>  Manufacturer of airclaft [default: ]
  -f, --first-flight <FIRST_FLIGHT>  First flight year of airclaft [default: 1904]
  -e, --engine-type <ENGINE_TYPE>    Engine type [default: reciprocating] [possible values: reciprocating, turboprop, turbojet, turbofan]
  -p, --pretty-print                 Specify pretty print mode
  -h, --help                         Print help
  -V, --version                      Print version
```


