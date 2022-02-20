use clap::Parser;
use std::process::Command;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the example to build
    #[clap(short, long, default_value = "All")]
    sketch: String,
}


fn build_sketch(sketch: &str) {
    println!("Building {}...", sketch);
    // cargo build --release --example shiftyc --target wasm32-unknown-unknown
    let build_cmd = Command::new("cargo")
        .arg("build")
        .arg("--example")
        .arg(sketch)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .output()
        .expect("failed to execute process");
    println!("status: {}", build_cmd.status);
    let ls = build_cmd.stdout;
    println!("umm {}", String::from_utf8(ls).unwrap());
    let err = build_cmd.stderr;
    println!("{}", String::from_utf8(err).unwrap());
}


#[allow(dead_code)]
fn main() {
    let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.example)
    // }
    if args.sketch == "All" {
        println!("TODO: Go through all examples and build sketches")
    } else {
        build_sketch(args.sketch.as_str());
        // println!("TODO: Build the example sketch {}", args.sketch);
    }
}