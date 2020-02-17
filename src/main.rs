use std::thread;
use std::sync::{Arc, Mutex};

const LSIZE: usize = 10;
const WALL: char = '#';
const HALL: char = 'O';
const BEG: char = '>';
const END: char = 'X';

fn main() {
    let mut lab = vec![['#';LSIZE];LSIZE];


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
    lab [9][9]= END;



    for (x, row) in lab.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
            print!("{}", col);
        }
        println!("");
    }



}
