use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    //let records = file_lines_as_vec("./example");
    let records = file_lines_as_vec("./input");

    // for r in &records {
    //     println!("{}", r);
    // }

    let preamble = 25;

    for k in preamble..(records.len() - 1) {

       let check_val = records[k];
       let mut broke = false;

        for i in k - preamble..k {
            if broke {
                break;
            }

            for j in k - preamble..k {
                if records[i] == records[j] {
                    continue;
                }

                println!("Comparing values {} and {} from indexes {} and {}, sum {}, against check_val {}", records[i], records[j], i, j, records[i] + records[j], check_val);
                if records[i] + records[j] == check_val {
                    println!("Found that values {} and {} from indexes {} and {} equal check_val {}", records[i], records[j], i, j, check_val);
                    broke = true;
                    break;
                }
            }
        }

        if !broke {
            println!{"Found invalid code {}", check_val};
            break;
        }

    }

}

// 50047984



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