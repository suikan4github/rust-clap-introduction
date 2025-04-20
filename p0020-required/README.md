# バージョン表示プログラム

clapクレートによるバージョン表示プログラム


```
$ cargo run -q -- B747
Cli { name: "B747" }
```

`-h`または`--help`オプションを付けると、ヘルプメッセージが表示される。

```
$ cargo run -q -- -h
Usage: airplane <NAME>

Arguments:
  <NAME>  

Options:
  -h, --help     Print help
  -V, --version  Print version
```