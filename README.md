## https简单代理工具，练习

练习项目

开一个终端

```sh
cargo run --example client
```

再开一个终端

```sh
cargo run
```

开另一个终端

```sh
export https_proxy=127.0.0.1:9000
export http_proxy=127.0.0.1:9000
```

然后在第三个终端尝试curl
