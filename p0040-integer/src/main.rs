// clapクレートを使ってコマンドライン引数をパースするプログラム。
// Cargo.tomlに以下を追加する。
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }
// clapのderive機能を使うために、クレートのバージョンを4.0以上にする必要がある。

// clapクレートは、コマンドライン引数をパースするためのライブラリ。
use clap::Parser;

// derive(parser)属性を使ってコマンドライン引数のパースのためのコードを自動生成する。
#[derive(Parser, Debug)]
// command(version)属性を使って、コマンドラインに -V --version オプションを追加する。
#[command(version)]
// コマンドライン引数をパースするための構造体を定義する。
// このプログラムではユーザー定義の構造体は空である。
struct Cli {
    // 省略できないコマンドライン文字列。
    #[clap(required = true)]
    name: String,

    // 省略可能なコマンドライン文字列。
    #[clap(short, long, default_value = "")]
    manufacturer: String,

    // 文字列以外のコマンドライン引数をパースする。
    #[clap(short, long, default_value_t = 1904)]
    first_flight_year: i32,
}

fn main() {
    // コマンドライン引数をパースする。
    let cli = Cli::parse();

    // コマンドライン引数をパースした結果を表示する。
    println!("{:?}", cli);
}
