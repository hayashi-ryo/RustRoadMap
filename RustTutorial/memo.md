# The Rust Programming Language

[公式ドキュメント](https://doc.rust-jp.rs/book-ja/)を読み、理解した内容について記録していく。

## 開発環境

vscode上で色々試していくので、今回はdevcontainer上にMS公式のRustイメージを利用して色々試すことにした。

## メモ

- Rustはデフォルトで変数がimmutable(不変)。可変変数として定義したい場合は、名義的に`mut`とする必要がある。