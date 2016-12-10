use std::io::{self, BufRead};

fn main() {
    let reader = io::stdin();

    // each line the user types in the terminal will be evaluated seperately
    for line in reader.lock().lines()   {
        let to_analyze = line.unwrap();

        // gives me the most used char and also it's occurence.
        let result = get_most_used_char(&to_analyze);

        // This is the analyzed output:
        println!("String Length: {}", to_analyze.len());
        println!("Most used character: '{}', {} occurences", result.0, result.1);
        println!("");
    }
}

fn get_most_used_char(new_string: &str) -> (char, u32) {
    // get a list of characters from the input string
    let chars = new_string.chars();

    // using an array of 500 to store most possible char values
    let mut char_counter: [u32; 500] = [0; 500];

    let mut index;
    for character in chars {
        // each char get's converted to a u32 index. this index helps to keep track of the
        // count of each character in the input string
        index = u32::from(character) as usize;

        // if the index is non-valid as a char, just replace it with the last value of the array.
        if (index < 0) || (index >= char_counter.len()) {
            index = char_counter.len() - 1;
        }

        char_counter[index] += 1;
    }

    // try to transform the first u32 of the array to a char
    let mut opt_char = std::char::from_u32(0);;
    // this tmp_count saves the current highest count of a char.
    let mut tmp_count = 0;

    // for each possible char in the array, just check which one is the biggest.
    for i in 0u32..char_counter.len() as u32 {
        if tmp_count < char_counter[i as usize] {
            opt_char = char::from_u32(i);
            tmp_count = char_counter[i as usize];
        }
    }

    // return the most used char, together with the occurence.
    // if the char cannot be unwrapped (out of the Option<char>) just return ' ' instead
    return (opt_char.unwrap_or(' '), tmp_count);
}
