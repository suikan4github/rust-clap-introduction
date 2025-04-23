// clapクレートを使ってコマンドライン引数を解析（パース）するプログラム。
// Cargo.tomlに以下を追加する。
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }
// clapのderive機能を使うために、クレートのバージョンを4.0以上にする必要がある。

// Parser トレイトを使って、コマンドライン引数を解析するための構造体を定義する。
use clap::Parser;

// derive(parser)属性を使ってコマンドライン引数の解析のためのコードを自動生成する。
#[derive(Parser, Debug)]
// command(version)属性を使って、コマンドラインに -V --version オプションを追加する。
#[command(version)]
// Doc コメントを使って、コマンドライン引数の説明をヘルプ情報に追加する。
/// Demonstration of a optional arguments.
///
/// By adding default_value to #[arg()] attribute, the field becomes an optional argument.
/// This optional argument is a String type.
struct Cli {
    // 省略できないコマンドライン文字列。Docコメントはヘルプ情報に追加される。
    #[arg()]
    /// Name of aircraft.
    name: String,

    // 省略可能なコマンドライン文字列。
    #[arg(short, long, default_value = "")]
    /// Manufacturer of aircraft.
    manufacturer: String,
}

fn main() {
    // コマンドライン引数を解析する。
    let cli = Cli::parse();

    // コマンドライン引数を解析した結果を表示する。
    println!("{:?}", cli);
}
