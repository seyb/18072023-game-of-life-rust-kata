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
    fn new(x: i32, y: i32) -> Self {
        let mut line = vec![];
        for _i in 0..x {
            line.push(LivingCell::DEAD)
        }
        let mut board = vec![];
        for _i in 0..y {
            board.push(line.clone())
        }
        Self{ board }
    }
}

#[cfg(test)]
mod test {
    use crate::{GameOfLife, LivingCell};

    #[test]
    fn it_inits_the_board_with_dead_cells() {
        let line = vec![LivingCell::DEAD, LivingCell::DEAD, LivingCell::DEAD, LivingCell::DEAD];
        let vec1 = vec![line.clone(), line.clone(), line.clone(), line.clone()];
        assert_eq!(GameOfLife::new(4, 4), GameOfLife { board: vec1 });
    }
}

