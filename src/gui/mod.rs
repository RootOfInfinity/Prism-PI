use std::{str::FromStr, thread, vec};

use slint::{invoke_from_event_loop, Color, Model};

use super::lang::run_code;
slint::include_modules!();
pub fn run_gui_test(args: Vec<String>) -> Result<(), slint::PlatformError> {
    // main func for gui tests
    println!("gui stuff");
    let main_window = MainWindow::new()?;
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
    println!("new block");
    let mut blocks: Vec<BlockData> = main_window.get_blocks().iter().collect();
    blocks.push(BlockData { block_color: Color::from_rgb_u8(255, 0, 0), block_name: String::from_str("Another Block").unwrap().into(), block_width: 130.0 });
    let out_blocks = std::rc::Rc::new(slint::VecModel::from(blocks));
    main_window.set_blocks(out_blocks.into());
    main_window.on_summon_block( || {
        println!("new block");
        //figure out how to add blocks
        });

    main_window.run()
}
