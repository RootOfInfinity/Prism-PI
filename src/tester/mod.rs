use std::{fs, time::Duration};

use crate::lang::{
    errors::CompileError,
    run_code_timed,
    tokens::{Literal, Type},
};

use json::JsonValue;
use slint::format;

#[derive(Debug)]
pub struct TestResult {
    pub success: bool,
    pub errors: Vec<CompileError>,
    pub correct: u32,
    pub incorrect: u32,
}

pub struct TestInfo {
    pub code: String,
    pub inputs_type: Vec<Type>,
    pub output_type: Type,
    pub json: JsonValue,
}

include!(concat!(env!("OUT_DIR"), "/jsonstuff.rs"));

/// This function assumes an entry point of 'fun test(inputs) -> output'.
/// The json also needs to be formatted in a specific way. There must be a top
/// level value "tests" that holds an array of objects. These objects must have a
/// value "inputs" which holds an array of data, and a value "output" wich holds one
/// piece of data.
pub fn test_against_json(data: TestInfo) -> TestResult {
    let tests = data.json["tests"].clone();
    let length: usize = tests[0].dump().parse().unwrap();
    let mut correct = 0;
    let mut errors = Vec::new();
    for i in 1..length {
        let length: usize = tests[i]["inputs"][0].dump().parse().unwrap();
        let mut inputs_string = String::new();
        for j in 1..length {
            inputs_string.extend(tests[i]["inputs"][j].dump().chars());
            inputs_string += ", ";
        }
        if inputs_string.len() > 0 {
            inputs_string.pop();
        }
        let output = tests[i]["inputs"][0].dump();
        let code_to_add = format!(
            "fun main() -> int {{ if test({}) == {} {{ return 0; }} return 1; }}",
            inputs_string, output
        );
        let new_code = data.code.clone() + code_to_add.as_str();
        let res = run_code_timed(new_code, Duration::from_secs(2), Duration::from_millis(100));
        match res {
            Ok(Some(x)) if x == 0 => {
                correct += 1;
            }
            Err(e) if errors.len() == 0 => {
                errors.extend(e);
            }
            _ => (),
        }
    }
    return TestResult {
        success: correct == length - 1,
        errors,
        correct: correct as u32,
        incorrect: (length - 1 - correct) as u32,
    };
}

/*

{
    "tests": [
        len, in this case 3
        {
            "inputs": [
                len,
                input,
                input,
                input,
            ],
            "output": output,
        },
        {
            same stuff as right above
        }
    ]
}

*/
