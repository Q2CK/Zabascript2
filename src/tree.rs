use crate::tokenizer::separate;
use crate::tokenizer::Token;

use std::fs;
use std::thread::sleep;
use std::time::Duration;


pub struct Node {
    pub token: Token,
    pub children: Vec<Node>,
    pub depth: u16,
}

impl Node {
    fn new(new_name: String, line: usize) -> Node {
        Node {
            token: Token::new(new_name, line),
            children: Vec::new(),
            depth: 0,
        }
    }
    fn add_child(&mut self, mut new_node: Node) {
        new_node.depth = self.depth + 1;
        self.children.push(new_node);
    }
    fn traverse(&mut self, function: fn(&mut Node)) {
        function(self);
        for child in &mut self.children {
            child.traverse(function);
        }
    }
}

pub fn handle_tree() {
    let program_test: String =
        fs::read_to_string("C:/Users/Oem/PycharmProjects/tokenizer/src/test")
        .expect("Should have been able to read the file");

    let mut tree: Node = Node::new(String::from("root"), 0);

    for element in separate(program_test).iter() {
        tree.add_child(
            Node::new(element.token_name.clone(), element.line_number)
        );
    }

    tree.traverse(|x: &mut Node| {
        print!("Line {}: ", x.token.line_number);
        for _ in 0..x.depth {
            print!("  ");
        }
        //x.name.push_str("_test");
        print!("{}\n", x.token.token_name);
    });
}