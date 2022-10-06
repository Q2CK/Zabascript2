pub trait Extract {
    fn get(&self) -> char;
}

impl Extract for Option<char> {
    fn get(&self) -> char {
        match self {
            Some(x) => *x,
            None => ' '
        }
    }
}

pub fn separate(mut input_string: String) -> (String, Vec<usize>) {

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

            for n in 0..tokens_in_line {
                line_number_list.push(line);
            }
        }

        if absolute_separators.contains(&input_string.chars().nth(i).get()) {

            input_string.insert(i,' ');
            i += 2;
            input_string.insert(i,' ');

        }

        if possible_separators.contains(&input_string.chars().nth(i).get()) && ! possible_separators.contains(&input_string.chars().nth(i - 1).get()) {

            input_string.insert(i, ' ');
            i += 1;

        }

        if possible_separators.contains(&input_string.chars().nth(i).get()) && ! possible_separators.contains(&input_string.chars().nth(i + 1).get()) {

            input_string.insert(i + 1, ' ');
            i += 1;

        }

        i += 1;
    }

    return (input_string, line_number_list);
}