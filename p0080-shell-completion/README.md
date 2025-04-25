# シェル補完スクリプト

CLIアプリケーションをシェルの補完に対応させるために、シェルスクリプトを生成する。

## ソースコード

新たにclap_completeクレートを使用する。そこで、このクレートを`Cargo.toml`に追加する。

```toml
[dependencies]
clap = { version = "4.x", features = ["derive"] }
clap_complete = "4.x"
```


クレートのインポートが変わる。これまで利用していたトレイトに加え、`clap::CommandFactory`が必要になる。また、`clap_complete`クレートも必要になる。
```rust
use clap::{CommandFactory as _, Parser, Subcommand, ValueEnum};
use clap_complete;
```

`clap::CommandFactory`を`as _`としてインポートしているのは、トレイト名を直接使わないからである。一方で、このトレイトのメソッドを後で使うことになるのでインポート自体は必要である。`as _`無しでインポートしても問題なくビルドできるし動作する。


最後にコマンドライン引数の列挙型に`GenerateShellCompletion`を追加する。これはコマンドとして機能し、サブコマンドとして`shell : clap_complete::Shell`フィールドを持つ。clapはここから`-s`および`--shell`オプションを作る。`clap_complete::Shell`は列挙型である。

```rust:main.rs
#[derive(Subcommand, Debug)]
enum Commands {
    /// Real aircraft.
    Real {
      /// ...中略...
    },
    /// Idea only.
    Idea {
      /// ...中略...
    },
    /// Generate shell completion script.
    GenerateCompletion {
        // shellの種類を指定する。
        #[arg(short, long, value_enum)]
        /// Generate shell completion script.
        shell: clap_complete::Shell,
    },
}

```
main()関数では、match文に`Commands::GenerateCompletion`を追加する。このアームでは与えられた引数のシェル種別に応じて補完用のシェルスクリプトを実行時に生成する。
```rust
fn main() {
    // コマンドライン引数を解析する。
    let cli = Cli::parse();

    match cli.command {
        Commands::Real 
        /// ...中略...
        Commands::Idea 
        /// ...中略...
        Commands::GenerateCompletion { shell } => {
            // コマンドライン引数を解析するための構造体を生成する。
            let mut cmd = Cli::command();
            // 補完ファイルを生成するシェルの種類を指定する。
            clap_complete::generate(
                shell,                  // シェルの種類
                &mut cmd,               // コマンドライン引数情報
                env!("CARGO_PKG_NAME"), // パッケージ名
                &mut std::io::stdout(), // 出力先
            );

        }
    }
}
```
Commands::GenerateCompletionアームの実行文は定型文なので、他のアプリケーションでもそのまま利用できる。以下にその内容を簡単に説明する。

最初の文では、CLI引数を受け取るユーザー定義構造体Cliから、アプリケーションと引数の情報を生成している。`command()`メソッドは`clap::CommandFactory`トレイトで定義されたものであり、これを利用するために同トレイトをインポートしている。
```rust
            let mut cmd = Cli::command();
```
次の文ではユーザーが引数で選択したシェルの種別に応じてシェルスクリプトを生成している。
```rust
            clap_complete::generate(
                shell,                  // シェルの種類
                &mut cmd,               // コマンドライン引数情報
                env!("CARGO_PKG_NAME"), // パッケージ名
                &mut std::io::stdout(), // 出力先
            );
```
シェルの種類はユーザーが引数として渡したもの。

コマンドライン引数情報は引数の値ではなくオプションの文字列や列挙型の情報をパックしたものです。この情報から`generate()`メソッドはシェルが補完するために必要な文字のリストなどを作り出す。

パッケージ名はビルド環境からCargo.tomlのname情報を取り出している。これは実行ファイル名と同じになる。

出力先はこの例では標準出力を選んでいる。

## 実行

シェル名を`generate-shell-completion -s `とともに与えると、シェル補完スクリプトが標準出力に出力される。これを補完ファイルに保存してしかるべき場所に置けばシェル補完が有効になる。

bashの場合の例を挙げる。

```sh
$ cargo run -q -- generate-shell-completion -s bash > ~/.local/share/bash-completion/completions/aircraft.bash 
```

シェル補完スクリプトを保存したらシェルを再度開くか、以下のコマンドを実行する。
```sh
$ . ~/.local/share/bash-completion/completions/aircraft.bash 
```

これでシェル補完が使えるようになる。利用できるシェルの種類はヘルプ機能で表示させることができる。

```sh
$ cargo run -q -- generate-shell-completion -h
Generate shell completion script

Usage: aircraft generate-shell-completion --shell <SHELL>

Options:
  -s, --shell <SHELL>  Generate shell completion script [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help           Print help
```
