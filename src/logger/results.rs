#![allow(dead_code)]
pub enum Result {
    Success,
    Failure
}
pub fn is_success(result: Result) -> bool {
    match result {
        Result::Success => true,
        Result::Failure => false
    }
}