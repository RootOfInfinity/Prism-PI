use std::{rc::Rc, str::FromStr, thread, vec};

use slint::{Color, ComponentHandle, Model, invoke_from_event_loop};

use super::lang::run_code;
slint::include_modules!();
pub fn run_gui_test(args: Vec<String>) -> Result<(), slint::PlatformError> {
    // main func for gui tests
    println!("gui stuff");
    let main_window = MainWindow::new()?;

    // CALLBACK BINDINGS //

    // Running a string of freestyle code and updating
    let main_window_weak = main_window.as_weak();
    main_window.on_run_freestyle_code(move |code| {
        let main_window_weak = main_window_weak.clone();
        thread::spawn(move || {
            let expensive_string = match run_code(code.try_into().unwrap()) {
                Ok(int) => int.to_string(),
                Err(e) => format!("{:#?}", e),
            };
            let main_window_clone = main_window_weak.clone();
            invoke_from_event_loop(move || {
                main_window_clone
                    .unwrap()
                    .set_freestyle_string(expensive_string.try_into().unwrap());
            })
        });
    });

    // Summoning a block without defined features
    let main_window_weak = main_window.as_weak();
    main_window.on_summon_block(move || {
        let main_window_weak = main_window_weak.clone();
        println!("new block");
        //figure out how to add blocks
        let mut current_blocks: Vec<BlockData> =
            main_window_weak.unwrap().get_blocks().iter().collect();
        current_blocks.push(BlockData {
            block_color: Color::from_rgb_u8(0, 255, 0),
            block_name: "Spawn'd Block".into(),
            block_width: 150,
            code: "no code :)".into(),
        });
        invoke_from_event_loop(move || {
            main_window_weak
                .unwrap()
                .set_blocks(Rc::new(slint::VecModel::from(current_blocks)).into());
        })
        .unwrap();
    });

    main_window.run()
}
