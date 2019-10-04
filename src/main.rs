#[macro_use]
extern crate strum_macros;

extern crate pathfinding;

use std::time::Instant;

use rand::Rng;
use pathfinding::prelude::astar;

mod puzzle15;
use puzzle15::Node;
use puzzle15::Direction;

fn main() {
    let n = 10;
    let scramble_depth = 100;
    let mut rng = rand::thread_rng();
    let mut start_nodes: Vec<Node> = Vec::new();
    for _i in 0..n{
        let mut node = Node::from_tiles(Node::SOLVED_TILES);
        for _j in 0..scramble_depth{
            let rand_num: u8 = rng.gen_range(1,5);
            let direction = match rand_num {
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Right,
                4 => Direction::Up,
                _ => panic!("rng not working?!"),
            };
            node.slide(direction);
        }
        start_nodes.push(node);
    }
    for i in 1..=2{
        let start = Instant::now();
        solve(&start_nodes, i);
        let duration = start.elapsed();
        println!("heuristic {}: {:?}", i, duration);
    }
}
fn solve(start_nodes: &Vec<Node>, heuristic: u8){
    for node in start_nodes{
        let _solution = astar(node, |n| n.successors(), |n| match heuristic{
            1 => n.heuristic1(),
            2 => n.heuristic2(),
            _ => 0,
        }, |n| n.is_solved());
        let _path = Node::reconstruct_path(_solution.unwrap().0);
    }
}
