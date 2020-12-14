use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    let lines = file_lines_as_vec("./input");

    let mut curr_mask_str: &str;
    let mut mem: HashMap<usize, u64> = HashMap::new();

    // For setting 0s
    let mut and_masks: Vec<u64> = Vec::new();
    // For setting 1s
    let mut or_masks: Vec<u64> = Vec::new();

    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();

        match tokens[0] {
            "mask" => {
                curr_mask_str = tokens[2];
                println!("current mask: {}", curr_mask_str);

                for (i, c) in curr_mask_str.char_indices() {
                    match c {
                        '1' => {
                            println!("Found {} at index {}", c, i);
                            let mut mask = 1;
                            mask = mask << curr_mask_str.chars().count() - i - 1;
                            or_masks.push(mask);
                        },
                        '0' => {
                            println!("Found {} at index {}", c, i);
                            let mut mask = 1;
                            mask = mask <<  curr_mask_str.chars().count() - i - 1;
                            //invert
                            mask = !mask;
                            and_masks.push(mask);
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
                println!("addr: {}, val: {}", addr, val);

                for a in &and_masks {
                    println!("  applying and mask: {:b}", a);
                    val = val & a;
                    println!("value now {:b}", val);

                }

                for o in &or_masks {
                    println!("  applying or mask: {:b}", o);
                    val = val | o;
                    println!("value now {:b}", val);
                }

                mem.insert(addr, val);
            }
        }
    }

    let mut sum = 0;
    for (_k, v) in mem {
        sum = sum + v;
    }

    println!("sum: {}", sum);
    //26829988517442 to high
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