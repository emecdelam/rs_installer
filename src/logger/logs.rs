use crate::logger::colors::{Colors, LogLevel};

#[allow(dead_code)]
fn print_color(message: String, color1: &str, color2 : Option<&str>) {
    if color2.is_some() {
        println!("{}{}{}{}", color1,color2.unwrap(), message, Colors::RESET);
        return;
    }
    else {
        println!("{}{}{}", color1, message, Colors::RESET);
    }
}
#[allow(dead_code)]
fn ljust(content: &str, length: usize) -> String {
    let mut padded_content = String::from(content);
    for _ in 0..(length - content.len()) {
        padded_content.push(' ');
    }
    format!("[{}]", padded_content)
}
#[allow(dead_code)]
fn log_message_builder(prefix : String, message : &str) -> String{
    format!("{} : {}", prefix, message)
}

pub fn log(message: &str, level: LogLevel) {
    match level {
        LogLevel::Info => print_color(log_message_builder(ljust("INFO",10), message), Colors::BLUE, None),
        LogLevel::Warn => print_color(log_message_builder(ljust("WARN",10), message), Colors::YELLOW, None),
        LogLevel::Error => print_color(log_message_builder(ljust("ERROR",10), message), Colors::RED, None),
        LogLevel::Critical => print_color(log_message_builder(ljust("CRITICAL",10), message), Colors::BG_ORANGE, None),
        LogLevel::Debug => print_color(log_message_builder(ljust("DEBUG",10), message), Colors::BOLD, None),
        LogLevel::Attempt => print_color(log_message_builder(ljust("ATTEMPT",10), message), Colors::CYAN, None),
        LogLevel::Success => print_color(log_message_builder(ljust("SUCCESS",10), message), Colors::GREEN, None),
    }
}