use clap::Parser;
use std::process::{ Command, Stdio };
use std::fs;
use std::path::Path;
use serde_json::json;
use serde_json::{Result, Value};

const CANVAS_HTML_TPL: &str = "www/window-matching-canvas.template.html";
const JSON_LIST: &str = "www/sketches.json";


fn gen_html_from_template(sketch: &str) {
    let file_contents = fs::read_to_string(
        CANVAS_HTML_TPL
    ).expect("Unable to read file");

    let file_contents = file_contents.replace("{{sketch}}", sketch);
    fs::write(format!("www/{}.html", sketch), file_contents).expect("Unable to write html file");
}


fn add_to_sketch_to_json_list(sketch: &str) -> Result<()> {
    let list_path = Path::new(JSON_LIST);

    if !list_path.exists() {
        println!("JSON list doesn't exist, creating...");
        // Based on: https://github.com/serde-rs/json#constructing-json-values
        let new_list = json!({ "sketches": [sketch] });
        fs::write(JSON_LIST, new_list.to_string()).expect("Unable to write json list");
    } else {
        let json_list = fs::read_to_string(JSON_LIST).expect("Unable to read json list");
        let mut json_list: Value = serde_json::from_str(&json_list).expect("Failed to get JSON from file");
        if let Some(sketches)  = json_list["sketches"].as_array_mut() {
            let sketch_json = & json!(sketch);
            if ! sketches.contains(sketch_json) {
                sketches.push(json!(sketch_json));
            }
        }
        // println!("{}", json_list.to_string());
        fs::write(JSON_LIST, json_list.to_string()).expect("Unable to rewrite json list");        
    }
    return Ok(());
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
    add_to_sketch_to_json_list(sketch).expect("Could not add sketch to json list");
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