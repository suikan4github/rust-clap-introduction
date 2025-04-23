# clapクレート入門
Rustのclapクレートを使ってCLIの引数を解析（パース）する。

試験環境
- Ubuntu 24.04 (WSL)
- rustc 1.86.0 
- clap 4.5.37
- clap_complete 4.5.47

# 内容
以下のサンプルプログラムを用意している。

- [p0010](./p0010-version/README.md) : バージョン表示プログラム
- [p0020](./p0020-required/README.md) :  必須引数
- [p0030](./p0030-option/README.md) :  省略可能引数
- [p0040](./p0040-integer/README.md) : 非文字列引数
- [p0050](./p0050-enum/README.md) : 列挙型のための引数解析
- [p0060](./p0060-switch/README.md) : 引数を持たないスイッチ
- [p0070](./p0070-subcommand/README.md) : コマンドとサブコマンド
- [p0080](./p0080-shell-completion/README.md) : シェル補完スクリプト

# 準備
Ubuntu 24.04の例。
## Rustのインストール
コマンドラインから以下のプログラムを実行する。root権限は不要
```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
インストールが終わったらshellを再起動し、以下のコマンドを実行する。
```sh
rustc --version
```
バージョン番号が表示されたらインストールは成功である。なお、コンパイラは~/.cargo/binにインストールされている。

## アップデート
一度インストールしたコンパイラは以下のコマンドでアップデートできる。
```sh
rustup update
```

## アンインストール
Rustコンパイラとrustupをアンインストールするときには以下のコマンドを実行する。
```sh
rustup self uninstall
```
