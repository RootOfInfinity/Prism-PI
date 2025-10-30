use std::{
    collections::HashMap,
    ops::DerefMut,
    rc::Rc,
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
    vec,
};

use crate::lang::tokens::Type;
use blockdata::{Assign, Block, BlockType, World, WorldManipulation};
use data2gui::create_blockdata_from_world;
use popup_asker::{Message, ask_popup};
use slint::{Color, ComponentHandle, Model, ToSharedString, format, invoke_from_event_loop};

mod blockdata;
mod data2gui;
mod popup_asker;

use crate::tester::{LEVELS, TestInfo, test_against_json};

use super::lang::run_code;
slint::include_modules!();
pub fn run_gui_test(args: Vec<String>) -> Result<(), slint::PlatformError> {
    // main func for gui tests
    println!("gui stuff");
    let main_window = MainWindow::new()?;

    // all the blocks in the data structure
    let world: Arc<Mutex<World>> = Arc::new(Mutex::new((HashMap::new(), HashMap::new(), 1, 1)));
    // messages to pass from a popup window
    let messages: Arc<Mutex<Message>> = Arc::new(Mutex::new(Message {
        message_type: MessageType::None,
        message_contents: String::new(),
    }));

    let message_clone = Arc::clone(&messages);
    main_window.on_send_message_blocks(move |mtype: MessageType, cont: slint::SharedString| {
        let mut message_lock = message_clone.lock().unwrap();
        let new_message = Message {
            message_type: mtype,
            message_contents: cont.to_string(),
        };
        *message_lock.deref_mut() = new_message;
    });

    // CALLBACK BINDINGS //

    // Running a string of freestyle code and updating
    let main_window_weak = main_window.as_weak();
    main_window.on_run_freestyle_code(move |code| {
        let main_window_weak = main_window_weak.clone();
        thread::spawn(move || {
            let expensive_string = match run_code(code.try_into().unwrap()) {
                Ok(int) => int.to_string(),
                Err(e) => format!("{:#?}", e).to_string(),
            };
            let main_window_clone = main_window_weak.clone();
            invoke_from_event_loop(move || {
                main_window_clone
                    .unwrap()
                    .set_freestyle_string(expensive_string.try_into().unwrap());
            })
        });
    });

    let main_window_weak = main_window.as_weak();
    main_window.on_run_hello_one_test(move |code| {
        let main_window_weak = main_window_weak.clone();
        thread::spawn(move || {
            let info = TestInfo {
                code: code.to_string() + " fun test() -> int { return helloOne(); }",
                inputs_type: vec![],
                output_type: crate::lang::tokens::Type::Int,
                json: json::parse(LEVELS[0]).unwrap(),
            };
            let result = test_against_json(info);
            let mut final_string = String::new();
            if result.success {
                final_string += "You Won!\n";
            } else {
                final_string += "You LOSE! LOL!\n";
            }
            if result.errors.len() > 0 {
                final_string += "Unfortunately, you had some compile errors.\n";
                final_string = format!("{}{:#?}", final_string, result.errors).to_string();
                final_string += "\n";
            }
            final_string.extend(
                format!(
                    "Correct: {} |#| Incorrect: {}",
                    result.correct, result.incorrect
                )
                .chars(),
            );
            let main_window_weak = main_window_weak.clone();
            invoke_from_event_loop(move || {
                main_window_weak
                    .unwrap()
                    .set_hello_one_result(final_string.try_into().unwrap());
            })
        });
    });

    let main_window_weak = main_window.as_weak();
    main_window.on_run_average_nums_test(move |code| {
        let main_window_weak = main_window_weak.clone();
        thread::spawn(move || {
            let info = TestInfo {
                code: code.to_string() + " fun test(dcml num1, dcml num2) -> dcml { return averageDecimals(num1, num2); }",
                inputs_type: vec![Type::Dcml, Type::Dcml],
                output_type: Type::Dcml,
                json: json::parse(LEVELS[1]).unwrap(),
            };
            let result = test_against_json(info);
            let mut final_string = String::new();
            if result.success {
                final_string += "You Won!\n";
            } else {
                final_string += "You LOSE! LOL!\n";
            }
            if result.errors.len() > 0 {
                final_string += "Unfortunately, you had some compile errors.\n";
                final_string = format!("{}{:#?}", final_string, result.errors).to_string();
                final_string += "\n";
            }
            final_string.extend(
                format!(
                    "Correct: {} |#| Incorrect: {}",
                    result.correct, result.incorrect
                )
                .chars(),
            );
            let main_window_weak = main_window_weak.clone();
            invoke_from_event_loop(move || {
                main_window_weak
                    .unwrap()
                    .set_average_nums_result(final_string.try_into().unwrap());
            })
        });
    });

    let main_window_weak = main_window.as_weak();
    main_window.on_run_fibonacci_test(move |code| {
        let main_window_weak = main_window_weak.clone();
        thread::spawn(move || {
            let info = TestInfo {
                code: code.to_string() + " fun test(int num) -> int { return fib(num); }",
                inputs_type: vec![],
                output_type: crate::lang::tokens::Type::Int,
                json: json::parse(LEVELS[2]).unwrap(),
            };
            let result = test_against_json(info);
            let mut final_string = String::new();
            if result.success {
                final_string += "You Won!\n";
            } else {
                final_string += "You LOSE! LOL!\n";
            }
            if result.errors.len() > 0 {
                final_string += "Unfortunately, you had some compile errors.\n";
                final_string = format!("{}{:#?}", final_string, result.errors).to_string();
                final_string += "\n";
            }
            final_string.extend(
                format!(
                    "Correct: {} |#| Incorrect: {}",
                    result.correct, result.incorrect
                )
                .chars(),
            );
            let main_window_weak = main_window_weak.clone();
            invoke_from_event_loop(move || {
                main_window_weak
                    .unwrap()
                    .set_fib_result(final_string.try_into().unwrap());
            })
        });
    });

    // Summoning a block without defined features
    let message_clone = Arc::clone(&messages);
    let main_window_weak = main_window.as_weak();
    let world_clone = Arc::clone(&world);
    main_window.on_summon_block(move |type_of_block| {
        match type_of_block {
            SlintBlockType::Declaration => {}
            SlintBlockType::Expression => {
                ask_popup(
                    Message {
                        message_type: MessageType::ExprExpr,
                        message_contents: String::from(
                            "Please put in expression for the expression block.",
                        ),
                    },
                    &main_window_weak,
                );

                let message_clone = Arc::clone(&message_clone);
                let world_clone = Arc::clone(&world_clone);
                let main_window_weak = main_window_weak.clone();
                thread::spawn(move || {
                    let response = loop {
                        std::thread::sleep(Duration::from_millis(100));
                        let messagelock = message_clone.lock().unwrap();
                        if let MessageType::ExprExpr = messagelock.message_type {
                            break messagelock.message_contents.clone();
                        }
                    };
                    let mut messagelock = message_clone.lock().unwrap();
                    *messagelock.deref_mut() = Message {
                        message_type: MessageType::None,
                        message_contents: String::new(),
                    };
                    let mut lock = world_clone.lock().unwrap();
                    let current_id = lock.3;
                    lock.0.insert(
                        current_id,
                        Block {
                            btype: BlockType::Expression(response),
                            id: current_id,
                            next: 0,
                            loc: (300, 300),
                            is_root: true,
                            length: 126 / 2,
                        },
                    );
                    lock.3 += 1;
                    std::mem::drop(messagelock);
                    let main_window_weak = main_window_weak.clone();
                    let world_clone = Arc::clone(&world_clone);
                    invoke_from_event_loop(move || {
                        main_window_weak.unwrap().set_blocks(
                            Rc::new(create_blockdata_from_world(
                                &mut world_clone.lock().unwrap(),
                            ))
                            .into(),
                        );
                    })
                    .unwrap();
                });

                // ask for the popup, then wait until message is filled.
            }
            _ => return,
        }
    });

    let main_window_weak = main_window.as_weak();
    let world_clone = Arc::clone(&world);
    main_window.on_move_fs_block(move |id, x, y| {
        world_clone
            .lock()
            .unwrap()
            .move_block(id as u64, x as u64, y as u64);

        let main_window_weak = main_window_weak.clone();
        let world_clone = Arc::clone(&world_clone);
        invoke_from_event_loop(move || {
            main_window_weak.unwrap().set_blocks(
                Rc::new(create_blockdata_from_world(
                    &mut world_clone.lock().unwrap(),
                ))
                .into(),
            );
        })
        .unwrap()
    });

    main_window.run()
}
