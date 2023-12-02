use crate::game::Move;
use crate::game::moves::GameMove;

pub trait PlayBuilder {
    fn from_opponent_string(&self, opponent_play: &Box<dyn GameMove>, value: String) -> Move;
}

pub struct OutcomeMapper;

impl PlayBuilder for OutcomeMapper {
    fn from_opponent_string(&self, opponent_play: &Box<dyn GameMove>, value: String) -> Move {
        return match value.as_str() {
            "X" => opponent_play.beats(),
            "Y" => opponent_play.as_move(),
            "Z" => opponent_play.loses_to(),
            _ => panic!("Invlaid Move label: {}", value)
        }
    }
}

pub struct PlayMapper;

impl PlayBuilder for PlayMapper {
    fn from_opponent_string(&self, _opponent_play: &Box<dyn GameMove>, value: String) -> Move {
        return match value.as_str() {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Invalid Move label: {}", value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Move;
    use crate::game::moves::GameMove;
    use super::{OutcomeMapper, PlayBuilder};

    #[test]
    fn test_outcome_mapper() {
        let mapper = OutcomeMapper{};
        let opponent_play = <dyn GameMove>::from_move(Move::Rock);
        assert_eq!(Move::Scissors, mapper.from_opponent_string(&opponent_play, "X".to_string()));
        assert_eq!(Move::Rock, mapper.from_opponent_string(&opponent_play, "Y".to_string()));
        assert_eq!(Move::Paper, mapper.from_opponent_string(&opponent_play, "Z".to_string()));
    }
}