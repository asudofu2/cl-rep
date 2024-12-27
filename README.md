# cl-rep

クリップボードの文字列を置換しクリップボードに追加する。
置換するパターンはコマンドライン引数でパターンファイルを指定する。

## コマンド例

```sh
Usage: cl-rep.exe [OPTIONS]

Options:
  -p, --pattern-file <PATTERN_FILE>  [default: cl-rep-patt.json]
  -h, --help                         Print help
  -V, --version                      Print version
```

## パターンファイル

srcに変換元、dstに変換先を指定する。

```json
[
    {
        "src": "a src string",
        "dst": "a dst string"
    },
    {
        "src": "b src string",
        "dst": "b dst string"
    }
]
```
