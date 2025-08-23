use super::lang::run_code;
slint::include_modules!();
pub fn run_gui_test(args: Vec<String>) -> Result<(), slint::PlatformError> {
    // main func for gui tests
    println!("gui stuff");
    let main_window = MainWindow::new()?;
    // let main_window_weak = main_window.as_weak();
    main_window.on_run_freestyle_code(move |code| match run_code(code.try_into().unwrap()) {
        Ok(int) => {
            return format!("Returned value: {}", int).try_into().unwrap();
        }
        Err(e) => {
            return format!("{:#?}", e).try_into().unwrap();
        }
    });

    main_window.run()
}
