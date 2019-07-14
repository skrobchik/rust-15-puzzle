#[macro_use]
extern crate strum_macros;

extern crate pathfinding;

use std::io;
use pathfinding::prelude::astar;

mod puzzle15;
use puzzle15::Node;

fn main() {
    let mut tiles: [[u8; 4]; 4] = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4{
            println!("{},{}: ", i, j);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let val = input.trim().parse::<u8>().unwrap();
            tiles[i][j] = val;
        }
    }
    let node = Node::from_tiles(tiles);

    let solution = astar(&node, |n| n.successors(), |n| n.heuristic(), |n| n.is_solved());

    let path = Node::reconstruct_path(solution.unwrap().0);

    println!("{:?}", path);
}
