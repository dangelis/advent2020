use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;


struct Intermediate {
    int1: i32,
    int2: i32,
}

fn main() {
    println!("Hello, world!");

        let mut records = Vec::new();

        let mut intermediate_records: Vec<Intermediate> = Vec::new();

        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines("./input") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    //println!("{}", ip);
                    records.push(ip.parse::<i32>().unwrap());
                }
            }
        }

        records.sort();

        for i in 0..=records.len() - 1 {
            for j in 0..=records.len() - 1 {
                let sum = records[i] + records[j];
                //println!("{} + {} = {}", i, j, sum);

                if sum <= 2020 {
                    println!("Found that {} + {} <= 2020", records[i], records[j]);
                    let holder = Intermediate
                    {
                        int1: records[i],
                        int2: records[j],
                    };
                    intermediate_records.push(holder);
                }
            }
        }

        for i in 0..=intermediate_records.len() - 1 {
            for j in 0..=records.len() - 1 {
                let sum = intermediate_records[i].int1 + intermediate_records[i].int2 + records[j];
                //println!("{} + {} = {}", i, j, sum);

                if sum == 2020 {
                    println!("Found that {} + {} + {} = 2020", intermediate_records[i].int1, intermediate_records[i].int2, records[j]);
                    println!("Solution: {}", intermediate_records[i].int1 * intermediate_records[i].int2 * records[j]);
                    process::exit(0);
                }
            }
        }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
