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
wasm-bindgen --out-dir bevy_sketches/www/wasms --target web target/wasm32-unknown-unknown/release/examples/shiftyc.wasm
```

For quick copy:

```
circle:
cargo build --release --example shiftyc --target wasm32-unknown-unknown
wasm-bindgen --out-dir bevy_sketches/www/wasms --target web target/wasm32-unknown-unknown/release/examples/shiftyc.wasm


ufo:
cargo build --release --example shiftyufo --target wasm32-unknown-unknown
wasm-bindgen --out-dir bevy_sketches/www/wasms --target web target/wasm32-unknown-unknown/release/examples/shiftyufo.wasm

rect:
cargo build --release --example shiftyrect --target wasm32-unknown-unknown
wasm-bindgen --out-dir bevy_sketches/www/wasms --target web target/wasm32-unknown-unknown/release/examples/shiftyrect.wasm
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

# Display DEBUG level logs
cargo run -- --debuglog --sketch shiftyc
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

## Rust with Visual Studio Code

VSCode has also [provided a topic](https://code.visualstudio.com/docs/languages/rust) around using Rust with VSCode. This came out after I had already done my set up, and I think it covers much of what I have already done. However, I'm leaving this link here in case there are any details I may want to revisit, especially around debugging.

The [rust-analyzer manual](https://rust-analyzer.github.io/manual.html#vs-code-2) also shows a lot of nifty config tips for VSCode.


## Android Chrome Developer Tools

1. In Android Settings, search for "Build Number" and click several times to set phone to Developer mode
2. Connect phone to PC via USB.
3. Open Chrome and navigate to `chrome://inspect/#devices`. Phone should be listed there. It's usually kind of wonky, so tinker around and/or give it a few moments if you don't see it right away.


# Troubleshooting

## bindgen format mismatch

After running `rustup update` since some time with the project, I got this error when running the `build-sketches` tool:

```
Building cellular...
    Finished release [optimized] target(s) in 0.11s
Running wasm-bindgen for cellular...
error: 

it looks like the Rust project used to create this wasm file was linked against
version of wasm-bindgen that uses a different bindgen format than this binary:

  rust wasm file schema version: 0.2.83
     this binary schema version: 0.2.79

Currently the bindgen format is unstable enough that these two schema versions
must exactly match. You can accomplish this by either updating the wasm-bindgen
dependency or this binary.

You should be able to update the wasm-bindgen dependency with:

    cargo update -p wasm-bindgen

or you can update the binary with

    cargo install -f wasm-bindgen-cli

if this warning fails to go away though and you're not sure what to do feel free
to open an issue at https://github.com/rustwasm/wasm-bindgen/issues!

```

Solution: Basically just had to update `wasm-bindgen-cli` (which is installed on the user level, not the project level) with the command `cargo install -f wasm-bindgen-cli`, as instructed in the message.
