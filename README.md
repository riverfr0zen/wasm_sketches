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


# Tooling notes

Much of the tooling setup here is from my [rust learning project](https://github.com/riverfr0zen/riverfr0zen-learns-rust). Items below are in addition to that.


## formatOnSave for rust code

Added the following to VSCode workspace settings:

```
		"[rust]": {
			"editor.defaultFormatter": "matklad.rust-analyzer",
			"editor.formatOnSave": true,
		},
```

`rust-analyzer` is using rustfmt, and this can be configured by adding a `.rustfmt.toml` file in the project folder. I've heard that using the `rustfmt` defaults is considered "best practice", so I'll be sticking to them with very few overrides (coming from Python, I simply must separate some things with at least two lines =).


## Using unstable rustfmt features

Many useful rustfmt features are currently in "Unstable" status and only available in nightly builds. It is possible to use these "nightly" features for rustfmt only (i.e., stable rustc with "nightly" rustfmt):

Based on: https://github.com/rust-lang/vscode-rust/issues/438#issuecomment-1003671382


1. Install the nightly toolchain

`rustup toolchain install nightly`

2. Pass the "+nightly" arg to rustfmt by modifying VSCode settings (I added to my VSCode workspace settings):

```
"rust-analyzer.rustfmt.extraArgs": [
        "+nightly"
]
```

