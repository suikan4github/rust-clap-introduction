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
#[command(version)]
// コマンドライン引数を解析するための構造体を定義する。
// このプログラムではユーザー定義の構造体は空である。
struct Cli {
    // 省略できないコマンドライン文字列。
    #[clap(required = true)]
    name: String,

    // 省略可能なコマンドライン文字列。
    #[clap(short, long, default_value = "")]
    manufacturer: String,

    // 文字列以外のコマンドライン引数を解析する。
    #[clap(short, long, default_value_t = 1904)]
    first_flight_year: i32,
}

fn main() {
    // コマンドライン引数を解析する。
    let cli = Cli::parse();

    // コマンドライン引数を解析した結果を表示する。
    println!("{:?}", cli);
}
