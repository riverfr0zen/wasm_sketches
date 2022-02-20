use clap::Parser;
use std::process::{ Command, Stdio };


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the example to build
    #[clap(short, long, default_value = "All")]
    sketch: String,
}


fn build_sketch(sketch: &str) {
    println!("Building {}...", sketch);
    // In the previous commit I was following this example
    // https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#continuously-process-child-process-outputs
    //
    // ...but I couldn't figure out how to get the exit code. Then I found this example
    // which I think serves this use case better:
    // https://stackoverflow.com/a/32020376/4655636
    let mut build_cmd = Command::new("cargo")
        .arg("build")
        .arg("--example")
        .arg(sketch)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let build_success = build_cmd.wait().unwrap().success();
    if build_success {
        println!("Running wasm-bindgen for {}...", sketch);
        Command::new("wasm-bindgen")
            .arg("--out-dir")
            .arg("www/wasms")
            .arg("--target")
            .arg("web")
            .arg(format!("target/wasm32-unknown-unknown/release/examples/{}.wasm", sketch))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
    
    }
}


#[allow(dead_code)]
fn main() {
    let args = Args::parse();

    if args.sketch == "All" {
        println!("TODO: Go through all examples and build sketches")
    } else {
        build_sketch(args.sketch.as_str());
    }
}