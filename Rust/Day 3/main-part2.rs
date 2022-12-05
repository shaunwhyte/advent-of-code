use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut running_total = 0;

    let lines : Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    let mut i = 0;
 
    loop {


        
       // println!("Count {}", i); 
    

        let first_word = &lines[i];
 
        let second_word = &lines[i + 1];

        let third_word = &lines[i + 2];

        let mut s = String::from("");
        s.push_str(first_word);
        s.push_str(second_word);
        s.push_str(third_word);

        println!("a {}", s); 

        let a = get_value_of_item_which_appears_in_three(first_word, second_word, third_word);
       
        running_total += a;
        i += 2;
        if i >= lines.len() - 2 { break; }
      
    }
    println!("Count {}", running_total); 
    
}


fn get_value_of_item_which_appears_in_three(first_word: &str, second_word: &str, third_word: &str) -> u32 {

    let all_letters : Vec<char>  = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let chars_of_second_word : Vec<char> = second_word.chars().collect();

    let word_to_get_count_of : Vec<char> = [first_word, second_word, third_word].join("").chars().collect();

    let borrowed_string: &str = "world";
    


    for ch in word_to_get_count_of {

        let is_it_in_the_second_word = all_letters.iter().filter(|&n| *n == ch).count();

        
        if is_it_in_the_second_word == 3 {
            let number_value_of_first_char = all_letters.iter().position(|&r| r == ch).unwrap();
            let value_1 : u32 = number_value_of_first_char as u32;
            return value_1;
        }

    }
    return 0;
}
