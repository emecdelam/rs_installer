use std::fs;
use std::path::Path;
use crate::logger::{*, results::Result};
#[allow(dead_code)]
pub fn move_file(base: impl AsRef<Path>, out: impl AsRef<Path>) -> results::Result {
    let base = base.as_ref();
    let out = out.as_ref();

    // Rest of the code remains the same
    logs::log(&format!("Trying to move {:?} to {:?}", base, out), colors::LogLevel::Attempt);
    if out.is_dir() {
        let file_name = base.file_name().unwrap().to_str().unwrap();
        let mut out = out.to_path_buf(); 
        out.push(file_name);
        logs::log(out.to_str().unwrap(), colors::LogLevel::Debug)
    }
    match fs::rename(base, out) {
        Ok(_) => {
            logs::log(&format!("Successfully moved {:?} to {:?}", base, out), colors::LogLevel::Success);
            Result::Success
        },
        Err(err) => {
            logs::log(&format!("Error moving file: {:?}", err.to_string()), colors::LogLevel::Error);
            Result::Failure
        }
    }
}
#[allow(dead_code)]
pub fn file_is_in_dir(file: impl AsRef<Path>, dir: impl AsRef<Path>) -> bool {
    let file_name = file.as_ref().file_name().unwrap().to_str().unwrap();
    let mut dir = dir.as_ref().to_path_buf(); 
    dir.push(file_name);
    dir.exists()
}