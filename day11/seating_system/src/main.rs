use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;


#[derive(Clone, PartialEq)]
enum LocationType {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocationType::EmptySeat => {write!(f, "EmptySeat")},
            LocationType::OccupiedSeat => {write!(f, "OccupiedSeat")},
            LocationType::Floor => {write!(f, "Floor")},
        }
    }
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
struct FerrySeat {
    loc_type: LocationType,
    up_loc: Point,
    down_loc: Point,
    right_loc: Point,
    left_loc: Point,
    ne: Point,
    nw: Point,
    se: Point,
    sw: Point,
    loc: Point
}

impl fmt::Display for FerrySeat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type: {}, loc: {}, up: {}, ne: {}, right: {}, se: {}, down: {}, sw: {}, left: {}, nw: {}",
               self.loc_type, self.loc, self.up_loc, self.ne, self.right_loc, self.se, self.down_loc, self.sw, self.left_loc, self.nw)
    }
}


fn main() {
    // let records = file_lines_as_vec("./example");
    let records = file_lines_as_vec("./input");

    let cols = records[0].len() as i32;
    let rows = records.len() as i32;
    let mut ferry:Vec<Vec<FerrySeat>> = Vec::new();

    //
    // Prep the matrix
    //
    println!("Creating matrix");
    for l in 0..records.len() {

        let mut row_vec:Vec<FerrySeat> = Vec::new();
        let row: Vec<_> = records[l].chars().collect();

        for (i, seat) in row.iter().enumerate() {

            //println!("Creating seat for ({}, {})", i, l);
            let mut ferry_seat = FerrySeat {
                loc_type: LocationType::EmptySeat,
                loc: Point{x: i as i32, y: l as i32},
                up_loc: Point { x: i as i32, y: (l as i32)  - 1 },
                down_loc: Point { x: i as i32, y: (l as i32) + 1 },
                right_loc: Point { x: (i as i32) + 1 as i32, y: l as i32},
                left_loc: Point {x: (i as i32) - 1 as i32, y: l as i32},
                ne: Point { x: (i as i32) + 1, y: (l as i32) - 1 },
                nw: Point { x: (i as i32) - 1, y: (l as i32) - 1 },
                se: Point { x: (i as i32) + 1 as i32, y: (l as i32) + 1},
                sw: Point {x: (i as i32) - 1 as i32, y: (l as i32) + 1},
            };

            //println!("Assigning loc_type for seat '{}'", seat);
            match seat {
                'L' => {ferry_seat.loc_type = LocationType::EmptySeat; },
                '.'=> { ferry_seat.loc_type = LocationType::Floor; },
                '#' => {ferry_seat.loc_type = LocationType::OccupiedSeat;},
                _ => {},
            }

            println!("type: {}, loc: {}, up: {}, down: {}, right: {}, left: {}",
                    ferry_seat.loc_type, ferry_seat.loc, ferry_seat.up_loc, ferry_seat.down_loc, ferry_seat.right_loc, ferry_seat.left_loc);

            row_vec.push(ferry_seat);
        }

        ferry.push(row_vec);
    }

    let mut seats_are_changing = true;

    println!("Simulating people");
    let mut loop_sim = 0;
    while seats_are_changing {

        let mut next_ferry:Vec<Vec<FerrySeat>> = ferry.to_vec();


        loop_sim = loop_sim + 1;
        println!("At loop {}", loop_sim);
        fancy_print(&mut ferry);


        seats_are_changing = false;
        // Now check the seats
        //println!("first: {}", ferry[0][0].to_string());
        for i in 0..ferry.len() {
            for j in 0..ferry[i].len() {
                let curr_seat = &ferry[i][j];
                println!("checking {}", curr_seat.to_string());

                //
                // RULE 1
                //
                if curr_seat.loc_type == LocationType::EmptySeat {
                    // check for occupied adjacents
                    // let mut checkable_neighbors: Vec<&FerrySeat> = Vec::new();

                    let mut adjacent_seat_occupied = false;

                    if curr_seat.up_loc.y >= 0 {
                        let neighbor: &FerrySeat = &ferry[curr_seat.up_loc.y as usize][curr_seat.up_loc.x as usize];
                        // checkable_neighbors.push(neighbor);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            adjacent_seat_occupied = true;
                        }
                    }

                    if curr_seat.ne.y >= 0 && curr_seat.ne.x < cols {
                        let neighbor: &FerrySeat = &ferry[curr_seat.ne.y as usize][curr_seat.ne.x as usize];
                        // checkable_neighbors.push(neighbor);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            adjacent_seat_occupied = true;
                        }
                    }

                    if curr_seat.right_loc.x < cols {
                        let neighbor: &FerrySeat = &ferry[curr_seat.right_loc.y as usize][curr_seat.right_loc.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.right_loc.y as usize][curr_seat.right_loc.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            adjacent_seat_occupied = true;
                        }
                    }

                    if curr_seat.se.y < rows && curr_seat.se.x < cols {
                        let neighbor: &FerrySeat = &ferry[curr_seat.se.y as usize][curr_seat.se.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.se.y as usize][curr_seat.se.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            adjacent_seat_occupied = true;
                        }
                    }

                    if curr_seat.down_loc.y < rows {
                        let neighbor: &FerrySeat = &ferry[curr_seat.down_loc.y as usize][curr_seat.down_loc.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.down_loc.y as usize][curr_seat.down_loc.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            adjacent_seat_occupied = true;
                        }
                    }

                    if curr_seat.sw.y < rows && curr_seat.sw.x >= 0 {
                        let neighbor: &FerrySeat = &ferry[curr_seat.sw.y as usize][curr_seat.sw.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.sw.y as usize][curr_seat.sw.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            adjacent_seat_occupied = true;
                        }
                    }

                    if curr_seat.left_loc.x >= 0 {
                        let neighbor: &FerrySeat = &ferry[curr_seat.left_loc.y as usize][curr_seat.left_loc.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.left_loc.y as usize][curr_seat.left_loc.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            adjacent_seat_occupied = true;
                        }
                    }

                    if curr_seat.nw.y >= 0 && curr_seat.nw.x >= 0 {
                        let neighbor: &FerrySeat = &ferry[curr_seat.nw.y as usize][curr_seat.nw.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.nw.y as usize][curr_seat.nw.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            adjacent_seat_occupied = true;
                        }
                    }

                    if !adjacent_seat_occupied {
                        // curr_seat.loc_type = LocationType::OccupiedSeat; // can't mod on curr_seat because rust
                        println!("  Unoccupied seat at {}, {} changing to occupied", i, j);
                        // ferry[i][j].loc_type = LocationType::OccupiedSeat;
                        next_ferry[i][j].loc_type = LocationType::OccupiedSeat;
                        seats_are_changing = true;
                    }

                    // for neighbor in checkable_neighbors {
                    //     if neighbor.loc_type == LocationType::OccupiedSeat {
                    //
                    //     }
                    // }

                    //
                    // RULE 2
                    //
                } else if curr_seat.loc_type == LocationType::OccupiedSeat {

                    // check for occupied adjacents
                    // let mut checkable_neighbors: Vec<&FerrySeat> = Vec::new();

                    let mut occupied_adjacent_seats = 0;

                    if curr_seat.up_loc.y >= 0 {
                        let neighbor: &FerrySeat = &ferry[curr_seat.up_loc.y as usize][curr_seat.up_loc.x as usize];
                        // checkable_neighbors.push(neighbor);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            occupied_adjacent_seats = occupied_adjacent_seats + 1;
                        }
                    }

                    if curr_seat.ne.y >= 0 && curr_seat.ne.x < cols {
                        let neighbor: &FerrySeat = &ferry[curr_seat.ne.y as usize][curr_seat.ne.x as usize];
                        // checkable_neighbors.push(neighbor);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            occupied_adjacent_seats = occupied_adjacent_seats + 1;
                        }
                    }

                    if curr_seat.right_loc.x < cols {
                        let neighbor: &FerrySeat = &ferry[curr_seat.right_loc.y as usize][curr_seat.right_loc.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.right_loc.y as usize][curr_seat.right_loc.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            occupied_adjacent_seats = occupied_adjacent_seats + 1;
                        }
                    }

                    if curr_seat.se.y < rows && curr_seat.se.x < cols {
                        let neighbor: &FerrySeat = &ferry[curr_seat.se.y as usize][curr_seat.se.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.se.y as usize][curr_seat.se.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            occupied_adjacent_seats = occupied_adjacent_seats + 1;
                        }
                    }

                    if curr_seat.down_loc.y < rows {
                        let neighbor: &FerrySeat = &ferry[curr_seat.down_loc.y as usize][curr_seat.down_loc.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.down_loc.y as usize][curr_seat.down_loc.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            occupied_adjacent_seats = occupied_adjacent_seats + 1;
                        }
                    }

                    if curr_seat.sw.y < rows && curr_seat.sw.x >= 0 {
                        let neighbor: &FerrySeat = &ferry[curr_seat.sw.y as usize][curr_seat.sw.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.sw.y as usize][curr_seat.sw.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            occupied_adjacent_seats = occupied_adjacent_seats + 1;
                        }
                    }

                    if curr_seat.left_loc.x >= 0 {
                        let neighbor: &FerrySeat = &ferry[curr_seat.left_loc.y as usize][curr_seat.left_loc.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.left_loc.y as usize][curr_seat.left_loc.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            occupied_adjacent_seats = occupied_adjacent_seats + 1;
                        }
                    }

                    if curr_seat.nw.y >= 0 && curr_seat.nw.x >= 0 {
                        let neighbor: &FerrySeat = &ferry[curr_seat.nw.y as usize][curr_seat.nw.x as usize];
                        // checkable_neighbors.push(&ferry[curr_seat.nw.y as usize][curr_seat.nw.x as usize]);

                        if neighbor.loc_type == LocationType::OccupiedSeat {
                            occupied_adjacent_seats = occupied_adjacent_seats + 1;
                        }
                    }

                    if occupied_adjacent_seats >= 4 {
                        // curr_seat.loc_type = LocationType::OccupiedSeat; // can't mod on curr_seat because rust
                        println!("  Occupied seat at {}, {} changing to unoccupied", i, j);
                        // ferry[i][j].loc_type = LocationType::EmptySeat;
                        next_ferry[i][j].loc_type = LocationType::EmptySeat;
                        seats_are_changing = true;
                    }
                }
            }
        }

        ferry = next_ferry.to_vec();

    }

    println!("seats have finished changing");

    let mut occupied_seats = 0;
    for i in 0..ferry.len() {
        for j in 0..ferry[i].len() {
            if ferry[i][j].loc_type == LocationType::OccupiedSeat {
                occupied_seats = occupied_seats + 1;
            }
        }
    }

    println!("occupied seats: {}", occupied_seats); //2338
}


fn fancy_print (ferry: &mut Vec<Vec<FerrySeat>>) {
    for i in 0..ferry.len() {
        let mut row: Vec<String> = Vec::new();

        for j in 0..ferry[i].len() {
            if ferry[i][j].loc_type == LocationType::OccupiedSeat {
                row.push("#".to_string());
            } else if ferry[i][j].loc_type == LocationType::EmptySeat {
                row.push("L".to_string());
            } else {
                row.push(".".to_string());
            }
        }

        println!("{}",row.join(""));
    }
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
