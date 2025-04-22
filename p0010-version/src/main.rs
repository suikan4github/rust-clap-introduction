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
/// Demonstration of the simple applicaiton with version and help.
///
/// Without expliciit programming, the clap crate add -h and --help options.
/// The added help display is well formatted and easy to read.
/// In addtion, the programmer can add -V and --version options explicitly.
struct Cli {}

fn main() {
    // コマンドライン引数を解析する。
    let cli = Cli::parse();

    // コマンドライン引数を解析した結果を表示する。
    println!("{:?}", cli);
}
