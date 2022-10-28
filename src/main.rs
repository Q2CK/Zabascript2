use tokenizer::separate;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

mod tokenizer;

fn main() {
    let program_test: String = fs::read_to_string("C:/Users/Oem/PycharmProjects/tokenizer/src/test")
        .expect("Should have been able to read the file");

    for element in separate(program_test).iter() {
        println!("Line nr {}: {}", element.line_number, element.token_name);
    }
}
