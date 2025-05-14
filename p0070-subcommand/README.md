# サブコマンド

より大規模なアプリケーションに向けて、clapにおけるサブコマンドの解析（パース）について説明する。

例えばgitを例に取ると、git cloneとgit commitは扱うオプションが異なる。
```sh
git clone <URL>
git commit [-a] [-m <COMMENT>]
```
gitの例でいえば、`clone`や`commit`はサブコマンドであり、それらに続いてサブコマンド固有のオプションが指定される。

この例では、これまで作ったaircraftアプリケーションにサブコマンドを追加する。サブコマンドは`Real`と`Idea`である。

`Real`サブコマンドはこれまで通り実在する飛行機の機種名と製造者や初飛行の時期を引数として受け取る。

`Idea`サブコマンドは空想上の飛行機について、機種名とそれを考案した人物の名前を引数として受け取る。

## ソースコード

この例題は列挙型への対応のときよりもさらに大がかりな変更が入る。サブコマンドはそれなりの規模のアプリケーションのためのものなので、引数の解析が複雑になるのは仕方のないことである。

まず、clapクレートのインポートが変わる。これまでインポートしていたトレイトに加えて`clap::ValueEnum`が加わる。
```rust
use clap::{Parser, Subcommand, ValueEnum};
```

サブコマンドは列挙型として実装し、オプションはその列挙型の値に保持される構造体型として実装する。

今回の例では`Real`および`Idea`というサブコマンドを使うので、列挙型の要素として`Real`と`Idea`を宣言する。そしてそれぞれが保持する値として構造体を宣言する。

`Real`が保持する構造体については、これまで`Cli`構造体の中に記述したものをそのままそっくり持ってきている。`Idea`が保持する構造体については、`Name`フィールドと`designer`フィールドを新設している。

この列挙型には`#[derive(Subcommand)]`を宣言していることに注意。この宣言によってclapにこの列挙型がコマンドライン引数のサブコマンドであることを知らせている。
```rust
// derive(subcommand)属性を使って、コマンドライン引数の解析のためのコードを自動生成する。
#[derive(Subcommand, Debug)]
enum Commands {
  #[arg()]
    /// Real aircraft.
    Real {
        #[arg()]
        /// Name of aircraft.
        name: String,

        #[arg(short, long, default_value = "")]
        /// Manufacturer of aircraft.
        manufacturer: String,

        #[arg(short, long, default_value_t = 1903)]
        /// First flight year of aircraft.
        first_flight: i32,

        #[arg(short, long, value_enum, default_value_t = EngineType::Reciprocating,
         )]
        /// Engine type of aircraft.
        engine_type: EngineType,

        #[arg(short, long)]
        /// Pretty print mode.
        pretty_print: bool,
    },
    /// Idea only.
    Idea {
        #[arg()]
        /// Name of aircraft.
        name: String,

        #[arg(short, long, default_value = "")]
        /// Designer of aircraft.
        designer: String,
    },
}

```
最後にコマンドライン引数構造体に、先に宣言したCommands列挙型のフィールドを追加する。

```rust
#[derive(Parser, Debug)]
#[command(version,about)]
struct Cli {
    // コマンドライン引数のサブコマンドを定義する。
    #[command(subcommand)]
    command: Commands,
}
```
上の例では`engin_type`フィールドが列挙型`EnginType`である。

clapはこの変数名から類推して`-e`および`--engine-type`オプションを作り出す。引数は列挙型のリテラルと同じ文字列である。

なお、この例ではサブコマンドをふたつ実装したが、自動的に`help`コマンドも実装される。

解析したCli構造体型変数は列挙型変数である。したがって、実行時にはmatch文を使って対応することになる。

```rust
fn main() {
    // コマンドライン引数を解析する。
    let cli = Cli::parse();

    match cli.command {
        Commands::Real {
            name,
            manufacturer,
            first_flight,
            engine_type,
            pretty_print,
        } => {
            if pretty_print {
                // コマンドライン引数をきれいに表示する。
                println!("Name         {:#?}", name);
                println!("Manufacturer {:#?}", manufacturer);
                println!("First flight {:#?}", first_flight);
                println!("Engine       {:#?}", engine_type);
            } else {
                // コマンドライン引数をそのまま表示する。
                println!(
                    " {}, {}, {}, {:?}",
                    name, manufacturer, first_flight, engine_type
                );
            }
        }
        Commands::Idea { name, designer } => {
            println!("{}, {}", name, designer);
        }
    }
}

```

## 実行

サブコマンドとして`real`か`idea`を与えることで、異なる動作を実現できる。

```
$ cargo run -q -- real B747 -m Boeing -f 1964 -e turbofan
 B747, Boeing, 1964, Turbofan

$ cargo run -q -- idea helicoptor -d "Da Vinci"
helicoptor, Da Vinci
```
試しに`-h`オプションを与えてみると、情報が随分減っている。コマンドとヘルプ、そしてバージョン表示のオプションしかない。
```
$ cargo run -q -- -h
Demonstration of command/subcommand

Usage: aircraft <COMMAND>

Commands:
  real  Real aircraft
  idea  Idea only
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

より詳しい情報が欲しいときには、コマンドに続いて`-h`を与える。

```
$ cargo run -q -- real -h
Real aircraft

Usage: aircraft real [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of aircraft

Options:
  -m, --manufacturer <MANUFACTURER>  Manufacturer of aircraft [default: ]
  -f, --first-flight <FIRST_FLIGHT>  First flight year of aircraft [default: 1903]
  -e, --engine-type <ENGINE_TYPE>    Engine type of aircraft [default: reciprocating] [possible values: reciprocating, turboprop, turbojet, turbofan]
  -p, --pretty-print                 Pretty print mode
  -h, --help                         Print help
```
```
$ cargo run -q -- idea -h
Idea only

Usage: aircraft idea [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of aircraft

Options:
  -d, --designer <DESIGNER>  Designer of aircraft [default: ]
  -h, --help                 Print help
```