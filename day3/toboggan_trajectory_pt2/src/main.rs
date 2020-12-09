use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

static mut TREE_COUNT: usize = 0;
// 6708199680

fn main() {

    let records = file_lines_as_vec("./input");
    println!("Number of records: {}", records.len());

    let r: usize = 1;
    let d: usize = 1;

    let pos = Point {
        x: 0,
        y: 0,
    };
    
    // Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.


    unsafe {
        let s1 = get_trees_for_slope(1, 1, &records);
        TREE_COUNT = 0;    
        let s2 = get_trees_for_slope(3, 1, &records);
        TREE_COUNT = 0;
        let s3 = get_trees_for_slope(5, 1, &records);
        TREE_COUNT = 0;
        let s4 = get_trees_for_slope(7, 1, &records);
        TREE_COUNT = 0;
        let s5 = get_trees_for_slope(1, 2, &records);
    
        println!("total trees: {}:{}:{}:{}:{}", s1, s2, s3, s4, s5);
        println!("total trees: {}", f64::from(s1) * f64::from(s2) * f64::from(s3) * f64::from(s4) * f64::from(s5) );
    }

}

fn get_trees_for_slope(right_dist: usize, down_dist: usize, records: &Vec<String>) -> u32 {
    
    let mut row: usize = 0;

    let mut at = Point {
        x: 0,
        y: 0,
    };

    while row < records.len() {
        println!("At: {},{}", at.x, at.y);
        
        row = at.y;
        at = check_point(at, right_dist, down_dist, &records);
        
        if row + 1 + down_dist == records.len() {
            println!("Breaking at row {}", row);
            break;
        }
    }

    unsafe {
        println!("Number of trees: {}", TREE_COUNT);
        return TREE_COUNT.try_into().unwrap();
    }
}

    

fn check_point(mut at: Point, right_dist: usize, down_dist: usize, records: &Vec<String>) -> Point {
    
    println!("traverse right, from: {},{}", at.x, at.y);
    for _i in 0..right_dist {
        at = inc_right(at, records);
    }

    println!("traverse down, from: {},{}", at.x, at.y);
    at.y = at.y + down_dist;

    let line = &records[at.y];
    println!("Checking for tree at {},{} for line {}", at.x, at.y, line);

    if line.chars().nth(at.x).unwrap() == '#' {
        println!("Found a tree at {},{}", at.x, at.y);
        unsafe {
            TREE_COUNT = TREE_COUNT + 1;
        }
    }

    at
}

fn inc_right (mut at: Point, records: &Vec<String>) -> Point {
    if at.x + 1 >= records[at.y].len() {
        at.x = 0;
    } else {
        at.x = at.x + 1;
    }

    at
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