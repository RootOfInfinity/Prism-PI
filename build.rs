use std::{env, fs};

fn main() {
    slint_build::compile("ui/main.slint").unwrap();
    let json_file = fs::read_to_string("./levels/hello_one.json")
        .expect("could not find ./levels/hello_one.json");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{}/jsonstuff.rs", out_dir);
    let data = format!("pub const LEVELS: [&str; 1] = [r#\"{}\"#];", json_file);
    fs::write(dest_path, data).unwrap();
}
