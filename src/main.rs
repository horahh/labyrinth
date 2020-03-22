#![deny(missing_docs)]

//! A simple labyrinth solver
//! To dive into the intricacies of Rust

//use std::thread;
//use std::sync::{Arc, Mutex};

const LSIZE: usize = 10; /// Lenght of the Labyrinth
const WALL: char = '#';  /// Identification of the WALLS
const HALL: char = 'O';  /// Identification of the HALLS
const BEG: char = '>';   /// BEGINNING of the labyrinth 
const END: char = 'X';   /// END of the labyrith
const WIDTH: usize=2;    /// Space for printing labyrinth characters

/// Main function to just perform a simple call to resolve the labyrinth
fn main() {
    let _res = match solve_lab() {
        Ok(value) => value,
        Err(_e) => return,
    };
}
/// Function to handle the results of the methods
fn solve_lab() -> Result<bool,&'static str> {
    /* find the exit with recursion */

    // count the number of exits from each of the cells
    //let mut lab_out_count = vec![[0u64; LSIZE];LSIZE];
    // store the min cost function in the map from each cell
    //let mut lab_out_min_count = vec![[0u64; LSIZE];LSIZE];
    let mut lab = Labyrinth { 
        lab : vec![vec![WALL;LSIZE];LSIZE], 
        solution : Vec::<(usize,usize)>::new(),
        solved : false ,
        created: false,
        start: (5,2),
        end: (9,9),
        };
    let _ = lab.create()?;
    let _ = lab.print()?;
    let _ = lab.solve()?;
    let _ = lab.print_solution()?;
    return Ok(true);
}

struct Labyrinth {
    lab : Vec<Vec<char>>,
    solution : Vec<(usize,usize)>,
    solved: bool,
    created: bool,
    start: (usize,usize),
    end: (usize, usize),
}

/// Trait that has all the methods to create a labyrinth and solve it
impl Labyrinth {
    /// Method that implements the labyrinth creation
    fn create(&mut self ) -> Result<bool, &'static str> {

        //let mut lab = vec![vec![WALL;LSIZE];LSIZE];

        /* Set some HALL area to walk to labyrinth */
        for (x, row) in self.lab.iter_mut().enumerate() {
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
        let (x_start,y_start) = self.start;
        self.lab [x_start][y_start] = BEG;
        let (x_end,y_end) = self.end;
        self.lab [x_end][y_end] = END;
        return Ok(true);
    }
    fn print(&self) -> Result<bool,&'static str> {
        if self.created == true {
            return Err("Labyrinth not yet created");
        }
        print!("       ");
        for y in 0..self.lab.len() {
            print!("{val:>w$}",val=y, w=WIDTH);
        }
        println!();
        for (x, row) in self.lab.iter().enumerate() {
            print!("x = {val:>w$} ", val=x, w=WIDTH);
            for (y,col) in row.iter().enumerate() {
                print!("{val:>w$}",val=col, w=WIDTH);
            }
            println!();
        }
        return Ok(true);
    }
    /// Method to find the solution
    fn solve(&mut self) -> Result<bool,&'static str> {
        return self.advance(5,2);
    }
    /// Method to print the solution
    fn print_solution(&self) -> Result<bool, &'static str> {
        if self.solved == true {
            return Err("Labyrinth not solved");
        }

        println!("Solution in tuples:");
        for step in &self.solution {
            print!("{:?}", step);
        }
        println!();
        return Ok(true);
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
    fn _next(&mut self, x: usize, y: usize) -> Result<Vec<(usize,usize)>, &'static str> {
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
    fn advance(&mut self, x: usize, y: usize ) -> Result<bool, &'static str> {

        let possible_paths = self._next(x,y)?;
        let mut ret: Result<bool,&'static str> = Ok(false);

        println!("trying pos: {:?}", (x,y));

        for (new_x,new_y) in possible_paths {
            match self.lab[new_x][new_y] {
                '#' => ret = Ok(false),
                    '>' => ret = Ok(false),
                    'X' => ret = Ok(true),
                    'O' => ret = self.advance(new_x, new_y),
                    _   => return Err("Invalid character found"),
            }
            match ret {
                Ok(true) => break,
                _ => (),
            }
        }
        // Do somethign with ret, store path to a solution
        match ret {
            Ok(true) => self.solution.push((x,y)),
            Ok(false) => println!("wrong path: {:?}", (x,y)),
            _ => return ret,
        }
        return ret;
    }
}
