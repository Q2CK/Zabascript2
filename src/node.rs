pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub depth: u16,
    pub line_number: usize
}

impl Node {
    pub fn new(new_name: &str, line: usize) -> Node {
        Node {
            name: String::from(new_name),
            children: Vec::new(),
            depth: 0,
            line_number: line
        }
    }
    pub fn add_child(&mut self, mut new_node: Node) {
        new_node.depth = self.depth + 1;
        self.children.push(new_node);
    }
    pub fn traverse(&mut self, function: fn(&mut Node)) {
        function(self);
        for child in &mut self.children {
            Node::traverse(child, function);
        }
    }
}