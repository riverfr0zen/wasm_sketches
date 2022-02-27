This repository contains graphics "sketches" I am learning to make in Rust. 

Currently I'm mainly working with [Bevy Engine](https://bevyengine.org), but I expect I will branch out to other things too.

Note that I am also new to Rust itself, so expect at least some "wtfs" herein =). Any feedback welcome! 


# shiftyc

Live examples so far:

* "shifty circle" variations
    * [shiftyc](https://irfanbaig.com/shiftyc.html)
    * [shiftyrect](https://irfanbaig.com/shiftyrect.html)
    * [shiftufo](https://irfanbaig.com/shiftyufo.html)


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

rect:
cargo build --release --example shiftyrect --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/shiftyrect.wasm
```


# build-sketches tool

## Run with cargo (debug)

Notice the omission of the binary name. I haven't learned yet what happens if there are multiple binaries.

```
# Get help
cargo run -- --help

# Build all sketches
cargo run -- 

#Limit building to only one sketch:
cargo run -- --sketch shiftyc

# Skip generation of html file
cargo run -- --no-html --sketch shiftyc

# Enable frame stats log
cargo run -- --framestats --sketch shiftyc

```

## Run with cargo (release)

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


