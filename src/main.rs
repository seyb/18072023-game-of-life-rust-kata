use std::cmp::PartialEq;

fn main() {
    println!("Hello, world!");
    println!("Hello, world!");
}

#[derive(Debug, Clone, PartialEq)]
pub enum LivingCell {
    DEAD,
    ALIVE,
}

#[derive(Debug, PartialEq)]
pub struct GameOfLife {
    pub board: Vec<Vec<LivingCell>>,
}

impl PartialEq<Vec<Vec<LivingCell>>> for GameOfLife {
    fn eq(&self, other: &Vec<Vec<LivingCell>>) -> bool {
        self.board.len() == other.len()
    }
}


impl GameOfLife {
    fn new(number_of_cells: usize, number_of_lines: usize) -> Self {
        let mut line = vec![LivingCell::DEAD; number_of_cells];
        let mut board = vec![line.clone(); number_of_lines];
        Self { board }
    }
}

#[cfg(test)]
mod test {
    use crate::{GameOfLife, LivingCell};

    /*
    1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    2. Any live cell with more than three live neighbours dies, as if by overcrowding.
    3. Any live cell with two or three live neighbours lives on to the next generation.
    4. Any dead cell with exactly three live neighbours becomes a live cell.
    */

    #[test]
    fn it_inits_the_board_with_dead_cells() {
        let line = vec![LivingCell::DEAD, LivingCell::DEAD, LivingCell::DEAD, LivingCell::DEAD];
        let vec1 = vec![line.clone(), line.clone(), line.clone(), line.clone()];
        assert_eq!(GameOfLife::new(4, 4), GameOfLife { board: vec1 });
    }
}

