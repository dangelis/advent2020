use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut records = file_lines_as_vec("./input");
    records.sort();
    let mut adapters: Vec<usize> = Vec::new();

    find_next_adapter(&mut records, 0, &mut adapters);
    println!("num adapters uses {}", adapters.len());

    let mut one_count:usize = 0;
    let mut three_count:usize = 0;

    // So we include the first adapter in the diff count and the built in adapter
    let mut final_adapters = vec![0];
    final_adapters.append(&mut adapters);
    final_adapters.push(final_adapters.iter().max().unwrap() + 3usize);
    
    for a in 1..final_adapters.len() {
        println!("{}: comparing adapter: {} against {}, diff {}", a, final_adapters[a], final_adapters[a - 1], final_adapters[a] - final_adapters[a - 1]);
        if final_adapters[a] - final_adapters[a - 1] == 1 {
            // count 1s
            one_count = one_count + 1;
        } else if final_adapters[a] - final_adapters[a - 1] == 3 {
            // count 3s
            three_count = three_count + 1;
        }
    }

    println!("one_count: {}, three_count: {}, mutl: {}", one_count, three_count, one_count * three_count);
    //2516
}


fn find_next_adapter(records: &mut Vec<usize>, curr_adapter: usize, adapters: &mut Vec<usize>) {
    let mut new_adapter: usize = 0;
    println!{"Finding next addapter, current adapter: {}, have {} adapters left, used {} adapters", curr_adapter, records.len(), adapters.len()};
    for r in 0..records.len() {
        
        new_adapter = records[r];
        println!{"  checking addapter {} against current addapter {}", new_adapter, curr_adapter};

        if new_adapter - curr_adapter <= 3 {
            println!{"    Found usable adapter with rating {} to match current adapter of {}", records[r], curr_adapter};
            adapters.push(records[r]);
            records.remove(r);
            
            break;
        }
    }

    if records.len() > 0 {
        find_next_adapter(records, new_adapter, adapters);
    }
}





// File reading
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn file_lines_as_vec(filename: &str) -> Vec<usize> {
    let mut records: Vec<usize> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {           
            if let Ok(ip) = line {
                records.push(ip.parse::<usize>().unwrap());
            }
        }
    }

    records
}