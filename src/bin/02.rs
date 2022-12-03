use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn get(index: u32) -> Shape {
        match index {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => panic!("Invalid Index {}", index),
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

struct Round {
    opponent: Shape,
    player: Shape,
}

impl Round {
    pub fn new(opponent: Shape, player: Shape) -> Round {
        Round { opponent, player }
    }
}

fn game_result(round: &Round) -> GameResult {
    let Round {
        mut opponent,
        mut player,
    } = round;

    // hacky way to deal with the wrap around
    if (player == Shape::Rock && opponent == Shape::Scissors)
        || (player == Shape::Scissors && opponent == Shape::Rock)
    {
        (player, opponent) = (opponent, player);
    }

    return match player.cmp(&opponent) {
        Ordering::Equal => GameResult::Draw,
        Ordering::Greater => GameResult::Win,
        Ordering::Less => GameResult::Loss,
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let rounds = input.split("\n").map(|round| {
        let mut turns = round.split_whitespace().map(|turn| match turn {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid Shape"),
        });

        Round::new(turns.next().unwrap(), turns.next().unwrap())
    });

    let score = rounds
        .map(|round| round.player as u32 + game_result(&round) as u32 + 1)
        .sum();

    Some(score)
}

struct Guide {
    opponent: Shape,
    result: GameResult,
}

impl Guide {
    pub fn new(opponent: Shape, result: GameResult) -> Guide {
        Guide { opponent, result }
    }
}

fn find_shape_to_play(guide: &Guide) -> Shape {
    let Guide { opponent, result } = guide;

    return match result {
        GameResult::Draw => *opponent,
        GameResult::Win => Shape::get((*opponent as u32 + 1) % 3),
        GameResult::Loss => Shape::get(((*opponent as i32 - 1 + 3) % 3).try_into().unwrap()),
    };
}

pub fn part_two(input: &str) -> Option<u32> {
    let guides = input.split("\n").map(|round| {
        let mut columns = round.split_whitespace();

        Guide::new(
            match columns.next().unwrap() {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => panic!("Invalid Shape"),
            },
            match columns.next().unwrap() {
                "X" => GameResult::Loss,
                "Y" => GameResult::Draw,
                "Z" => GameResult::Win,
                _ => panic!("Invalid Result"),
            },
        )
    });

    let score = guides
        .map(|guide| find_shape_to_play(&guide) as u32 + guide.result as u32 + 1)
        .sum();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn game_is_draw() {
        let rock_round = Round::new(Shape::Rock, Shape::Rock);
        let paper_round = Round::new(Shape::Paper, Shape::Paper);
        let scissor_round = Round::new(Shape::Scissors, Shape::Scissors);

        assert_eq!(game_result(&rock_round), GameResult::Draw);
        assert_eq!(game_result(&paper_round), GameResult::Draw);
        assert_eq!(game_result(&scissor_round), GameResult::Draw);
    }
    #[test]
    fn game_is_win() {
        let rock_round = Round::new(Shape::Scissors, Shape::Rock);
        let paper_round = Round::new(Shape::Rock, Shape::Paper);
        let scissor_round = Round::new(Shape::Paper, Shape::Scissors);

        assert_eq!(game_result(&rock_round), GameResult::Win); // needs to wrap around
        assert_eq!(game_result(&paper_round), GameResult::Win);
        assert_eq!(game_result(&scissor_round), GameResult::Win);
    }
    #[test]
    fn game_is_loss() {
        let rock_round = Round::new(Shape::Paper, Shape::Rock);
        let paper_round = Round::new(Shape::Scissors, Shape::Paper);
        let scissor_round = Round::new(Shape::Rock, Shape::Scissors);

        assert_eq!(game_result(&rock_round), GameResult::Loss);
        assert_eq!(game_result(&paper_round), GameResult::Loss);
        assert_eq!(game_result(&scissor_round), GameResult::Loss); // needs to wrap around
    }

    #[test]
    fn must_play_rock() {
        let guide = Guide::new(Shape::Rock, GameResult::Draw);
        assert_eq!(find_shape_to_play(&guide), Shape::Rock);

        let guide = Guide::new(Shape::Scissors, GameResult::Win);
        assert_eq!(find_shape_to_play(&guide), Shape::Rock);

        let guide = Guide::new(Shape::Paper, GameResult::Loss);
        assert_eq!(find_shape_to_play(&guide), Shape::Rock);
    }
    #[test]
    fn must_play_paper() {
        let guide = Guide::new(Shape::Paper, GameResult::Draw);
        assert_eq!(find_shape_to_play(&guide), Shape::Paper);

        let guide = Guide::new(Shape::Scissors, GameResult::Loss);
        assert_eq!(find_shape_to_play(&guide), Shape::Paper);

        let guide = Guide::new(Shape::Rock, GameResult::Win);
        assert_eq!(find_shape_to_play(&guide), Shape::Paper);
    }

    #[test]
    fn must_play_scissors() {
        let guide = Guide::new(Shape::Scissors, GameResult::Draw);
        assert_eq!(find_shape_to_play(&guide), Shape::Scissors);

        let guide = Guide::new(Shape::Rock, GameResult::Loss);
        assert_eq!(find_shape_to_play(&guide), Shape::Scissors);

        let guide = Guide::new(Shape::Paper, GameResult::Win);
        assert_eq!(find_shape_to_play(&guide), Shape::Scissors);
    }
}
