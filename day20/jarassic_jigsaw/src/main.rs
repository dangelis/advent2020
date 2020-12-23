use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct Tile {
    tile: Vec<String>,
    id: usize,
}

impl Tile {
    pub fn print(&self) {
        println!("id: {}", self.id.to_string());

        for i in &self.tile {
            println!("{}", i);
        };
    }

    pub fn top(&self) -> Vec<char> {
        self.tile.get(0).unwrap().chars().collect()
    }

    pub fn bottom(&self) -> Vec<char> {
        let side_len = self.tile.len();
        self.tile.get(side_len - 1).unwrap().chars().collect()
    }

    pub fn right(&self) -> Vec<char> {
        let mut right: Vec<char> = Vec::new();
        let side_len = self.tile.len();

        for r in 0..side_len {
            right.push(self.tile.get(r).unwrap().chars().nth(side_len - 1).unwrap());
        }

        right
   }

    pub fn left(&self) -> Vec<char> {
        let mut right: Vec<char> = Vec::new();
        let side_len = self.tile.len();

        for r in 0..side_len {
            right.push(self.tile.get(r).unwrap().chars().nth(0).unwrap());
        }

        right
    }
}


fn main() {


    // vec test
    // let mut v1: Vec<usize> = vec![0, 1, 2];
    // let mut v2: Vec<usize> = vec![2, 1, 0];
    //
    // v2.reverse();
    //
    // if v1 == v2 {
    //     println!("vectors match!");
    // } else {
    //     println!("vectors DON'T match!");
    // }




    let mut records = file_lines_as_vec("./input");

    let mut tiles: Vec<Tile> = Vec::new();

    // create tiles
    let mut tile = Tile {
        tile: Vec::new(),
        id: 0,
    };

    for line in records {

        let mut split_line = line.split(' ');

        match split_line.next().unwrap() {
            "Tile" => {
                let id = split_line.next().unwrap().replace(':',"");
                tile.id = id.parse::<usize>().unwrap();
                // println!("Found tile {}", tile.id);
            },
            "" => {
                // println!("Found sep");
                tiles.push(tile);
                tile = Tile {
                    tile: Vec::new(),
                    id: 0,
                };
            }
            _ => {
                // println!("Found image row: {}", line);
                tile.tile.push(line);
            }
        }
    }

    tiles.push(tile);

    // for t in &tiles {
    //     t.print();
    // }

    // compare tiles and build image
    // I think we can cheat a bit here. I really only want the corners. Only the corners will
    // have 2 unmatched sides

    let mut corner_tiles: Vec<&Tile> = Vec::new();

    // for t in &tiles {
    let side_len = tiles.len();
    for t in 0..side_len {

        let mut unmatched_sides = 4;

        //println!("checking tile at {}", t);
        // tiles.get(t).unwrap().print();

        // let top: Vec<char> = tiles.get(t).unwrap().top();
        // let side_len = top.len();
        // let bottom: Vec<char> = tiles.get(t).unwrap().bottom();
        // let mut right: Vec<char> = tiles.get(t).unwrap().right();
        // let mut left: Vec<char> = tiles.get(t).unwrap().left();

        // println!("tile left:");
        // for i in &left {
        //     println!("  {}", i);
        // }

        for t2 in 0..side_len {

            println!("Comparing: {}:{} to {}:{}", t, tiles.get(t).unwrap().id, t2, tiles.get(t2).unwrap().id);

            if t == t2 {
                continue;
            }

            // top
            if tiles.get(t).unwrap().top() == tiles.get(t2).unwrap().top() {
                unmatched_sides = unmatched_sides - 1;
                // println!("Matched {} top to {} top", t, t2);
                println!("Found match tt");
            } else {
                // tiles.get(t2).unwrap().top().reverse();
                // if tiles.get(t).unwrap().top() == tiles.get(t2).unwrap().top() {
                //     unmatched_sides = unmatched_sides - 1;
                //     // println!("Matched {} top to {} top", t, t2);
                //     println!("Found match ttr");
                // }
               unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().top()),
                                                  &mut (tiles.get(t2).unwrap().top()),
                                                  unmatched_sides);
            }
            // unmatched_sides = compare_top_bott(tiles.get(t).unwrap(), tiles.get(t2).unwrap(), unmatched_sides);

            if tiles.get(t).unwrap().top() == tiles.get(t2).unwrap().right() {
                unmatched_sides = unmatched_sides - 1;
                //println!("Matched {} top to {} top", t, t2);
                println!("Found match tr");
            } else {
                // tiles.get(t2).unwrap().right().reverse();
                // if tiles.get(t).unwrap().top() == tiles.get(t2).unwrap().right() {
                //     unmatched_sides = unmatched_sides - 1;
                //     //println!("Matched {} top to {} top", t, t2);
                //     println!("Found match tr");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().top()),
                                                   &mut (tiles.get(t2).unwrap().right()),
                                                   unmatched_sides);
            }


            println!("  comparing {} top to {} bottom", t, t2);
            tiles.get(t).unwrap().print();
            tiles.get(t2).unwrap().print();
            if tiles.get(t).unwrap().top() == tiles.get(t2).unwrap().bottom() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match tb");
            } else {

                // let mut rev =  tiles.get(t2).unwrap().bottom();
                // rev.reverse();
                // //tiles.get(t2).unwrap().bottom().reverse();
                // // println!("rev:");
                // // for i in &rev {
                // //     println!("{}", i);
                // // };
                //
                // println!("  Now checking reversed");
                // //tiles.get(t2).unwrap().print();
                //
                // //if tiles.get(t).unwrap().top() == tiles.get(t2).unwrap().bottom() {
                // if tiles.get(t).unwrap().top() == rev {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match tbr");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().top()),
                                                   &mut (tiles.get(t2).unwrap().bottom()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().top() == tiles.get(t2).unwrap().left() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match tl");
            } else {
                // tiles.get(t2).unwrap().left().reverse();
                // if tiles.get(t).unwrap().top() == tiles.get(t2).unwrap().left() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match tl");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().top()),
                                                   &mut (tiles.get(t2).unwrap().left()),
                                                   unmatched_sides);
            }

            // right
            if tiles.get(t).unwrap().right() == tiles.get(t2).unwrap().top() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match rt");
            } else {
                // tiles.get(t2).unwrap().top().reverse();
                // if tiles.get(t).unwrap().right() == tiles.get(t2).unwrap().top() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match rt");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().right()),
                                                   &mut (tiles.get(t2).unwrap().top()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().right() == tiles.get(t2).unwrap().right() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match rr");
            } else {
                // tiles.get(t2).unwrap().right().reverse();
                // if tiles.get(t).unwrap().right() == tiles.get(t2).unwrap().right() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match rr");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().right()),
                                                   &mut (tiles.get(t2).unwrap().right()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().right() == tiles.get(t2).unwrap().bottom() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match rb");
            } else {
                // tiles.get(t2).unwrap().bottom().reverse();
                // if tiles.get(t).unwrap().right() == tiles.get(t2).unwrap().bottom() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match rb");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().right()),
                                                   &mut (tiles.get(t2).unwrap().bottom()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().right() == tiles.get(t2).unwrap().left() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match rl");
                // println!("Matched {} right to {} left", t, t2);
                // tiles.get(t).unwrap().print();
                // tiles.get(t2).unwrap().print();
            } else {
                // tiles.get(t2).unwrap().left().reverse();
                // if tiles.get(t).unwrap().right() == tiles.get(t2).unwrap().left() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match rl");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().right()),
                                                   &mut (tiles.get(t2).unwrap().left()),
                                                   unmatched_sides);
            }

            // bottom
            if tiles.get(t).unwrap().bottom() == tiles.get(t2).unwrap().top() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match bt");
            } else {
                // tiles.get(t2).unwrap().top().reverse();
                // if tiles.get(t).unwrap().bottom() == tiles.get(t2).unwrap().top() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match bt");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().bottom()),
                                                   &mut (tiles.get(t2).unwrap().top()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().bottom() == tiles.get(t2).unwrap().right() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match br");
            } else {
                // tiles.get(t2).unwrap().right().reverse();
                // if tiles.get(t).unwrap().bottom() == tiles.get(t2).unwrap().right() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match br");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().bottom()),
                                                   &mut (tiles.get(t2).unwrap().right()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().bottom() == tiles.get(t2).unwrap().bottom() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match bb");
            } else {
                // tiles.get(t2).unwrap().bottom().reverse();
                // if tiles.get(t).unwrap().bottom() == tiles.get(t2).unwrap().bottom() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match bb");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().bottom()),
                                                   &mut (tiles.get(t2).unwrap().bottom()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().bottom() == tiles.get(t2).unwrap().left() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match bl");
            } else {
                // tiles.get(t2).unwrap().left().reverse();
                // if tiles.get(t).unwrap().bottom() == tiles.get(t2).unwrap().left() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match bl");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().bottom()),
                                                   &mut (tiles.get(t2).unwrap().left()),
                                                   unmatched_sides);
            }


            // left
            if tiles.get(t).unwrap().left() == tiles.get(t2).unwrap().top() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match lt");
            } else {
                // tiles.get(t2).unwrap().top().reverse();
                // if tiles.get(t).unwrap().left() == tiles.get(t2).unwrap().top() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match lt");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().left()),
                                                   &mut (tiles.get(t2).unwrap().top()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().left() == tiles.get(t2).unwrap().right() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match lr");
            } else {
                // tiles.get(t2).unwrap().right().reverse();
                // if tiles.get(t).unwrap().left() == tiles.get(t2).unwrap().right() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match lr");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().left()),
                                                   &mut (tiles.get(t2).unwrap().right()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().left() == tiles.get(t2).unwrap().bottom() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match lb");
            } else {
                // tiles.get(t2).unwrap().bottom().reverse();
                // if tiles.get(t).unwrap().left() == tiles.get(t2).unwrap().bottom() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match lb");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().left()),
                                                   &mut (tiles.get(t2).unwrap().bottom()),
                                                   unmatched_sides);
            }

            if tiles.get(t).unwrap().left() == tiles.get(t2).unwrap().left() {
                unmatched_sides = unmatched_sides - 1;
                println!("Found match ll");
            } else {
                // tiles.get(t2).unwrap().left().reverse();
                // if tiles.get(t).unwrap().left() == tiles.get(t2).unwrap().left() {
                //     unmatched_sides = unmatched_sides - 1;
                //     println!("Found match ll");
                // }
                unmatched_sides = compare_reversed(&mut (tiles.get(t).unwrap().left()),
                                                   &mut (tiles.get(t2).unwrap().left()),
                                                   unmatched_sides);
            }


        }
        println!("Unmatched sides at {}: {}", t, unmatched_sides);
        if unmatched_sides >= 2 {
            println!("Found corner: {}", tiles.get(t).unwrap().id);
            corner_tiles.push(tiles.get(t).unwrap());
        }
    }

    let mut prod: f64 = 1f64;
    for t in corner_tiles {
        println!("corner tile: {}", t.id);
        prod = prod * t.id as f64;
    }

    println!("sol: {}", prod);

}

fn compare_top_bott (tile1: &Tile, tile2: &Tile, unmatched_sides: usize) -> usize {
    // println!("  comparing {} top to {} bottom", t, t2);
    // tiles.get(t).unwrap().print();
    // tiles.get(t2).unwrap().print();
    let mut unmatched = unmatched_sides;


    if tile1.top() == tile2.bottom() {
        unmatched = unmatched - 1;
        // println!("Found match tb");
    } else {
        unmatched = compare_reversed(&mut tile1.top(), &mut tile2.bottom(), unmatched);
    }

    unmatched
}

fn compare_reversed (line1: &mut Vec<char>, line2: &mut Vec<char>, mut unmatched: usize) -> usize {

    let mut rev =  line2.as_mut_slice();
    rev.reverse();

    println!("  Now checking reversed");

    if line1.as_mut_slice() == rev {
        unmatched = unmatched - 1;
    }

    unmatched
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