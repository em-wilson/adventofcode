use crate::game::{BattleResult, Move};
use crate::game::moves::{GameMove, Paper};

impl GameMove for Paper {
    fn beats(&self) -> Move { Move::Rock }
    fn as_move(&self) -> Move { Move::Paper }
    fn loses_to(&self) -> Move { Move::Scissors }
    fn versus(&self, opponent: Move) -> BattleResult {
        match opponent {
            Move::Rock => BattleResult::Win,
            Move::Paper => BattleResult::Draw,
            Move::Scissors => BattleResult::Loss,
        }
    }
    fn value(&self) -> u32 { 2 }
}
