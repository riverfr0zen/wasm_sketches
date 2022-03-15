// Looks like `extern crate` is older syntax and no longer needed
// extern crate wasm_sketches;
use sketches;

fn main() {
    sketches::shifty_circle::app("circle");
}
