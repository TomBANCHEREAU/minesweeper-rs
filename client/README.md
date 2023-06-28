

## dev
cargo install cargo-watch
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install http-server

```
cargo watch -c -s "cargo build && wasm-pack build --target web && http-server"
```

