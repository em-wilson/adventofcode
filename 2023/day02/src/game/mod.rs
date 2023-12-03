pub mod impossible_combinations;
pub mod power_cubes;

pub enum Mode {
    ImpossibleCombinations,
    PowerCubes
}

impl Mode {
    fn calculate(&self, input:String) -> u32 {
        return match self {
            Mode::ImpossibleCombinations => impossible_combinations::calculate_game_score(input.to_string()),
            Mode::PowerCubes => power_cubes::calculate_game_score(input.to_string()),
        }
    }
}

pub struct Game {
    input: String,
}

impl Game {
    pub fn play(&self, game_mode:Mode) -> u32 {
        return sum_games(self.input.to_string(), game_mode);
    }
}

pub fn new_game(input:String) -> Game {
    return Game {
        input: input,
    };
}

fn sum_games(input: String, game_mode:Mode) -> u32 {
    let mut possible_game_sum = 0;
    for game_line in input.split("\n") {
        possible_game_sum += game_mode.calculate(game_line.to_string());
    }
    return possible_game_sum;
}