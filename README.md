# mi7-reverse
内网代码小工具

cargo run --bin main -- client --server-addr 47.116.196.247:7000 --target-addr 127.0.0.1:9502 --product-id 1
cargo run --bin main -- client --server-addr 0.0.0.0:7000 --target-addr 0.0.0.0:9502 --product-id 12345
cargo run --bin main -- client --product-id 12345

cargo run --bin main -- server --client-addr 0.0.0.0:7000 --user-addr 0.0.0.0:8080 --product-id 12345
cargo run --bin main -- server --product-id 12345


cargo run --bin main -- server

cargo run --bin main -- client --product-id