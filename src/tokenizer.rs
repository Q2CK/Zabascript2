use std::iter::zip;

pub struct Token {
    pub token_name: String,
    pub line_number: usize
}

fn combine(token_string: String, token_line_numbers: Vec<usize>) -> Vec<Token> {
    let mut output_vec: Vec<Token> = vec![];

    let token_names: Vec<String> = token_string.split_whitespace().map(String::from).collect();

    for (name, line) in zip(token_names, token_line_numbers) {
        output_vec.push(Token{token_name: name, line_number: line});
    }

    return output_vec;
}

pub fn separate(mut input_string: String) -> Vec<Token> {

    let possible_separators: Vec<char> = "~!@#$%^&*()-=+;:'\"|<>?/".chars().collect();
    let absolute_separators: Vec<char> = "()[]{};:'\",.".chars().collect();

    let mut beginning: usize = 0;
    let mut i: usize = 1;
    let mut line: usize = 0;

    let mut line_number_list: Vec<usize> = vec![];

    while i < input_string.len() {

        if input_string.chars().nth(i) == Some('\n') {
            line += 1;

            let input_slice: &str = &input_string[beginning..i];
            let input_split_vec: Vec<&str> = input_slice.split_whitespace().collect();
            let tokens_in_line: usize = input_split_vec.len();

            beginning = i + 1;

            for _ in 0..tokens_in_line {
                line_number_list.push(line);
            }
        }

        if absolute_separators.contains(
            &input_string
                .chars()
                .nth(i)
                .unwrap()) {

            input_string.insert(i,' ');
            i += 2;
            input_string.insert(i,' ');
        }

        if possible_separators.contains(
            &input_string
                .chars()
                .nth(i)
                .unwrap()) && ! possible_separators.contains(
            &input_string
                .chars()
                .nth(i - 1)
                .unwrap()) {

            input_string.insert(i, ' ');
            i += 1;
        }

        if possible_separators.contains(
            &input_string
                .chars()
                .nth(i)
                .unwrap()) && ! possible_separators.contains(
            &input_string
                .chars()
                .nth(i + 1)
                .unwrap()) {

            input_string.insert(i + 1, ' ');
            i += 1;
        }

        i += 1;
    }

    return combine(input_string, line_number_list);
}