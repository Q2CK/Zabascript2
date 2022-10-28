use tokenizer::separate;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use crate::node::Node;
use crate::tokenizer::Token;

mod tokenizer;
mod node;

fn main() {
    let program_test: String = fs::read_to_string("C:/Users/Oem/PycharmProjects/tokenizer/src/test")
        .expect("Should have been able to read the file");

    let mut tree = Node::new("root", 0);

    for element in separate(program_test).iter() {
        tree.add_child(Node::new(&element.token_name, element.line_number));
    }

    tree.traverse(|x: &mut Node| {
        print!("Line {}: ", x.line_number);
        for _ in 0..x.depth {
            print!("  ");
        }
        //x.name.push_str("_test");
        print!("{}\n", x.name);
    });
}
