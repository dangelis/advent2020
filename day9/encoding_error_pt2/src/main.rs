use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    //let records = file_lines_as_vec("./example");
    let records = file_lines_as_vec("./input");

    // for r in &records {
    //     println!("{}", r);
    // }


    let invalid_code = 50047984;
    //let invalid_code = 127;

    for k in 0..(records.len() - 1) {

       let mut found = false;
       let mut sum = 0;
       let mut small = u64::MAX;
       let mut large = 0;

        for i in k..(records.len() - 1) {

            if records[i] < small {
                small = records[i];
            } else if records[i] > large {
                large = records[i];
            }

            sum = sum + records[i];
            if sum == invalid_code {
                found = true;
                //println!("Sum = {}, found it between indexes {} and {}, values {} + {} = {}", sum, k, i, records[k], records[i], records[k] + records[i]);
                println!("small: {} + large: {} = Sum: {}, found it between indexes {} and {}",small, large, small + large, k, i);
                break;
            } else if sum > invalid_code {
                break;
                // didn't find it, increment outer loop
            }

        }

        if found {
            println!{"Found chunk"};
            break;
        }

    }

    


}

// 5407707



// File reading
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn file_lines_as_vec(filename: &str) -> Vec<u64> {
    let mut records: Vec<u64> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                records.push(ip.parse::<u64>().unwrap());
            }
        }
    }

    records
}