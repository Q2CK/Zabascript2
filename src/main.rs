use crate::tokenizer::separate;

mod tokenizer;

fn main() {
    let program_test = String::from("fn fibonacci(n) {
        if(n == 0 or n == 1) {
            return n
        }
        else(True) {
            return fibonacci(n - 1) + fibonacci(n - 2)
        }
        return 1
    }");

    let token_list: Vec<String> = vec![];

    for element in token_list {
        println!("{}", element);
    }
}
