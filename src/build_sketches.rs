use clap::Parser;
use std::process::{ Command, Stdio };
use std::fs;


fn gen_html_from_template(sketch: &str) {
    let file_contents = fs::read_to_string(
        "www/window-matching-canvas.template.html"
    ).expect("Unable to read file");

    let file_contents = file_contents.replace("{{sketch}}", sketch);
    fs::write(format!("www/{}.html", sketch), file_contents).expect("Unable to write file");
}


fn build_sketch(sketch: &str) {
    println!("Building {}...", sketch);
    // In the previous commit I was following this example
    // https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#continuously-process-child-process-outputs
    //
    // ...but I couldn't figure out how to get the exit code. Then I found this example
    // using Stdio::inherit which I think serves this use case better:
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

    if ! build_cmd.wait().unwrap().success() {
        return;
    }

    println!("Running wasm-bindgen for {}...", sketch);
    let mut wasm_bgen_cmd = Command::new("wasm-bindgen")
        .arg("--out-dir")
        .arg("www/wasms")
        .arg("--target")
        .arg("web")
        .arg(format!("target/wasm32-unknown-unknown/release/examples/{}.wasm", sketch))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    if ! wasm_bgen_cmd.wait().unwrap().success() {
        return;
    }

    println!("Create html from template");
    gen_html_from_template(sketch);
}


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the example to build
    #[clap(short, long)]
    sketch: Option<String>,
}


#[allow(dead_code)]
fn main() {
    let args = Args::parse();

    match args.sketch {
        Some(sketch) => build_sketch(&sketch),
        None => println!("TODO: Go through all examples and build sketches"),
    }
}