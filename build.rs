use std::{env, fs};

fn main() {
    slint_build::compile("ui/main.slint").unwrap();
    let hello_one = fs::read_to_string("./levels/hello_one.json")
        .expect("Could not find ./levels/hello_one.json");
    let average_numbers = fs::read_to_string("./levels/average_nums.json")
        .expect("Could not find ./levels/average_nums.json");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{}/jsonstuff.rs", out_dir);
    let data = format!(
        "pub const LEVELS: [&str; 2] = [r#\"{}\"#, r#\"{}\"#];",
        hello_one, average_numbers
    );
    fs::write(dest_path, data).unwrap();
}
