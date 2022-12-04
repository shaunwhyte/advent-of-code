use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut running_amount= 0;
    let mut all_numbers = Vec::new();

    for s in contents.split("\n") {
        if s.trim() == "" {
            all_numbers.push(running_amount);
            running_amount = 0;
        }
        else {
            if s.trim() == "\n" { continue; }
            let my_int = s.trim().parse::<i32>().unwrap();
            running_amount += my_int;
        }
    }

    all_numbers.sort();
    let best_three = all_numbers[all_numbers.len() - 1] + all_numbers[all_numbers.len() - 2] + all_numbers[all_numbers.len() - 3];
    println!("Sum of the top three {}", &best_three); 
}