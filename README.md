# pipgre

**pipe + grep** — パイプラインに特化した行フィルタ。

「この文字列を含む」「この文字列を含まない」を好きなだけ並べて、1コマンドで絞り込めます。
正規表現ではなく単純な文字列マッチなので、エスケープ不要でそのまま書けます。

```shell
# before: grep を何本もつなぐ
cat log.txt | grep ERROR | grep timeout | grep -v healthcheck | grep -v retry

# after: pipgre なら1行
cat log.txt | pipgre ERROR timeout -V healthcheck retry
```

## Install

```shell
$ git clone https://github.com/rytkmt/pipgre.git
$ cd pipgre
$ cargo build --release
$ cp target/release/pipgre ~/.local/bin/
```

## Usage

```shell
# "foo" を含む行を抽出
cat file.txt | pipgre foo

# "foo" AND "bar" を含む行を抽出
cat file.txt | pipgre foo bar

# "foo" を含むが "baz" を含まない行
cat file.txt | pipgre foo -v baz

# 複数の除外条件
cat file.txt | pipgre foo -v baz -v qux

# -V: 以降をすべて除外条件にする
cat file.txt | pipgre foo bar -V baz qux piyo

# -G: -Vを解除してincludeに戻す
cat file.txt | pipgre -V exclude1 exclude2 -G include1
```

## Options

| Option | Description |
|--------|-------------|
| `<keyword>` | include条件（複数指定でAND） |
| `-v <keyword>` | 直後の1つをexclude条件にする |
| `-V` | 以降の文字列をすべてexclude条件にする |
| `-G` | `-V`を解除し、以降をinclude条件に戻す |

## Examples

### ログ解析

```shell
# ERRORかつtimeoutを含む行から、healthcheckとretryを除外
cat app.log | pipgre ERROR timeout -V healthcheck retry

# 特定ユーザーの400番台エラーを抽出
tail -f access.log | pipgre user_id=123 -v 200 -v 301
```

### プロセス管理

```shell
# rubyプロセスからbundleとspring以外を探す
ps aux | pipgre ruby -V bundle spring

# 特定ポートを使っているプロセスを探す
lsof -i | pipgre LISTEN 3000 -v node
```

### ファイル検索

```shell
# specファイルでmodelに関連するものだけ、factory以外
find . -name "*.rb" | pipgre spec model -v factory

# git差分から特定の変更を絞り込む
git log --oneline | pipgre fix -V merge typo
```

### Docker

```shell
# 動作中のコンテナからwebに関連するものを抽出、helperは除外
docker ps | pipgre web Up -v helper
```
