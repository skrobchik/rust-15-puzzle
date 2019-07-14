extern crate strum;

use strum::IntoEnumIterator;

#[derive(EnumIter, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}
impl Direction{
    fn reverse(direction: Direction) -> Direction{
        match direction{
            Direction::Up    => Direction::Down,
            Direction::Down  => Direction::Up,
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct MatrixIndex{
    i: usize,
    j: usize,
}
impl MatrixIndex{
    fn from(i: usize, j: usize) -> MatrixIndex{
        MatrixIndex{
            i: i,
            j: j,
        }
    }
    fn translate(&mut self, delta_i: isize, delta_j: isize){
        if !(self.i as isize + delta_i < 0){
            self.i = (self.i as isize + delta_i) as usize;
        } else {
            self.i = 0;
        }
        if !(self.j as isize + delta_j < 0){
            self.j = (self.j as isize + delta_j) as usize;
        } else {
            self.j = 0;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Node{
    tiles: [[u8; 4]; 4],
    pub parent_direction: Option<Direction>,
}
impl Node{
    const SOLVED_TILES: [[u8; 4]; 4] = [[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,0]];
    pub fn from_tiles(tiles: [[u8; 4]; 4]) -> Node{
        Node{
            tiles: tiles,
            parent_direction: None,
        }
    }
    pub fn successors(&self) -> Vec<(Node, u32)> {
        let mut successors: Vec<(Node, u32)> = Vec::with_capacity(4);
        for direction in Direction::iter(){
            let mut successor = Node::from_tiles(self.tiles);
            successor.parent_direction = Some(Direction::reverse(direction));
            successor.slide(direction);
            if successor.tiles != self.tiles {
                successors.push((successor, 1));
            }
        }
        successors
    }
    fn get_index(tiles: &[[u8; 4]; 4], value: u8) -> Option<MatrixIndex>{
        for i in 0..4 {
            for j in 0..4{
                if tiles[i][j] == value{
                    return Some(MatrixIndex::from(i, j));
                }
            }
        }
        None
    }
    fn manhatan_distance(index_1: MatrixIndex, index_2: MatrixIndex) -> u32{
        let mut distance: i32 = 0;
        distance += i32::abs(index_1.i as i32 - index_2.i as i32);
        distance += i32::abs(index_1.j as i32 - index_2.j as i32);
        distance as u32
    }
    pub fn heuristic(&self) -> u32{
        let mut value: u32 = 0;
        for i in 0..4{
            for j in 0..4{
                value += Node::manhatan_distance(MatrixIndex::from(i, j), Node::get_index(&Node::SOLVED_TILES, self.tiles[i][j]).unwrap());
            }
        }
        value
    }
    fn slide(&mut self, direction: Direction){
        let zero_index = Node::get_index(&self.tiles, 0).unwrap();
        let mut swap_index = MatrixIndex::from(zero_index.i, zero_index.j); 
        match direction{
            Direction::Down => swap_index.translate(1, 0),
            Direction::Left => swap_index.translate(0, -1),
            Direction::Right => swap_index.translate(0, 1),
            Direction::Up => swap_index.translate(-1, 0),
        };
        if swap_index.i > 3 {swap_index.i = 3;}
        if swap_index.j > 3 {swap_index.j = 3;}
        self.tiles[zero_index.i][zero_index.j] = self.tiles[swap_index.i][swap_index.j];
        self.tiles[swap_index.i][swap_index.j] = 0;
    }
    pub fn reconstruct_path(solution: Vec<Node>) -> Vec<Direction> {
        let mut path: Vec<Direction> = Vec::new();
        for node in solution{
            if node.parent_direction.is_some(){
                path.push(node.parent_direction.unwrap());
            }
        }
        path
    }
    pub fn is_solved(&self) -> bool{
        self.tiles == Node::SOLVED_TILES
    }
}