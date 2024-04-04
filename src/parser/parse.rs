use std::fs::File;
use std::io::{Read, Write};
use installer::logger::{logs::log, colors::LogLevel,results::is_success, results};
use installer::utils::file::{self, move_file};
use std::io::Error as IOError;
pub fn read_file(file_name: &str) -> Option<String> {
    log(&format!("Attempting to read file : {}",file_name), LogLevel::Attempt);
    match File::open(file_name) {
        Ok(mut data_file) => {
            let mut file_content = String::new();
            match data_file.read_to_string(&mut file_content) {
                Ok(_) => Some(file_content),
                Err(err) => {
                    log(&format!("Error 1 reading file: {:?}", err.to_string()), LogLevel::Error);
                    None
                
                }
            }
        },
        Err(err) => {
            log(&format!("Error 2 reading file: {:?}", err.to_string()), LogLevel::Error);
            None
        }
    }
}
pub fn append_file(file_name: &str, content: &str) -> results::Result{
    log(&format!("Attempting to append to file : {}",file_name), LogLevel::Attempt);
    match File::open(file_name) {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(_) => results::Result::Success,
            Err(err) => {
                log(&format!("Error 1 appending file: {:?}", err.to_string()), LogLevel::Error);
                results::Result::Failure
            }
        },
        Err(err) => {
            log(&format!("Error 2 appending file: {:?}", err.to_string()), LogLevel::Error);
            results::Result::Failure
        }
    }
}
pub fn parse_line(line :&str) -> (String,String){
    let line_without_first_char: String = line.chars().skip(1).collect();
    let mut first_arg: String = String::new();
    let mut second_arg: String = String::new();
    let mut count:u8 = 0;
    for i in line_without_first_char.split_whitespace() {
        count += 1;
        for j in i.chars(){
            if j == '"' {
                continue;
            }
            if count == 1 {
                first_arg.push(j);
            } else if count == 2 {
                second_arg.push(j);
            } else {
                log("Trying to parse 3 arguments with code 0, asking to move a file/dir to a place but giving more than 2 parameters, inputfile is wrong", LogLevel::Error);
            }
        }
    }
    if count < 2 {
        log("Trying to parse 1 argument with code 0, asking to move a file/dir to a place but giving less than 2 parameter, inputfile is wrong", LogLevel::Error);
    }
    return (first_arg,second_arg);
    
}
pub fn execute_move_order(line:&str){
    let (from, to) = parse_line(line);
    if !is_success(move_file(&from, &to)){
        log("Aborting due to previous error", LogLevel::Critical);
        panic!("Exited with code 1");
    };
}
pub fn execute_write_order(line:&str){
    let (from, to) = parse_line(line);
    let file_path = "inputs/".to_string() + &from;
    let file_result = read_file(&file_path);
    match file_result {
        Some(content) => {
            if results::is_success(append_file(&to, &content)){
                log("Successfully appended file", LogLevel::Success);
            } else {
                log("Aborting due to previous error", LogLevel::Critical);
                panic!("Exited with code 2");
            }
        },
        None => {
            log("Aborting due to previous error", LogLevel::Critical);
            panic!("Exited with code 3");
        }
    }

}

pub fn parse_file(file_name: &str) {
    let file_content = read_file(file_name);
    match file_content {
        Some(content) => {
            let lines = content.lines();
            for line in lines {
                match line.chars().nth(0).unwrap() {
                    '0' => execute_move_order(line),
                    '1' => execute_write_order(line),
                    _ => {
                        log("Unfound option indicator,passing", LogLevel::Error);
                    },
                }
            }
        },
        None => {
            log("Aborting due to previous error, this can be caused due to no input file", LogLevel::Critical);
            panic!("Exited with code 4");
        }
    }
    
}
