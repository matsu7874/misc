# Toy Photo Album

データベースもWebサーバも不要の写真共有アプリです。

## How To Use

```sh
cargo run --release
```

8080ポートでアプリケーションが立ち上がります。


## 依存ライブラリ

- actix-web = "4"
  - Webアプリケーションフレームワーク
- actix-files = "0.6.0"
  - 静的ファイルをactix-web経由で返却するために利用
- actix-multipart = "0.4.0"
  - アップロードされるファイルを受け取るために利用
- futures-util = { version = "0.3.7", default-features = false, features = ["std"] }
  - steamに対して `try_next` を生やすために利用

- tera = "1.15.0"
  - テンプレートエンジン
    - jinja inspired
- lazy_static = "1.0"
- anyhow = "1.0.57"

