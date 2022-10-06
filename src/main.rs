use crate::tokenizer::separate;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

mod tokenizer;

fn main() {
    let program_test = fs::read_to_string("C:/Users/Oem/PycharmProjects/Tokenizer/src/test")
        .expect("Should have been able to read the file");

    for element in separate(program_test).0.split_whitespace() {
        if !element.contains(" ") {
            print!("{} ", element);
        }
    }
}
