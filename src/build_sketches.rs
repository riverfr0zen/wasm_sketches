use clap::Parser;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the example to build
    #[clap(short, long, default_value = "All")]
    sketch: String,
}


fn build_sketch(sketch: &str) -> Result<(), Error> {
    println!("Building {}...", sketch);
    // Get streamed response, based on:
    // https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#continuously-process-child-process-outputs
    let build_cmd = Command::new("cargo")
        .arg("build")
        .arg("--example")
        .arg(sketch)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

    let reader = BufReader::new(build_cmd);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));


    println!("Running wasm-bindgen for {}...", sketch);
    let wasm_cmd = Command::new("wasm-bindgen")
        .arg("--out-dir")
        .arg("www/wasms")
        .arg("--target")
        .arg("web")
        .arg(format!("target/wasm32-unknown-unknown/release/examples/{}.wasm", sketch))
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

    let reader = BufReader::new(wasm_cmd);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("hello {}", line));

    Ok(())
}


#[allow(dead_code)]
fn main() {
    let args = Args::parse();

    if args.sketch == "All" {
        println!("TODO: Go through all examples and build sketches")
    } else {
        // Error handling based on:
        // https://stackoverflow.com/a/53368681/4655636
        match build_sketch(args.sketch.as_str()) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }
}