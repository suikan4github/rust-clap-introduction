# 必須引数

省略することができない引数の処理。

## ソースコード

コマンドライン構造体の要素に`#[arg(required = true)]`属性を付けると省略不能な引数として扱われる。
また、この場合の引数の値は文字列として処理される。

```
struct Cli {
    #[arg(required = true)]
    name: String,
}
```

## 実行

引数を省略するとプログラムはエラー終了する。

```
$ cargo run -q -- B747
Cli { name: "B747" }
```

ヘルプ画面では、引数名は`<NAME>`と表示される。
`<>`は省略不能であることを表す。
`NAME`はコンパイラが`[Derive]`によってソースコードの要素名から類推した引数名である。
```
$ cargo run -q -- -h
Usage: airplane <NAME>

Arguments:
  <NAME>  

Options:
  -h, --help     Print help
  -V, --version  Print version
```