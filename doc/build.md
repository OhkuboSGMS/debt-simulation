# ビルド方法
Windowsでのビルド方法を記録
### 通常のビルド
```shell
cargo build
```

`target/debug/<exe-name>.exe`に実行ファイルが作成される。


### リリース用ビルド

```shell
carge build --release
```

`targetr/release/<exe-name>.exe`に作成。

それ以上の最適化は[`profile`](https://doc.rust-lang.org/cargo/reference/profiles.html)を通して設定を行う
