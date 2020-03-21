//use std::thread;
//use std::sync::{Arc, Mutex};

#![deny(missing_docs)]

//! A simple labyrinth solver
//! To dive into the intricacies of Rust

const LSIZE: usize = 10; /// Lenght of the Labyrinth
const WALL: char = '#';  /// Identification of the WALLS
const HALL: char = 'O';  /// Identification of the HALLS
const BEG: char = '>';   /// BEGINNING of the labyrinth 
const END: char = 'X';   /// END of the labyrith

fn main() {
    let mut lab = vec![vec![WALL;LSIZE];LSIZE];

    /* Set some HALL area to walk to labyrinth */
    for (x, row) in lab.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
            if (x % 2 == 1) && (y < LSIZE-1) && ( y > 0 ) {
                *col = HALL;
            }
            if (y % 2 == 1) && ( x < LSIZE-1) && ( x > 0) && (y < LSIZE -1) {
                *col = HALL;
            }
        }
    }

    /* Set the start and finish */
    lab [5][2] = BEG;
    lab [9][9] = END;

    /* find the exit with recursion */

    // count the number of exits from each of the cells
    //let mut lab_out_count = vec![[0u64; LSIZE];LSIZE];
    // store the min cost function in the map from each cell
    //let mut lab_out_min_count = vec![[0u64; LSIZE];LSIZE];


    /* PRINT the LAB */
    for (_x, row) in lab.iter_mut().enumerate() {
        for (_y, col) in row.iter_mut().enumerate() {
            print!("{}", col);
        }
        println!("");
    }
    let mut solution : Vec<(usize,usize)> = Vec::<(usize,usize)>::new();
    let _res=match advance(5,2,&lab, &mut solution) {
        Ok(bool_result) => bool_result,
        Err(_err_str)  => return , // TODO: how to print here... println!("Error: Advance {}", err_str) 
    };
    for step in &solution {
        println!("{:?}", step);
    }
}


/// Function that takes a x and y position and returns a vector with the x,y tuples that can be advaced from 
/// the current position
///
/// * `x` - The horizontal position
/// * `y` - The vertical position
///
/// # Example:
///
/// ```
/// // Assuming been on position: (2,4)
/// let moves = vec![(3,4),(2,5);
/// asserteq!(moves,_next(2,4));
/// ```
fn _next(x: usize, y: usize) -> Result<Vec<(usize,usize)>, &'static str> {
    let mut next_xy : Vec<(usize,usize)> = Vec::new();
    if x + 1 < LSIZE {
        next_xy.push((x+1,y));
    }
    if y + 1 < LSIZE {
        next_xy.push((x,y+1));
    }
    //next_xy.push((x-1,y));
    //next_xy.push((x,y-1));
    return Ok(next_xy);
}

/// Recursive Function to advance a step in the labyrinth in the valid paths
/// takes x and y positions, a ref to the lab and returns a bool 
/// telling whether or not that branch found an exit
/// 
/// * `x` - The horizontal position
/// * `y` - The vertical position
fn advance(x: usize, y: usize, lab: &Vec<Vec<char>>, mut solution: &mut Vec<(usize,usize)> ) -> Result<bool, &'static str> {

    let possible_paths = _next(x,y)?;
    let mut ret: Result<bool,&'static str> = Ok(false);

    println!("trying pos: {:?}", (x,y));

    for (new_x,new_y) in possible_paths {
        match lab[new_x][new_y] {
            '#' => ret = Ok(false),
            '>' => ret = Ok(false),
            'X' => ret = Ok(true),
            'O' => ret = advance(new_x, new_y, &lab, &mut solution),
            _   => return Err("Invalid character found"),
        }
        match ret {
            Ok(true) => break,
            _ => (),
        }
    }
    // Do somethign with ret, store path to a solution
    match ret {
        Ok(true) => solution.push((x,y)),
        Ok(false) => println!("wrong path: {:?}", (x,y)),
        _ => return ret,
    }
    return ret;
}
