// Looks like `extern crate` is older syntax and no longer needed
// extern crate wasm_sketches;
use bevy_sketches;

fn main() {
    bevy_sketches::shifty_circle::app("circle");
}
