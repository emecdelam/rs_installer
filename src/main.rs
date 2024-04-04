#[allow(unused_imports)]
mod utils;
#[allow(unused_imports)]
use utils::file::*;
mod logger;
#[allow(unused_imports)]
use logger::{results::is_success, *};
mod parser;
use parser::parse;
fn main() {
    parse::parse_file("example.rsins");
}
