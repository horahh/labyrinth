use std::thread;
use std::sync::{Arc, Mutex};

const LSIZE: usize = 10;
const WALL: char = '#';
const HALL: char = 'O';
const BEG: char = '>';
const END: char = 'X';

fn main() {
    let mut lab = vec![vec!['#';LSIZE];LSIZE];


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
    let mut lab_out_count = vec![[0u64; LSIZE];LSIZE];
    // store the min cost function in the map from each cell
    let mut lab_out_min_count = vec![[0u64; LSIZE];LSIZE];


    /* PRINT the LAB */
    for (x, row) in lab.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
            print!("{}", col);
        }
        println!("");
    }
    let mut solution : Vec<(usize,usize)> = Vec::<(usize,usize)>::new();
    let res=match advance(5,2,&lab, &mut solution) {
        Ok(bool_result) => bool_result,
        Err(err_str)  => return , // TODO: how to print here... println!("Error: Advance {}", err_str) 
    };
    for step in &solution {
        println!("{:?}\n", step);
    }
}

fn next(x: usize, y: usize) -> Result<Vec<(usize,usize)>, &'static str> {
    let mut next_xy : Vec<(usize,usize)> = Vec::new();
    next_xy.push((x+1,y));
    next_xy.push((x,y+1));
    //next_xy.push((x-1,y));
    //next_xy.push((x,y-1));
    return Ok(next_xy);
}

/// Function to advance a step in the labyrinth 
/// takes x and y positions, a ref to the lab and returns a bool 
/// telling whether or not that branch found an exit
fn advance(x: usize, y: usize, lab: &Vec<Vec<char>>, mut solution: &Vec<(usize,usize)> ) -> Result<bool, &'static str> {

    let mut possible_paths = next(x,y)?;
    let mut ret: Result<bool,&'static str> = Ok(false);

    for (new_x,new_y) in possible_paths {
        match lab[new_x][new_y] {
            '#' => return Ok(false),
            '>' => return Ok(false),
            'X' => ret = Ok(true),
            'O' => ret = advance(new_x, new_y, &lab, &mut solution),
            _   => return Err("Invalid character found"),
        }
    }
    // Do somethign with ret, store path to a solution
    return ret;
}
