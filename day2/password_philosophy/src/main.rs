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

        if validate(min, max, &c, pass) {
            valid_count = valid_count + 1;
            println!("Valid passwords now: {}", valid_count);
        }
    }

    println!("Valid passwords: {}", valid_count);
    // not 466

}

fn validate(min: i32, max: i32, c: &str, pass: &str) -> bool {
    let mut valid = false;

    let tokens: Vec<&str> = pass.split(c).collect();

    if tokens.len() - 1 >= min.try_into().unwrap() && tokens.len() - 1 <= max.try_into().unwrap() {
        valid = true;
        println!("VALID: min: {}, max: {}, c: {}, pass: {}, instances: {}", min, max, c, pass, tokens.len() - 1);
    } else {
        println!("INVALID: min: {}, max: {}, c: {}, pass: {}, instances: {}", min, max, c, pass, tokens.len() - 1);
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