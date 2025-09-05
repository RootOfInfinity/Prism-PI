use std::env;

mod gui;
mod lang;
mod tester;

fn main() -> Result<(), slint::PlatformError> {
    // println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Use with argument 'lang' or 'gui'");
        println!("If using 'cargo run', the command would look like:");
        println!("cargo run -- gui");
    } else {
        match args[1].as_str() {
            "lang" => crate::lang::run_lang_test(args),
            "gui" => crate::gui::run_gui_test(args)?,
            x => println!("Invalid argument: {}", x),
        }
    }
    Ok(())
}
