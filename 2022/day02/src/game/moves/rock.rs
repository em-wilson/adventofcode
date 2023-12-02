use crate::game::{BattleResult, Move};
use crate::game::moves::{GameMove, Rock};

impl GameMove for Rock {
    fn beats(&self) -> Move { Move::Scissors }
    fn as_move(&self) -> Move { Move::Rock }
    fn loses_to(&self) -> Move { Move::Paper }
    fn versus(&self, opponent: Move) -> BattleResult {
        match opponent {
            Move::Rock => BattleResult::Draw,
            Move::Paper => BattleResult::Loss,
            Move::Scissors => BattleResult::Win,
        }
    }
    fn value(&self) -> u32 { 1 }
}
