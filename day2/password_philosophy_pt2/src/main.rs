use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

fn main() {

    let records = file_lines_as_vec("./input");

    let mut valid_count = 0;

    for r in records {
        println!("{}", r);
        let line_tokens: Vec<&str> = r.split(' ').collect();
        let range_tokens: Vec<&str> = line_tokens[0].split('-').collect();
        let min = range_tokens[0].parse::<i32>().unwrap();
        let max = range_tokens[1].parse::<i32>().unwrap();
        let c = line_tokens[1].replace(":", "");
        let pass = line_tokens[2];
        //println!("min: {}, max: {}, c: {}, pass: {}", min, max, c, pass);

        if validate(min, max, c.chars().next().unwrap(), pass) {
            valid_count = valid_count + 1;
            println!("Valid passwords now: {}", valid_count);
        }
    }

    println!("Valid passwords: {}", valid_count);
    // not 466

}

fn validate(min: i32, max: i32, c: char, pass: &str) -> bool {
    let mut valid = false;

    let pos1: char = pass.chars().nth((min -1).try_into().unwrap()).unwrap();
    
    let mut pos1_valid = false;
    if pos1 == c {
        pos1_valid = true
    }

    let pos2: char = pass.chars().nth((max - 1).try_into().unwrap()).unwrap();

    let mut pos2_valid = false;
    if pos2 == c {
        pos2_valid = true
    }

    if (pos1_valid || pos2_valid) && !(pos1_valid && pos2_valid) {
        valid = true;
    }

    valid
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn file_lines_as_vec(filename: &str) -> Vec<String> {
    let mut records: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                records.push(ip);
            }
        }
    }

    records
}