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

For quick copy:

```
circle:
cargo build --release --example shiftyc --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/shiftyc.wasm


ufo:
cargo build --release --example shiftyufo --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/shiftyufo.wasm

```


# build-sketches tool

## Run with cargo

```
cargo run --release --bin build-sketches
```

## Build

```
cargo build --release --bin build-sketches
```

The executable will then be at `target/release/build-sketches`.

You can then run `strip` on the executable to [minimize the binary size](https://github.com/johnthagen/min-sized-rust). 


```
strip target/release/build-sketches
```

@TODO: [There is a way to set Cargo to strip automatically on build](https://doc.rust-lang.org/cargo/reference/unstable.html#profile-strip-option), but it is apparently still in the nightly build of Cargo only, so something to revisit in the future. 


