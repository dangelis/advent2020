use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    let lines = file_lines_as_vec("./input");
    // let lines = file_lines_as_vec("./example");

    let mut curr_mask_str: &str;
    let mut mem: HashMap<usize, u64> = HashMap::new();

    // For setting 0s
    let mut and_mask: u64 = u64::max_value();
    // For setting 1s
    let mut or_mask: u64 = 0;


    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();

        match tokens[0] {
            "mask" => {
                and_mask = u64::max_value();
                or_mask = 0;
                curr_mask_str = tokens[2];
                println!("current mask: {}", curr_mask_str);

                for (i, c) in curr_mask_str.char_indices() {
                    match c {
                        '1' => {
                            //println!("Found {} at index {}", c, i);
                            let mut mask = 1;
                            mask = mask << curr_mask_str.chars().count() - i - 1;
                            println!("  new or mask: {:b}", mask);
                            or_mask = or_mask | mask;
                            println!("  total or mask: {:b}", or_mask);

                        },
                        '0' => {
                            //println!("Found {} at index {}", c, i);
                            let mut mask = 1;
                            mask = mask <<  curr_mask_str.chars().count() - i - 1;
                            //invert
                            mask = !mask;
                            println!("  new and mask: {:b}", mask);
                            and_mask = and_mask & mask;
                            println!("  total and mask: {:b}", and_mask);
                        }
                        _ => {}
                    }
                }
            },
            _ => {
                // assume mem
                let mem_tokens: Vec<&str> = line.split(' ').collect();
                let addr = mem_tokens[0].replace("mem[","").replace("]","").parse().unwrap();
                let mut val = mem_tokens[2].parse().unwrap();
                println!("addr: {}, val: {} 0x{:b}", addr, val, val);

                println!("  applying and mask: {:b}", and_mask);
                val = val & and_mask;
                println!("    value now {:b}", val);

                println!("  applying or mask: {:b}", or_mask);
                val = val | or_mask;
                println!("    value now {:b}", val);

                mem.insert(addr, val);
            }
        }
    }

    let mut sum = 0;
    for (k, v) in mem {
        sum = sum + v;
        println!("at addr {}, value {:b}, sum now {}", k, v, sum);
    }

    println!("sum: {}", sum);
    //8471403462063
}


// File reading
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