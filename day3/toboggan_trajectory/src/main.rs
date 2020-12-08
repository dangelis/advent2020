use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct Point {
    x: usize,
    y: usize,
}

static mut TREE_COUNT: usize = 0;
// 216

fn main() {

    let records = file_lines_as_vec("./input");
    println!("Number of records: {}", records.len());

    let r: usize = 3;
    let d: usize = 1;

    let mut pos = Point {
        x: 0,
        y: 0,
    };
    
    let mut row: usize = 0;

    while row < records.len() {
        println!("At: {},{}", pos.x, pos.y);
        
        row = pos.y;
        pos = check_point(pos, r, d, &records);
        
        if row + 1 + d == records.len() {
            println!("Breaking at row {}", row);
            break;
        }
    }

    unsafe {
        println!("Number of trees: {}", TREE_COUNT);
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



// Traversal methods that won't work because I misunderstood the problem

// fn traverse(at: Point, right_dist: usize, down_dist: usize, records: &Vec<String>) -> Point {
//     let mut pos = traverse_right(at, right_dist, &records);

//     if pos.y + 1 < records.len() {
//         // println!("about to traverse down from: {}, {}", pos.x, pos.y);
//         pos = travers_down(pos, down_dist, &records);
//     } else {
//         // kind of a cheat so we exit
//         pos.y = pos.y + 1;
//     }

//     pos
// }


// fn traverse_right(mut at: Point, dist: usize, records: &Vec<String>) -> Point {
//     let line = &records[at.y];
//     let mut moved: usize = 0;
    
//     println!("traverse right, at: {},{}, line: {}", at.x, at.y, line);

//     while moved < dist {
//         // println!("    Moving right, at: {},{}", at.x, at.y);
//         if line.chars().nth(at.x).unwrap() == '#' {
//             println!("Found a tree while traversing right at {},{}", at.x, at.y);
//             unsafe {
//                 TREE_COUNT = TREE_COUNT + 1;
//             }
//         }

//         at = inc_right(at, &records);
//         println!("    Now, at: {},{}", at.x, at.y);

//         moved = moved + 1;
//         // println!("traverse_right: moved: {}, dist: {}", moved, dist);
//     }

//     return at;
// }

// fn travers_down(mut at: Point, dist: usize, records: &Vec<String>) -> Point {
//     let mut moved: usize = 0;
    
//     println!("traverse down, at: {},{}", at.x, at.y);

//     while moved < dist {
//         at.y = at.y + 1;
//         let line = &records[at.y];

//         if line.chars().nth(at.x).unwrap() == '#' {
//             println!("Found a tree traversing down at {},{}", at.x, at.y);
//             unsafe {
//                 TREE_COUNT = TREE_COUNT + 1;
//             }
//         }

//         moved = moved + 1;
//     }
    
//     at

// }






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