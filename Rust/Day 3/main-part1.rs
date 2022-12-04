use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut running_total = 0;

    for line in contents.split("\n") {
        let count = line.trim().len();
        let word_count = count / 2;
        let first_word = &line[0..(word_count)];
        let second_word = &line[ (word_count )..count ];
        let a = get_value_of_item_which_appears_in_both(first_word, second_word);
       
        running_total += a;
    }
    println!("Count {}", running_total); 
    
}

fn get_value_of_item_which_appears_in_both(first_word: &str, second_word: &str) -> u32 {

    let all_letters : Vec<char>  = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let chars_of_second_word : Vec<char> = second_word.chars().collect();

    for ch in first_word.chars() {

        let is_it_in_the_second_word = chars_of_second_word.contains(&ch);

        if is_it_in_the_second_word {
            
            let number_value_of_first_char = all_letters.iter().position(|&r| r == ch).unwrap();
            let value_1 : u32 = number_value_of_first_char as u32;
            return value_1;
        }

    }
    return 0;
}