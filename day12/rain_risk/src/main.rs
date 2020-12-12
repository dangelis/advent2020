use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    //let records = file_lines_as_vec("./example");
    let records = file_lines_as_vec("./input");

    let mut x = 0;
    let mut y = 0;
    let mut d = 0;

    for l in records {
        let (cmd, distance) = l.split_at(1);
        let dist = distance.parse::<i32>().unwrap();
        match cmd {
            "N" => {
                println!("N {}", dist);
                y = y + dist;
            },
            "S" => {
                println!("S {}", dist);
                y = y - dist;
            },
            "E" => {
                println!("E {}", dist);
                x = x + dist;
            },
            "W" => {
                println!("W {}", dist);
                x = x - dist;
            },
            "L" => {
                println!("L {}", dist);
                d = get_nice_angle( d + dist);
            },
            "R" => {
                println!("R {}", dist);
                d = get_nice_angle( d - dist);
            },
            "F" => {
                println!("F {}", dist);
                match d {
                    90 => {
                        y = y + dist;
                    },
                    270 => {
                        y = y - dist;
                    },
                    0 => {
                        x = x + dist;
                    },
                    360 => {
                        x = x + dist;
                    },
                    180 => {
                        x = x - dist;
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    println!("I'm at {}, {}, total: {}", x, y, x.abs() + y.abs() );
    //1533
}

fn get_nice_angle(mut raw_angle: i32) -> i32 {
    while raw_angle < 0 {
        raw_angle = raw_angle + 3600000;
    }

    raw_angle % 360
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
