use crate::lang::{errors::CompileError, tokens::Literal, tokens::Type};

use json::JsonValue;

pub struct TestResult {
    success: bool,
    errors: Vec<CompileError>,
    correct: u32,
    incorrect: u32,
}

pub struct TestInfo {
    code: String,
    inputs_type: Vec<Type>,
    output_type: Type,
    json: JsonValue,
}

/// This function assumes an entry point of 'fun test(inputs, output) -> bool'.
/// It will return true if the function ran returns the right output with the inputs.
/// The json also needs to be formatted in a specific way. There must be a top
/// level value "tests" that holds an array of objects. These objects must have a
/// value "inputs" which holds an array of data, and a value "output" wich holds one
/// piece of data.
pub fn test_against_json(data: TestInfo) -> TestResult {
    todo!()
}
