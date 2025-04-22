// clapクレートを使ってコマンドライン引数を解析（パース）するプログラム。
// Cargo.tomlに以下を追加する。
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }
// clapのderive機能を使うために、クレートのバージョンを4.0以上にする必要がある。

// clapクレートは、コマンドライン引数を解析するためのライブラリ。
use clap::Parser;

// derive(parser)属性を使ってコマンドライン引数の解析のためのコードを自動生成する。
#[derive(Parser, Debug)]
// command(version)属性を使って、コマンドラインに -V --version オプションを追加する。
#[command(version, author, long_about)]
/// Demonstration of the simple applicaiton with version and help.
///
/// Without expliciit programming,
/// the clap crate add -h and --help options.
/// This is very convinient.
struct Cli {}

fn main() {
    // コマンドライン引数を解析する。
    let cli = Cli::parse();

    // コマンドライン引数を解析した結果を表示する。
    println!("{:?}", cli);
}
