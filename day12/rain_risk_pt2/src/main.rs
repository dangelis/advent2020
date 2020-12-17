use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    // let records = file_lines_as_vec("./example");
    let records = file_lines_as_vec("./input");

    let mut wx = 10;
    let mut wy = 1;
    let mut wd = 0;

    let mut x = 0;
    let mut y = 0;

    println!("Start at: {},{} wp at: {},{} at {}", x, y, wx, wy, wd);

    for l in records {
        let (cmd, distance) = l.split_at(1);
        let dist = distance.parse::<i32>().unwrap();
        match cmd {
            "N" => {
                println!("N {}", dist);
                wy = wy + dist;
            },
            "S" => {
                println!("S {}", dist);
                wy = wy - dist;
            },
            "E" => {
                println!("E {}", dist);
                wx = wx + dist;
            },
            "W" => {
                println!("W {}", dist);
                wx = wx - dist;
            },
            "L" => {
                println!("L {}", dist);
                match dist {
                    270 => {
                        let twy = wy;
                        let twx = wx;
                        wx = twy;
                        wy = -twx;
                    },
                    180 => {
                        let twy = wy;
                        let twx = wx;
                        wx = -twx;
                        wy = -twy;
                    },
                    90 => {
                        let twy = wy;
                        let twx = wx;
                        wx = twy;
                        wy = -twx;
                    },
                    _ => {}
                }
            },
            //TODO figure out the signage of the points 
            // rotate 90 at a time?
            // move origin to avoid axis
            "R" => {
                println!("R {}", dist);
                match dist {
                    90 => {
                        let twy = wy;
                        let twx = wx;
                        wx = -twy;
                        wy = twx;
                    },
                    180 => {
                        let twy = wy;
                        let twx = wx;
                        wx = -twx;
                        wy = -twy;
                    },
                    270 => {
                        let twy = wy;
                        let twx = wx;
                        wx = twy;
                        wy = -twx;
                    },
                    _ => {}
                }

            },
            "F" => {
                println!("F {}", dist);
                x = x + (wx * dist);
                y = y + (wy * dist);
            },
            _ => {}
        }
        println!("Now at: {},{} wp at: {},{} at {}", x, y, wx, wy, wd);
    }

    println!("I'm at {}, {}, total: {}", x, y, x.abs() + y.abs() );
    //34625 to high
    //12627 to low
    // not 29257
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
