# shiftyc

```
cargo build --release --example shiftyc --target wasm32-unknown-unknown
```

-OR-

```
cargo build --release --example shiftyc --target wasm32-unknown-unknown --features=framestats
```


```
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/shiftyc.wasm
```