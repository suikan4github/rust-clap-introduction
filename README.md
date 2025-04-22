# clapクレート入門
clapクレートを使ってCLIの引数を解析（パース）する。

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
