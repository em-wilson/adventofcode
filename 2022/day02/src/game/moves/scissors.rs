use crate::game::{BattleResult, Move};
use crate::game::moves::{GameMove, Scissors};

impl GameMove for Scissors {
    fn beats(&self) -> Move { Move::Paper }
    fn as_move(&self) -> Move { Move::Scissors }
    fn loses_to(&self) -> Move { Move::Rock }
    fn versus(&self, opponent: Move) -> BattleResult {
        match opponent {
            Move::Rock => BattleResult::Loss,
            Move::Paper => BattleResult::Win,
            Move::Scissors => BattleResult::Draw
        }
    }
    fn value(&self) -> u32 { 3 }
}
