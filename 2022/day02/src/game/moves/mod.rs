pub trait GameMove {
    fn beats(&self) -> Move;
    fn as_move(&self) -> Move;
    fn loses_to(&self) -> Move;
    fn versus(&self, opponent: Move) -> BattleResult;
    fn value(&self) -> u32;
}

mod rock;
mod paper;
mod scissors;

use crate::game::{BattleResult, Move};

impl dyn GameMove {
    pub fn from_move(value: Move) -> Box<dyn GameMove> {
        return match value {
            Move::Rock => Box::new(Rock{}),
            Move::Paper => Box::new(Paper{}),
            Move::Scissors => Box::new(Scissors{})
        }
    }
}

pub struct Rock {}
pub struct Paper {}
pub struct Scissors {}