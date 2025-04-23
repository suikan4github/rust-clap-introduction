// clapクレートを使ってコマンドライン引数を解析（パース）するプログラム。
// Cargo.tomlに以下を追加する。
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }

// Parser トレイトを使って、コマンドライン引数を解析するための構造体を定義する。
use clap::Parser;

// derive(parser)属性を使ってコマンドライン引数の解析のためのコードを自動生成する。
#[derive(Parser, Debug)]
// command(version)属性を使って、コマンドラインに -V --version オプションを追加する。
#[command(version)]
// Doc コメントを使って、コマンドライン引数の説明をヘルプ情報に追加する。
/// Demonstration of a required arguments.
///
/// By adding #[arg()] attribute, a field becomes a required argument.
/// The required arguments is a String type.
/// The Doc comment of the field is added to the help information.
struct Cli {
    // 省略できないコマンドライン文字列。Docコメントはヘルプ情報に追加される。
    #[arg()]
    /// Name of aircraft.
    name: String,
}

fn main() {
    // コマンドライン引数を解析する。
    let cli = Cli::parse();

    // コマンドライン引数を解析した結果を表示する。
    println!("{:?}", cli);
}
