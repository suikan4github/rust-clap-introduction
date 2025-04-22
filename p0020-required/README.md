# 必須引数

省略することができない引数の解析（パース）機能を追加する。

## ソースコード

コマンドライン構造体の要素に`clap`属性で省略時の値を指定しない場合には、省略不能な引数として扱われる。
また、この場合の引数の値は文字列として処理される。
なお、`clap`属性に`required = true`を追加した場合も省略不能な引数として扱われる。
```
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[clap(help = "Name of airclaft")]
    name: String,

    #[clap(short, long, default_value = "", help = "Manufacturer of airclaft")]
    manufacturer: String,
}
```
'arg'属性の'required'は、ユーザーに見えない部分で生成されるプログラムが呼び出すメソッド名である。
つまりここでは、`required(true)`メソッドを呼ぶことを指示しており、これによってname文字列に対応する引数は省略が許されないとわかる。

`clap`属性の`help`メソッドはヘルプ文字列を指定する。
この文字列はコマンドラインからhelpオプションを指定した際、この引数の情報として表示される。

なお、上の例でnameメンバー変数の型を`Option<String>`にした場合、clapは引数を省略可能にするので注意が必要である。

```rust
    #[clap(help = "Name of airclaft")]
    name: Option<String>,
```

## 実行

引数を省略するとプログラムはエラー終了する。

```
$ cargo run -q -- B747
Cli { name: "B747" }
```

ヘルプ画面では、引数名は`<NAME>`と表示される。`<>`は省略不能であることを示す記号である。

`NAME`はコンパイラが`[Derive]`によってソースコードの要素名から類推した引数名である。
また、`<NAME>`の横に`help`メソッドに与えたヘルプ文字列が表示されている。
```
$ cargo run -q -- -h
Usage: aircraft <NAME>

Arguments:
  <NAME>  Name of airclaft

Options:
  -h, --help     Print help
  -V, --version  Print version
```