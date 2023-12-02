use std::fs;

pub mod game;
pub mod play_builder;

use crate::game::Move;
use crate::game::moves::GameMove;
use crate::play_builder::{OutcomeMapper, PlayBuilder, PlayMapper};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

        println!("Total score with direct plays: {0}", game_outcome(input.to_string(), &PlayMapper{}));
    
        println!("Total score with reactive plays: {0}", game_outcome(input.to_string(), &OutcomeMapper{}));
}

fn game_outcome(input: String, play_builder: &impl PlayBuilder) -> u32 {
    return input.split("\n")
        .map(|round| round_outcome(round.to_string(), play_builder))
        .sum();
}

fn round_outcome(input: String, play_builder: &impl PlayBuilder) -> u32 {
    let plays: Vec<_> = input
        .split(" ")
        .collect();

    let opponent_move = Move::from_string(plays[0].to_string());
    let opponent_play = <dyn GameMove>::from_move(opponent_move);
    let my_play = <dyn GameMove>::from_move(play_builder.from_opponent_string(&opponent_play, plays[1].to_string()));

    let outcome = game::play(&opponent_play, &my_play);
    return outcome.score();
}

#[cfg(test)]
mod tests {
    use super::{game_outcome, round_outcome, PlayMapper};

    #[test]
    fn test_game_outcome() {
        assert_eq!(15, game_outcome("A Y\nB X\nC Z".to_string(), &PlayMapper{}));
    }

    #[test]
    fn test_round_outcome() {
        assert_eq!(8, round_outcome("A Y".to_string(), &PlayMapper{}));
        assert_eq!(1, round_outcome("B X".to_string(), &PlayMapper{}));
        assert_eq!(6, round_outcome("C Z".to_string(), &PlayMapper{}));
    }

    #[test]
    fn test_rock_beats_paper() {
        assert_eq!(7, round_outcome("C X".to_string(), &PlayMapper{}));
        assert_eq!(3, round_outcome("A Z".to_string(), &PlayMapper{}));
    }
}