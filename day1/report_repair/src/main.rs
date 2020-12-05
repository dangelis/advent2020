use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;


fn main() {
    println!("Hello, world!");

        let mut records = Vec::new();

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

                if sum == 2020 {
                    println!("Found that {} + {} = 2020", records[i], records[j]);
                    println!("Solution: {}", records[i] * records[j]);
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
