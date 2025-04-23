// clapクレートを使ってコマンドライン引数を解析（パース）するプログラム。
// Cargo.tomlに以下を追加する。
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }

// clapクレートは、コマンドライン引数を解析するためのライブラリ。
use clap::{Parser, Subcommand, ValueEnum};

// enum型のコマンドライン引数を定義する。
#[derive(Debug, Clone, ValueEnum)]
enum EngineType {
    Reciprocating,
    Turboprop,
    Turbojet,
    Turbofan,
}

// derive(parser)属性を使ってコマンドライン引数の解析のためのコードを自動生成する。
#[derive(Parser, Debug)]
// command(version)属性を使って、コマンドラインに -V --version オプションを追加する。
#[command(version)]
// コマンドライン引数を解析するための構造体を定義する。
// Doc コメントを使って、コマンドライン引数の説明をヘルプ情報に追加する。
/// Demonstration of a switch argument.
///
/// By using bool field with #[arg()] attribute, the field becomes a switch artument.
/// Switch doesn't have any value. If not specified, the field value is false.
struct Cli {
    // コマンドライン引数のサブコマンドを定義する。
    #[command(subcommand)]
    /// Subcommand of aircraft.
    command: Commands,
}

// derive(subcommand)属性を使って、コマンドライン引数の解析のためのコードを自動生成する。
#[derive(Subcommand, Debug)]
enum Commands {
    Real {
        // 省略できないコマンドライン文字列。Docコメントはヘルプ情報に追加される。
        #[arg()]
        /// Name of aircraft.
        name: String,

        // 省略可能なコマンドライン文字列。
        #[arg(short, long, default_value = "")]
        /// Manufacturer of aircraft.
        manufacturer: String,

        // 文字列以外のコマンドライン引数を解析する。
        #[arg(short, long, default_value_t = 1904)]
        /// First flight year of aircraft.
        first_flight: i32,

        // enum型のコマンドライン引数を解析する。
        #[arg(short, long, value_enum, default_value_t = EngineType::Reciprocating,
         )]
        /// Engine type of aircraft.
        engine_type: EngineType,

        // 論理型のコマンドライン引数を解析する。このオプションはスイッチとして機能する。
        #[arg(short, long)]
        /// Pretty print mode.
        pretty_print: bool,
    },
    Idea {
        // 省略できないコマンドライン文字列。
        #[arg()]
        /// Name of aircraft.
        name: String,

        // 省略可能なコマンドライン文字列。
        #[arg(short, long, default_value = "")]
        /// Designer of aircraft.
        designer: String,
    },
}

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
                    " {} {} {:?} {:?}",
                    name, manufacturer, first_flight, engine_type
                );
            }
        }
        Commands::Idea { name, designer } => {
            println!("{} {}", name, designer);
        }
    }
}
