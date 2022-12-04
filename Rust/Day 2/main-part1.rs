use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut score = 0;

    for line in contents.split("\n") {

        let values: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

        let elfs_answer : &str = convert_elfs_answer(&values[0]);
        let my_answer : &str = convert_my_answer(&values[1]);

        score += get_score_for_round(elfs_answer, my_answer);
    }
    println!("Score {}", &score); 
}

fn get_score_for_round(elf: &str , me: &str) -> u32 {

    let mut shape_score  = 0;

    match &me as &str {
        "Rock" =>  shape_score = 1,
        "Paper" =>  shape_score = 2,
        "Scissors" =>  shape_score = 3,
        _ => shape_score = 0,
    }

    let mut round_amount= 0;

    // plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
    if elf == me {
        round_amount = 3;
    }
    else if elf == "Rock" && me == "Scissors" {
        round_amount = 0;
    }
    else if elf == "Scissors" && me == "Paper" {
        round_amount = 0;
    }
    else if elf == "Paper" && me == "Rock" {
        round_amount = 0;
    } else {
        round_amount = 6;
    }

    
    return round_amount + shape_score;
}


fn convert_elfs_answer(answer: &String) -> &str {
    match &answer as &str {
        "A" => return "Rock",
        "B" => return "Paper",
        "C" => return "Scissors",
        _ => return ""
    }
}

fn convert_my_answer(answer: &String) -> &str {
    match &answer.trim() as &str {
        "X" => return "Rock",
        "Y" => return "Paper",
        "Z" => return "Scissors",
        _ => return ""
    }
}