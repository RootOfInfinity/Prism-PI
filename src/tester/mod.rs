use super::lang::errors::CompileError;
use json::JsonValue;

pub struct TestResult {
    success: bool,
    errors: Vec<CompileError>,
    correct: u32,
    incorrect: u32,
}

pub fn test_against_json(code: String, json: JsonValue) -> TestResult {
    todo!()
}
