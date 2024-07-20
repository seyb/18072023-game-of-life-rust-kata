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
        let line = vec![LivingCell::DEAD; number_of_cells];
        let board = vec![line.clone(); number_of_lines];
        Self { board }
    }
    fn set_living_cell(&mut self, line: usize, cell: usize) {
        self.board[line-1][cell-1] = LivingCell::ALIVE
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
        let line = vec![LivingCell::DEAD; 4];
        let board = vec![line.clone(); 4];
        assert_eq!(GameOfLife::new(4, 4), GameOfLife { board });
    }

    #[test]
    fn it_set_living_cell() {
        let mut board = GameOfLife::new(4, 4);
        board.set_living_cell(2, 2);

        let line = vec![LivingCell::DEAD, LivingCell::DEAD, LivingCell::DEAD, LivingCell::DEAD];
        let expected_board = vec![line.clone(), vec![LivingCell::DEAD, LivingCell::ALIVE, LivingCell::DEAD, LivingCell::DEAD], line.clone(), line.clone()];
        assert_eq!(board.board, expected_board)
    }
}

