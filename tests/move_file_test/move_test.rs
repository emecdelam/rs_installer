
use installer::utils::file::*;
use installer::logger::results::*;
use std::path::Path;

#[test]
fn move_simple_file_test() {
    let base = Path::new("tests/move_file_test/init/test.txt");
    let out = Path::new("tests/move_file_test/end/test.txt");
    assert_eq!(true,is_success(move_file(base, out)));
    assert_eq!(true,file_is_in_dir("test.txt", Path::new("tests/move_file_test/end/")));
    move_file("tests/move_file_test/end/test.txt", "tests/move_file_test/init/test.txt");
}