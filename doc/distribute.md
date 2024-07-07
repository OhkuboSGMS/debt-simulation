# インストーラーの作成方法

## Windows

### Install

* [wix_v3](https://wixtoolset.org/docs/wix3/)

* `cargo install cargo-wix`

https://github.com/volks73/cargo-wix

Wix_v4にはcargo_wixが未対応だったのでv3を利用する。
### Init Setting

* `cargo wix init`

### Build

* `cargo wix`

`target/wix`にインストーラが生成される
