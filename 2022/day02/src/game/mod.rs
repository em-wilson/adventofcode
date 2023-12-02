pub mod moves;

use crate::game::moves::GameMove;

pub enum BattleResult {
    Win = 6,
    Draw = 3,
    Loss = 0
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn from_string(value: String) -> Move {
        return match value.as_str()
         {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid Move label: {}", value)
        }
    }
}

impl GameResult {
    pub fn score(&self) -> u32 {
        return self.base_score + self.bonus_score;
    }
}

pub struct GameResult {
    base_score : u32,
    bonus_score: u32,
}

pub fn play(opponent_move: &Box<dyn GameMove>, my_move: &Box<dyn GameMove>) -> GameResult {
    let outcome = my_move.versus(opponent_move.as_move());
    return GameResult{
        base_score: my_move.value(),
        bonus_score: outcome as u32
    };
}