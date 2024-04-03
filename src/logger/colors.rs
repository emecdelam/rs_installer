
#![allow(dead_code)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Critical,
    Debug,
    Success,
    Attempt
}
pub struct Colors;

impl Colors {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[01m";
    pub const DISABLE: &'static str = "\x1b[02m";
    pub const UNDERLINE: &'static str = "\x1b[04m";
    pub const REVERSE: &'static str = "\x1b[07m";
    pub const STRIKETHROUGH: &'static str = "\x1b[09m";
    pub const INVISIBLE: &'static str = "\x1b[08m";
    pub const BLACK: &'static str = "\x1b[30m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const ORANGE: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const PURPLE: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const LIGHTGREY: &'static str = "\x1b[37m";
    pub const DARKGREY: &'static str = "\x1b[90m";
    pub const LIGHTRED: &'static str = "\x1b[91m";
    pub const LIGHTGREEN: &'static str = "\x1b[92m";
    pub const YELLOW: &'static str = "\x1b[93m";
    pub const LIGHTBLUE: &'static str = "\x1b[94m";
    pub const PINK: &'static str = "\x1b[95m";
    pub const LIGHTCYAN: &'static str = "\x1b[96m";
    pub const BG_BLACK: &'static str = "\x1b[40m";
    pub const BG_RED: &'static str = "\x1b[41m";
    pub const BG_GREEN: &'static str = "\x1b[42m";
    pub const BG_ORANGE: &'static str = "\x1b[43m";
    pub const BG_BLUE: &'static str = "\x1b[44m";
    pub const BG_PURPLE: &'static str = "\x1b[45m";
    pub const BG_CYAN: &'static str = "\x1b[46m";
    pub const BG_LIGHTGREY: &'static str = "\x1b[47m";
}