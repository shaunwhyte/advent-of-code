use std::env;
use std::fs;
use std::fmt;

struct Section {
    start: u32,
    end: u32
}

trait FullOverlapDetector {
    fn detect(&self, other: Section) -> bool;
}

impl FullOverlapDetector for Section {
    fn detect(&self, other: Section) -> bool {

        if self.start == other.start {
            return true;
        }

        if self.end == other.end {
            return true;
        }

        if self.start <= other.start {
            if self.end >= other.end {
                return true;
            }
        }
        else {
            if other.end >= self.end {
                return true;
            }
        }
        return false;
    }
}

trait CreateSection {
    fn create(value: String) -> Section;
}

impl CreateSection for Section {
    // expects value to be like "1-10"
    fn create(value: String) -> Section {

        let splittedFromDash: Vec<String>  = value.split("-").map(|s| s.trim().to_string()).collect();

        let theSection = Section {
            start: (splittedFromDash[0].parse().unwrap()),
            end: (splittedFromDash[1].parse().unwrap())
        };

        return theSection;
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Section - (Start: {}, End: {})", self.start, self.end)
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines : Vec<String> = contents.split("\n").map(|s| s.trim().to_string()).collect();


    let mut count = 0;

    for line in &lines {
        println!("Line is {}", &line);
        let splitLine :  Vec<String>  = line.split(",").map(|s| s.trim().to_string()).collect();

        let one = Section::create(splitLine[0].clone());
        let two = Section::create(splitLine[1].clone());

        if one.detect(two) {
            count += 1;
        };
    }

    println!("Count is main-part1.rs{:?}", count);


}