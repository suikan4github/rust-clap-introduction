# 必須引数

省略することができない引数の解析機能を追加する。

## ソースコード

コマンドライン構造体の要素に`#[clap(required = true)]`属性を付けると省略不能な引数として扱われる。
また、この場合の引数の値は文字列として処理される。

```
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[clap(help = "Name of airclaft", required = true)]
    name: String,

    #[clap(short, long, default_value = "", help = "Manufacturer of airclaft")]
    manufacturer: String,
}
```
'arg'属性の'required'は、ユーザーに見えない部分で生成されるプログラムが呼び出すメソッド名である。
つまりここでは、`required(true)`メソッドを呼ぶことを指示しており、これによってname文字列に対応する引数は省略が許されないとわかる。

`clap`属性の`help`メソッドはヘルプ文字列を指定する。
この文字列はコマンドラインからhelpオプションを指定した際、この引数の情報として表示される。
## 実行

引数を省略するとプログラムはエラー終了する。

```
$ cargo run -q -- B747
Cli { name: "B747" }
```

ヘルプ画面では、引数名は`<NAME>`と表示される。


`<HELP>`はコマンド名の横に表示されており、省略不能であることがわかる。
`NAME`はコンパイラが`[Derive]`によってソースコードの要素名から類推した引数名である。
また、`<NAME>`の横に`help`メソッドに与えたヘルプ文字列が表示されている。
```
$ cargo run -q -- -h
Usage: airplane <NAME>

Arguments:
  <NAME>  Name of airclaft

Options:
  -h, --help     Print help
  -V, --version  Print version
```