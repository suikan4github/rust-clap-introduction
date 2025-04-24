# 必須引数

省略することができない引数の解析（パース）機能を追加する。

## ソースコード

コマンドライン構造体の要素に`#[arg]`属性を与え、かつ省略時の値を指定しない場合には、省略不能な引数として扱われる。
```rust
#[derive(Parser, Debug)]
#[command(version,about)]
struct Cli {
    #[arg()]
    /// Name of aircraft.
    name: String,
}
```
`#[arg]`属性を付加することで、コマンドライン引数の解析（パース）によって続くnameフィールドに値を束縛することを指示している。

また、Docコメントはヘルプ文字列として抽出され、ヘルプ画面に表示される。

なお、`#[arg]`属性に`required = true`を追加した場合も省略不能な引数として扱われる。


## 実行

引数を省略するとプログラムはエラー終了する。

```
$ cargo run -q -- B747
Cli { name: "B747" }
```

ヘルプ画面では、引数名は`<NAME>`と表示される。`<>`は省略不能であることを示す記号である。

`NAME`はclapが`#[arg]`属性によってソースコードのフィールド名から類推した引数名である。
また、`<NAME>`の横にDocコメントから抽出したヘルプ文字列が表示されている。
```
$ cargo run -q -- -h
Usage: aircraft <NAME>

Arguments:
  <NAME>  Name of airclaft

Options:
  -h, --help     Print help
  -V, --version  Print version
```