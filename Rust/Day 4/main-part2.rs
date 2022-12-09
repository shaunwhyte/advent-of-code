use std::env;
use std::fs;
use std::fmt;

struct Section {
    start: u32,
    end: u32
}

trait FullOverlapDetector {
    fn detect(&self, other: Section) -> u32;
}

impl FullOverlapDetector for Section {
    fn detect(&self, other: Section) -> u32 {

        /*
        |-234-----|
        |---45678-|


        */
        if self.start < other.start && self.end == other.start  {
            return 1;
        }

        /*

        2-4,6-8 -> 0
        2-3,4-5 -> 0

        5-7,7-9 -> 1
        2-8,3-7 -> 1
        6-6,4-6 -> 1
        2-6,4-8 -> 1


          |----567--|
          |------789|
        */
        if self.start <= other.start && self.end == other.start  {
            return 1;
        }
        if other.start <= self.start && self.end == other.start  {
            return 1;
        }

        /*
          |------789|
          |----567--|
         */
        if other.start <= self.start && self.end == other.start  {
            return 1;
        }
        if self.start <= other.start && self.end == other.start  {
            return 1;
        }


        /*
        |-234-----|
        |-----678-|
        */

        /*
        |-23456---|
        |---45678-|
        */

        if self.start <= other.start && self.end <= other.end &&  self.end > other.start {
            return 1;
        }
        if other.start <= self.start && other.end <= self.end &&  other.end > self.start {
            return 1;
        }



        // 1-10, 2-9
        if self.start <= other.start && self.end >= other.end {
            return 1;
        }

        //  2-9, 1-10
        if self.start >= other.start && self.end <= other.end  {
            return 1;
        }
        // ---------------------------
        if other.start <= self.start && other.end >= self.end {
            return 1;
        }

        //  2-9, 1-10
        if other.start >= self.start && other.end <= self.end  {
            return 1;
        }

        return 0;
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

    test1();
    test2();
    test3();
    test4();
    /*
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines : Vec<String> = contents.split("\n").map(|s| s.trim().to_string()).collect();


    let mut count = 0;

    for line in &lines {
        //println!("Line is {}", &line);
        let splitLine :  Vec<String>  = line.split(",").map(|s| s.trim().to_string()).collect();

        let one = Section::create(splitLine[0].clone());
        let two = Section::create(splitLine[1].clone());

        let result = one.detect(two);

        if result == 1 {
            println!("I worked {}  ", one);
        }
        else {
            println!("I didnt worked {}  ", one);
        }
        count += result;

    }

    println!("Count is {:?}", count);
    */

}

fn test1(){
    /*
    |-234-----|
    |-----678-|
    */


    let one = Section {
        start: 2,
        end: 4
    };
    let two = Section {
        start: 6,
        end: 8
    };
    let result = one.detect(two);
    if result == 0 {
        println!("PASS");
    }else{
        println!("FAIL");
    }
}

fn test2(){
    /*
    |-----678-|
    |-234-----|

    */


    let one = Section {
        start: 6,
        end: 8
    };
    let two = Section {
        start: 2,
        end: 4
    };
    let result = one.detect(two);
    if result == 0 {
        println!("PASS");
    }else{
        println!("FAIL");
    }
}

fn test3(){
    /*
    |---45678-|
    |-234-----|

    */

    let one = Section {
        start: 4,
        end: 8
    };
    let two = Section {
        start: 2,
        end: 4
    };
    let result = one.detect(two);
    if result == 0 {
        println!("PASS");
    }else{
        println!("FAIL");
    }
}

fn test4(){
    /*
    |-234-----|
    |---45678-|


    */

    let one = Section {
        start: 2,
        end: 4
    };
    let two = Section {
        start: 4,
        end: 8
    };
    let result = one.detect(two);
    println!("Result is {}", result);

    if result == 1 {
        println!("PASS");
    }else{
        println!("FAIL");
    }
}
