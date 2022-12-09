use std::env;
use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut running_total = 0;

    let lines : Vec<String> = contents.split("\n").map(|s| s.trim().to_string()).collect();


    println!("{:?}", lines);
    
    let mut i = 0;
 
    loop {

        let first_word =  reduce_string_to_string_which_contains_its_characters_once( &lines[i]);

        let second_word = reduce_string_to_string_which_contains_its_characters_once(  &lines[i + 1]);

        let third_word = reduce_string_to_string_which_contains_its_characters_once(&lines[i + 2]);

        let mut s = String::from("");
        s.push_str(&third_word);
        s.push_str(&second_word);
        s.push_str(&first_word);

        println!("string is {} ", s); 

        let a = get_value_of_item_which_appears_in_three(&s);
       
        running_total += a;

        i += 3;
        if i >= lines.len() - 2 { break; }
    }
    println!("Count {}", running_total); 
}


fn get_value_of_item_which_appears_in_three(the_word: &str) -> u32 {

    let all_letters : Vec<char>  = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
 
    let word_to_get_count_of : Vec<char> = the_word.chars().collect();

    for ch in &all_letters {

        let is_it_in_the_second_word = word_to_get_count_of.iter().filter(|&n| *n == *ch).count();
        
        if is_it_in_the_second_word == 3 {
            let number_value_of_first_char = all_letters.iter().position(|&r| r == *ch).unwrap();
            let value_1 : u32 = number_value_of_first_char as u32;
            println!("Char is  {} value is {}", ch, value_1);
            return value_1;
        }
    }
    return 0;
}

fn reduce_string_to_string_which_contains_its_characters_once(the_word: &str) -> String {
    let books : HashSet<char> = HashSet::from_iter(the_word.chars());
    let s : String = books.iter().cloned().collect();
    return s;
}
